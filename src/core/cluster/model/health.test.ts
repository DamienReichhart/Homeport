import { age, podTone, workloadHealth } from './health';

describe('health', () => {
  it('labels and colours a workload’s health', () => {
    expect(workloadHealth('degraded')).toEqual({ tone: 'danger', label: 'Degraded' });
    expect(podTone('crashing')).toBe('danger');
  });

  it('shows ages as kubectl does', () => {
    const now = Date.parse('2026-11-08T12:00:00Z');
    expect(age('2026-11-08T11:59:30Z', now)).toBe('30s');
    expect(age('2026-11-08T11:30:00Z', now)).toBe('30m');
    expect(age('2026-11-08T02:00:00Z', now)).toBe('10h');
    expect(age('2026-11-01T12:00:00Z', now)).toBe('7d');
    expect(age(null, now)).toBe('—');
  });
});
