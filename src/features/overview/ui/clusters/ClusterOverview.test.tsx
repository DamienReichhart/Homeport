import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, connectedHost } from '@/test/fixtures';
import { clusterView, workloadView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const summary = {
  nodes: {
    state: 'failed',
    error: {
      code: 'forbidden',
      message: 'Your credentials can’t list nodes.',
      hint: null,
      detail: null,
    },
  },
  pods: {
    state: 'read',
    value: { total: 5, running: 3, pending: 1, succeeded: 0, failed: 0, crashing: 1, other: 0 },
  },
  unready: {
    state: 'read',
    value: [workloadView('api', { ready: 1, desired: 3, health: 'progressing' })],
  },
  warnings: { state: 'read', value: [] },
};

describe('the Overview with Clusters', () => {
  it('shows a card per Host and Cluster, and the Cluster dashboard once one is selected', async () => {
    const saved: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost],
      clusters_list: () => [clusterView('kind-dev')],
      clusters_summary: () => summary,
      update_settings: ({ settings }) => {
        saved.push(settings as Record<string, unknown>);
        return settings;
      },
      system_info: () => {
        throw { code: 'unknown', message: 'not under test' };
      },
      system_disk_usage: () => {
        throw { code: 'unknown', message: 'not under test' };
      },
      hosts_events: () => [],
      usage_subscribe: () => 1,
      stream_cancel: () => null,
    });
    const user = userEvent.setup();
    renderApp('/');
    expect(await screen.findByText('3 of 5 pods running')).toBeInTheDocument();
    expect(screen.getByText('Nodes: not allowed')).toBeInTheDocument();

    await user.click(screen.getByText('kind-dev'));
    await waitFor(() => expect(saved.at(-1)?.lastSelection).toEqual({ kind: 'cluster', id: 'c1' }));
    expect(await screen.findByText('Workloads not ready')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /default\/api/ })).toHaveAttribute(
      'href',
      '/workloads/deployments/default/api',
    );
    expect(screen.getByText('Not allowed. Your credentials can’t list nodes.')).toBeInTheDocument();
    expect(screen.getByText('1 failing')).toBeInTheDocument();
  });
});
