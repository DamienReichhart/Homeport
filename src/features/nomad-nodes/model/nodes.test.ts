import type { NomadNodeView } from '@/shared/api';
import { nodeTone, parseDuration } from './nodes';

const node = (overrides: Partial<NomadNodeView>) =>
  ({ status: 'ready', eligible: true, draining: false, ...overrides }) as NomadNodeView;

describe('a Nomad node', () => {
  it('reads as working only when ready, eligible and not draining', () => {
    expect(nodeTone(node({}))).toBe('success');
    expect(nodeTone(node({ draining: true }))).toBe('warning');
    expect(nodeTone(node({ eligible: false }))).toBe('warning');
    expect(nodeTone(node({ status: 'down' }))).toBe('danger');
    expect(nodeTone(node({ status: 'initializing' }))).toBe('info');
  });

  it('takes a drain deadline as Nomad’s CLI writes it', () => {
    expect(parseDuration('1h')).toBe(3600);
    expect(parseDuration(' 1h30m ')).toBe(5400);
    expect(parseDuration('90s')).toBe(90);
    expect(parseDuration('0m')).toBeNull();
    expect(parseDuration('soon')).toBeNull();
    expect(parseDuration('')).toBeNull();
    expect(parseDuration('9999h')).toBeNull();
  });
});
