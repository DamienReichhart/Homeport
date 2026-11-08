import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

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

export const details = (revision: number) => ({
  userValues: `replicaCount: ${revision}\n`,
  computedValues: `replicaCount: ${revision}\nimage:\n  tag: 1.27\n`,
  notes: 'Visit http://web.shop\n',
  manifest: 'apiVersion: v1\nkind: Service\nmetadata:\n  name: web\n',
  resources: [{ apiVersion: 'v1', kind: 'Service', name: 'web', namespace: null }],
});

describe('helm releases', () => {
  it('lists releases with their chart and flags failed and pending ones', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_helm_releases: () => ({
        releases: [
          release('web'),
          release('db', 'failed'),
          release('api', 'pending-upgrade', { chartName: 'api', chartVersion: '0.3.0' }),
        ],
        onlyNamespace: null,
      }),
    });
    renderApp('/helm');
    expect(await screen.findByText('3 releases')).toBeInTheDocument();
    expect(screen.getByText('failed').querySelector('[data-tone]')).toHaveAttribute(
      'data-tone',
      'danger',
    );
    expect(screen.getByText('pending-upgrade').querySelector('[data-tone]')).toHaveAttribute(
      'data-tone',
      'warning',
    );
    expect(screen.getAllByText('deployed')[0].querySelector('[data-tone]')).toHaveAttribute(
      'data-tone',
      'success',
    );
  });

  it('says when only the default Namespace could be listed', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_helm_releases: () => ({ releases: [release('web')], onlyNamespace: 'shop' }),
    });
    renderApp('/helm');
    expect(
      await screen.findByText(/only those in shop are shown/, { exact: false }),
    ).toBeInTheDocument();
  });

  it('explains a missing helm instead of the list', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      get_capabilities: () => ({
        ...baseCommands.get_capabilities(),
        helm: {
          state: 'unavailable',
          problem: {
            code: 'helm_missing',
            message: 'The helm CLI isn’t installed on this computer.',
            hint: { text: 'Install Helm 3 with Homebrew.', commands: ['brew install helm'] },
            detail: null,
          },
        },
      }),
    });
    renderApp('/helm');
    expect(
      await screen.findByText('The helm CLI isn’t installed on this computer.'),
    ).toBeInTheDocument();
    expect(screen.getByText('brew install helm')).toBeInTheDocument();
  });

  it('opens a release with its history, values of any revision and resources', async () => {
    const asked: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_helm_releases: () => ({ releases: [release('web')], onlyNamespace: null }),
      clusters_helm_history: () => history,
      clusters_helm_release_details: (args: { revision: number }) => {
        asked.push(args.revision);
        return details(args.revision);
      },
      clusters_api_resources: () => [],
    });
    const user = userEvent.setup();
    renderApp('/helm');
    await user.click(await screen.findByText('web'));
    expect(await screen.findByText('Upgrade complete')).toBeInTheDocument();
    expect(screen.getByText('Install complete')).toBeInTheDocument();

    await user.click(screen.getByRole('tab', { name: 'Values' }));
    expect(await screen.findByLabelText('Values as YAML')).toHaveTextContent('replicaCount: 2');
    await user.click(screen.getByRole('radio', { name: 'Computed, with the chart’s defaults' }));
    expect(screen.getByLabelText('Values as YAML')).toHaveTextContent('tag: 1.27');

    await user.click(screen.getByRole('tab', { name: 'Resources' }));
    expect(await screen.findByRole('link', { name: 'web' })).toHaveAttribute(
      'href',
      '/services/shop/web',
    );
    await waitFor(() => expect(asked).toEqual([2]));
  });
});
