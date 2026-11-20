import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterView, nomadClusterView } from '@/test/clusters';
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

  it('renames it from the "More actions" menu', async () => {
    const renamed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_list: () => [clusterView('kind-dev')],
      clusters_update: (args) => {
        renamed.push(args);
        return clusterView('dev');
      },
    });
    const user = userEvent.setup();
    renderApp('/clusters/c1');
    await user.click(await screen.findByRole('button', { name: 'More actions' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Rename…' }));
    const dialog = await screen.findByRole('dialog');
    const name = within(dialog).getByLabelText('Name');
    await user.clear(name);
    await user.type(name, 'dev');
    await user.click(within(dialog).getByRole('button', { name: 'Rename' }));
    await waitFor(() => expect(renamed).toEqual([{ id: 'c1', name: 'dev' }]));
    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
  });

  it('removes a Nomad cluster without speaking of a kubeconfig', async () => {
    mockCommands({ ...baseCommands, clusters_list: () => [nomadClusterView('nomad-prod')] });
    const user = userEvent.setup();
    renderApp('/clusters/n1');
    const zone = await screen.findByRole('region', { name: 'Remove cluster' });
    expect(zone).toHaveTextContent('Nothing changes on the cluster.');
    await user.click(within(zone).getByRole('button', { name: 'Remove cluster' }));
    expect(
      await within(await screen.findByRole('alertdialog')).findByText(
        'You can add it again by its address.',
      ),
    ).toBeInTheDocument();
  });
});
