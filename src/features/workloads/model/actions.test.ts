import { describe, expect, it } from 'vitest';
import { parseReplicas, workloadActions } from './actions';

describe('workloadActions', () => {
  it('scales only what has replicas and restarts only what rolls out', () => {
    expect(workloadActions('deployment')).toEqual({ scale: true, restart: true, cron: false });
    expect(workloadActions('daemonSet')).toEqual({ scale: false, restart: true, cron: false });
    expect(workloadActions('job')).toEqual({ scale: false, restart: false, cron: false });
    expect(workloadActions('cronJob')).toEqual({ scale: false, restart: false, cron: true });
  });
});

describe('parseReplicas', () => {
  it('takes whole numbers from zero', () => {
    expect(parseReplicas(' 0 ')).toBe(0);
    expect(parseReplicas('12')).toBe(12);
    expect(parseReplicas('-1')).toBeNull();
    expect(parseReplicas('1.5')).toBeNull();
    expect(parseReplicas('')).toBeNull();
    expect(parseReplicas('99999999999')).toBeNull();
  });
});
