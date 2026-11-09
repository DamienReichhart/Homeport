import type { NomadGroupCounts, NomadJobView } from '@/shared/api';
import { jobCounts, jobPath, parseCount, shortOf } from './jobs';

const group = (name: string, running: number, desired: number | null): NomadGroupCounts => ({
  name,
  desired,
  running,
  starting: 0,
  queued: 0,
  failed: 0,
  lost: 0,
  complete: 0,
});

const job = (groups: NomadGroupCounts[]) => ({ groups }) as unknown as NomadJobView;

describe('a job’s counts', () => {
  it('sums its task groups’ running and desired allocations', () => {
    expect(jobCounts(job([group('api', 2, 3), group('worker', 1, 1)]))).toEqual({
      running: 3,
      desired: 4,
    });
  });

  it('doesn’t guess a desired count it couldn’t read', () => {
    expect(jobCounts(job([group('api', 2, null)])).desired).toBeNull();
    expect(shortOf(group('api', 2, 3))).toBe(true);
    expect(shortOf(group('api', 2, null))).toBe(false);
  });
});

describe('the scale dialog’s count', () => {
  it('accepts whole numbers within the scaling policy', () => {
    expect(parseCount('3', null)).toBe(3);
    expect(parseCount(' 0 ', null)).toBe(0);
    expect(parseCount('6', { min: 1, max: 5, enabled: true })).toBeNull();
    expect(parseCount('0', { min: 1, max: 5, enabled: true })).toBeNull();
    expect(parseCount('5', { min: 1, max: 5, enabled: true })).toBe(5);
    expect(parseCount('2.5', null)).toBeNull();
    expect(parseCount('', null)).toBeNull();
  });
});

it('escapes a job’s namespace and ID in its path', () => {
  expect(jobPath('default', 'backup/periodic-1700000000')).toBe(
    '/nomad/jobs/default/backup%2Fperiodic-1700000000',
  );
});
