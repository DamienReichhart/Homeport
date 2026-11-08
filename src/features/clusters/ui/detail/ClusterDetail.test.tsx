import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, clusterView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('a cluster’s page', () => {
  it('shows how it is reached, never its credentials, and refreshes it', async () => {
    const refreshed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_list: () => [clusterView('kind-dev')],
      clusters_refresh: (args) => {
        refreshed.push(args);
        return clusterView('kind-dev');
      },
    });
    const user = userEvent.setup();
    renderApp('/clusters/c1');
    expect(await screen.findByText('https://127.0.0.1:6443')).toBeInTheDocument();
    expect(screen.getByText('v1.33.1')).toBeInTheDocument();
    expect(screen.getByText('Client certificate')).toBeInTheDocument();
    expect(screen.getByText('Context kind-dev of this machine’s kubeconfig')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Refresh' }));
    await waitFor(() => expect(refreshed).toEqual([{ id: 'c1', text: null }]));
  });

  it('asks for the kubeconfig again to refresh a pasted cluster', async () => {
    const pasted = clusterView('kind-dev');
    pasted.cluster.connection.source = { context: 'kind-dev', pasted: true };
    mockCommands({ ...baseCommands, clusters_list: () => [pasted] });
    renderApp('/clusters/c1');
    expect(await screen.findByRole('textbox', { name: 'Kubeconfig' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Refresh' })).toBeDisabled();
  });
});
