import type { BundleHost } from '@/shared/api';
import { bundleChoices, bundleRowProblem, bundleRows, passphraseProblem } from './bundle';

const host = (index: number, name: string, overrides: Partial<BundleHost> = {}): BundleHost => ({
  index,
  name,
  suggestedName: name,
  nameTaken: false,
  connection: { kind: 'tcp', address: `${name}:2375` },
  secretsMissing: false,
  ...overrides,
});

describe('export passphrase', () => {
  it('needs twelve characters, typed the same twice', () => {
    expect(passphraseProblem('short', 'short')).toBe('hosts.bundle.problem.short');
    expect(passphraseProblem('twelve chars', 'twelve charz')).toBe('hosts.bundle.problem.mismatch');
    expect(passphraseProblem('twelve chars', 'twelve chars')).toBeNull();
    // Characters, not bytes: "é" counts once.
    expect(passphraseProblem('ééééééééééé', 'ééééééééééé')).toBe('hosts.bundle.problem.short');
  });
});

describe('import rows', () => {
  it('picks every Host under its suggested name', () => {
    const rows = bundleRows([host(0, 'prod', { nameTaken: true, suggestedName: 'prod (2)' })]);
    expect(rows).toEqual([expect.objectContaining({ selected: true, name: 'prod (2)' })]);
    expect(bundleChoices(rows)).toEqual([{ index: 0, name: 'prod (2)' }]);
  });

  it('skips unpicked Hosts and waits for free, distinct names', () => {
    const rows = bundleRows([host(0, 'a'), host(1, 'b'), host(2, 'prod', { nameTaken: true })]);
    rows[1] = { ...rows[1]!, selected: false };
    expect(bundleChoices(rows)).toBeNull();
    expect(bundleRowProblem(rows[2]!, rows)).toBe('hosts.import.problem.nameTaken');
    rows[2] = { ...rows[2]!, name: 'A' };
    expect(bundleRowProblem(rows[2]!, rows)).toBe('hosts.import.problem.nameTwice');
    rows[2] = { ...rows[2]!, name: 'prod-old' };
    expect(bundleChoices(rows)).toEqual([
      { index: 0, name: 'a' },
      { index: 2, name: 'prod-old' },
    ]);
    expect(bundleChoices(rows.map((r) => ({ ...r, selected: false })))).toBeNull();
  });
});
