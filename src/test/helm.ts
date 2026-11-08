// Fixtures of Helm releases, for the Helm pages' tests.

/** A release as `clusters_helm_releases` lists it. */
export const release = (name: string, status = 'deployed', overrides = {}) => ({
  name,
  namespace: 'shop',
  revision: 2,
  updated: '2026-11-01T18:02:03Z',
  status,
  chart: 'nginx-18.2.4',
  chartName: 'nginx',
  chartVersion: '18.2.4',
  appVersion: '1.27.2',
  ...overrides,
});

/** A release's history as `clusters_helm_history` returns it: revision 1, then 2. */
export const history = [
  {
    revision: 1,
    updated: '2026-10-30T12:00:00Z',
    status: 'superseded',
    chart: 'nginx-18.2.3',
    chartName: 'nginx',
    chartVersion: '18.2.3',
    appVersion: '1.27.1',
    description: 'Install complete',
  },
  {
    revision: 2,
    updated: '2026-11-01T18:02:03Z',
    status: 'deployed',
    chart: 'nginx-18.2.4',
    chartName: 'nginx',
    chartVersion: '18.2.4',
    appVersion: '1.27.2',
    description: 'Upgrade complete',
  },
];

/** What `clusters_helm_release_details` returns for `revision`. */
export const details = (revision: number) => ({
  userValues: `replicaCount: ${revision}\n`,
  computedValues: `replicaCount: ${revision}\nimage:\n  tag: 1.27\n`,
  notes: 'Visit http://web.shop\n',
  manifest: 'apiVersion: v1\nkind: Service\nmetadata:\n  name: web\n',
  resources: [{ apiVersion: 'v1', kind: 'Service', name: 'web', namespace: null }],
});
