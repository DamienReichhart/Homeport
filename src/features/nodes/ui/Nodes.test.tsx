import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const node = (name: string, ready: boolean) => ({
  name,
  ready,
  unschedulable: !ready,
  roles: ['control-plane'],
  version: 'v1.33.1',
  os: 'linux',
  arch: 'arm64',
  internalIp: '10.0.0.4',
  cpu: '4',
  memory: '16302048Ki',
  taints: 0,
  created: null,
});

describe('nodes', () => {
  it('lists Nodes with their readiness and opens one with its Pods', async () => {
    const pods: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_nodes_list: () => [node('node-a', true), node('node-b', false)],
      clusters_node_get: () => ({
        view: node('node-a', true),
        labels: {},
        annotations: {},
        conditions: [],
        capacity: { cpu: '4', memory: '15335132Ki' },
        allocatable: { cpu: '3800m' },
        taints: [],
        osImage: 'Debian GNU/Linux 12',
        kernel: '6.1',
        containerRuntime: 'containerd://2.0',
        addresses: ['InternalIP 10.0.0.4'],
      }),
      clusters_pods_list: (args) => {
        pods.push(args);
        return [];
      },
    });
    const user = userEvent.setup();
    renderApp('/nodes');
    expect(await screen.findByText('Not ready')).toBeInTheDocument();
    expect(screen.getByText('scheduling disabled', { exact: false })).toBeInTheDocument();
    await user.click(screen.getByText('node-a'));
    expect(await screen.findByText('3.8')).toBeInTheDocument();
    expect(screen.getByText('14.6 GB')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Pods' }));
    await waitFor(() =>
      expect(pods).toEqual([{ id: 'c1', namespace: null, selector: null, node: 'node-a' }]),
    );
  });

  it('says when the credentials can’t list Nodes', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_nodes_list: () => {
        throw { code: 'forbidden', message: 'Your credentials can’t list nodes.' };
      },
    });
    renderApp('/nodes');
    expect(await screen.findByText('Your credentials can’t list nodes.')).toBeInTheDocument();
  });
});
