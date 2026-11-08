import { kindOf, pathOf, readyText, workloadPath } from './kinds';

describe('workload kinds', () => {
  it('reads and writes the URL segment of each kind', () => {
    expect(kindOf('statefulsets')).toBe('statefulSet');
    expect(kindOf(undefined)).toBe('deployment');
    expect(kindOf('nonsense')).toBe('deployment');
    expect(pathOf('cronJob')).toBe('cronjobs');
    expect(workloadPath('job', 'kube system', 'a/b')).toBe('/workloads/jobs/kube%20system/a%2Fb');
  });

  it('shows ready counts, and a CronJob’s active Jobs', () => {
    expect(readyText(2, 3, 'deployment')).toBe('2/3');
    expect(readyText(1, 0, 'cronJob')).toBe('1');
  });
});
