import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands, podView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('pods', () => {
  it('lists Pods, filters them, and remembers the namespace picked', async () => {
    const saved: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      update_settings: ({ settings }) => {
        saved.push(settings as Record<string, unknown>);
        return settings;
      },
      clusters_pods_list: ({ namespace }) =>
        namespace === null
          ? [podView('web-1'), podView('db-0', { namespace: 'shop' })]
          : [
              podView('web-1'),
              podView('web-2', { status: 'CrashLoopBackOff', health: 'crashing', restarts: 7 }),
            ],
    });
    const user = userEvent.setup();
    renderApp('/pods');
    expect(await screen.findByText('web-2')).toBeInTheDocument();
    expect(screen.getByText('CrashLoopBackOff')).toBeInTheDocument();
    await user.type(screen.getByPlaceholderText('Search pods'), 'web-1');
    await waitFor(() => expect(screen.queryByText('web-2')).not.toBeInTheDocument());
    await user.clear(screen.getByPlaceholderText('Search pods'));

    await user.click(screen.getByRole('combobox', { name: 'Namespace' }));
    await user.click(await screen.findByRole('option', { name: 'All namespaces' }));
    expect(await screen.findByText('shop')).toBeInTheDocument();
    await waitFor(() => expect(saved.at(-1)?.clusterNamespaces).toEqual({ c1: '' }));
  });

  it('shows thousands of Pods a page at a time', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_pods_list: () => Array.from({ length: 2500 }, (_, i) => podView(`pod-${i}`)),
    });
    renderApp('/pods');
    expect(await screen.findByText('Page 1 of 25 · 2,500 rows')).toBeInTheDocument();
    expect(screen.getAllByRole('row')).toHaveLength(101);
  });

  it('opens a Pod with its containers', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_pod_get: () => ({
        view: podView('web-1'),
        labels: {},
        annotations: {},
        conditions: [],
        containers: [
          {
            name: 'migrate',
            image: 'app',
            init: true,
            state: 'terminated',
            reason: 'Completed',
            message: null,
            started: null,
            exitCode: 0,
            ready: false,
            restarts: 0,
            lastReason: null,
          },
          {
            name: 'web',
            image: 'nginx',
            init: false,
            state: 'running',
            reason: null,
            message: null,
            started: null,
            exitCode: null,
            ready: true,
            restarts: 2,
            lastReason: 'OOMKilled',
          },
        ],
        hostIp: '192.168.1.5',
        qosClass: 'Burstable',
        serviceAccount: 'default',
      }),
    });
    renderApp('/pods/default/web-1');
    expect(await screen.findByText('Burstable')).toBeInTheDocument();
    expect(screen.getByText('2 (last: OOMKilled)')).toBeInTheDocument();
    expect(screen.getByText('init')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'node-a' })).toBeInTheDocument();
  });
});
