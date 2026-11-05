import { checkDuration, healthTiming } from './health';

describe('healthchecks', () => {
  it('fills unset timings with the Engine defaults', () => {
    expect(
      healthTiming({
        command: 'true',
        intervalS: 10,
        timeoutS: null,
        retries: 1,
        startPeriodS: null,
      }),
    ).toBe('every 10s, timeout 30s, 1 retry, start period 0s');
  });

  it('says how long a check took', () => {
    expect(checkDuration('2026-11-04T10:00:00.000Z', '2026-11-04T10:00:00.250Z')).toBe('250 ms');
    expect(checkDuration('2026-11-04T10:00:00Z', '2026-11-04T10:00:02.5Z')).toBe('2.5 s');
    expect(checkDuration('', 'x')).toBeNull();
  });
});
