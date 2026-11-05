import type { ImportDraft } from '@/shared/api';
import { authSummary, canPick, rowProblem, rowsFrom, toChoices } from './importRows';

const ssh = (source: string, overrides: Partial<ImportDraft> = {}): ImportDraft => ({
  source,
  name: source,
  connection: {
    kind: 'ssh',
    host: `${source}.example.com`,
    port: 22,
    user: 'deploy',
    socketPath: '/var/run/docker.sock',
    auth: 'imported',
    publicKey: null,
    bastions: [],
  },
  files: ['/home/me/.ssh/id_ed25519'],
  needsPassphrase: false,
  problem: null,
  warnings: [],
  nameTaken: false,
  ...overrides,
});

const problem = {
  code: 'validation' as const,
  key: '',
  params: {},
  message: 'Its ProxyJump chain makes a loop through a.',
  hint: null,
  detail: null,
};

describe('import rows', () => {
  it('picks what can be imported, except names already taken', () => {
    const rows = rowsFrom([
      ssh('prod'),
      ssh('taken', { nameTaken: true }),
      ssh('loop', { connection: null, problem }),
    ]);
    expect(rows.map((r) => r.selected)).toEqual([true, false, false]);
    expect(rows.map(canPick)).toEqual([true, true, false]);
    expect(rows[0]!.socketPath).toBe('/var/run/docker.sock');
  });

  it('sends the picked rows with what was changed', () => {
    const rows = rowsFrom([ssh('prod', { needsPassphrase: true }), ssh('other')]);
    rows[0] = {
      ...rows[0]!,
      name: ' production ',
      passphrase: 'pw',
      socketPath: '/run/podman.sock',
    };
    rows[1] = { ...rows[1]!, selected: false };
    expect(toChoices(rows)).toEqual([
      { source: 'prod', name: 'production', socketPath: '/run/podman.sock', passphrase: 'pw' },
    ]);
  });

  it('holds the import until every picked row can go', () => {
    const [row] = rowsFrom([ssh('prod', { needsPassphrase: true })]);
    const check = (patch: object) => {
      const rows = [{ ...row!, ...patch }];
      return [rowProblem(rows[0]!, rows), toChoices(rows)];
    };
    expect(check({})).toEqual(['hosts.import.problem.passphrase', null]);
    expect(check({ passphrase: 'pw', name: ' ' })).toEqual([
      'hosts.import.problem.nameEmpty',
      null,
    ]);
    expect(check({ passphrase: 'pw', socketPath: 'docker.sock' })[0]).toBe(
      'hosts.import.problem.socket',
    );
    expect(check({ passphrase: 'pw' })[0]).toBeNull();
    expect(toChoices([{ ...row!, selected: false }])).toBeNull();
  });

  it('asks for another name when one is taken or used twice', () => {
    const rows = rowsFrom([ssh('prod', { nameTaken: true }), ssh('a'), ssh('b')]);
    rows[0] = { ...rows[0]!, selected: true };
    expect(rowProblem(rows[0]!, rows)).toBe('hosts.import.problem.nameTaken');
    rows[0] = { ...rows[0]!, name: 'prod-2' };
    expect(rowProblem(rows[0]!, rows)).toBeNull();
    rows[2] = { ...rows[2]!, name: 'A' };
    expect(rowProblem(rows[1]!, rows)).toBe('hosts.import.problem.nameTwice');
  });

  it('sends no socket for Hosts that aren’t SSH', () => {
    const rows = rowsFrom([
      ssh('lab', { connection: { kind: 'tcp', address: 'lab:2375' }, files: [] }),
    ]);
    expect(toChoices(rows)).toEqual([
      { source: 'lab', name: 'lab', socketPath: null, passphrase: null },
    ]);
  });

  it('sums up how a draft logs in', () => {
    expect(authSummary(ssh('a'))).toBe('hosts.import.auth.key');
    expect(authSummary({ ...ssh('a'), connection: { kind: 'tls', address: 'h:1' } })).toBe(
      'hosts.import.auth.certificates',
    );
    expect(authSummary({ ...ssh('a'), connection: null })).toBeNull();
  });
});
