import { screen, waitFor, within } from '@testing-library/react';
import { baseCommands, connectedHost } from '@/test/fixtures';
import { clusterView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const settings = baseCommands.get_settings();

function withCluster(selected: 'host' | 'cluster') {
  mockCommands({
    ...baseCommands,
    get_settings: () => ({
      ...settings,
      lastSelection: selected === 'cluster' ? { kind: 'cluster', id: 'c1' } : null,
    }),
    hosts_list: () => [connectedHost],
    clusters_list: () => [clusterView('kind-dev')],
    containers_list: () => [],
    hosts_events: () => [],
    system_info: () => {
      throw { code: 'unknown', message: 'not under test' };
    },
    system_disk_usage: () => {
      throw { code: 'unknown', message: 'not under test' };
    },
    usage_subscribe: () => 1,
    stream_cancel: () => null,
  });
}

describe('pages of what is selected', () => {
  it('hides the Host pages from the sidebar while a Cluster is selected', async () => {
    withCluster('cluster');
    renderApp('/settings');
    const nav = screen.getByRole('navigation', { name: 'Main' });
    await waitFor(() =>
      expect(within(nav).queryByRole('link', { name: /Containers/ })).not.toBeInTheDocument(),
    );
    expect(within(nav).getByRole('link', { name: /Overview/ })).toBeInTheDocument();
    expect(within(nav).getByRole('link', { name: /Settings/ })).toBeInTheDocument();
  });

  it('shows the Host pages while a Host is selected', async () => {
    withCluster('host');
    renderApp('/settings');
    const nav = screen.getByRole('navigation', { name: 'Main' });
    expect(await within(nav).findByRole('link', { name: /Containers/ })).toBeInTheDocument();
  });

  it('goes to the Overview when a Host page opens while a Cluster is selected', async () => {
    withCluster('cluster');
    renderApp('/containers');
    await waitFor(() =>
      expect(screen.queryByRole('heading', { name: 'Containers' })).not.toBeInTheDocument(),
    );
    expect(await screen.findByRole('heading', { name: 'Overview' })).toBeInTheDocument();
  });

  it('lists Clusters under their own heading in the switcher', async () => {
    withCluster('cluster');
    renderApp('/settings');
    const trigger = await screen.findByRole('combobox', { name: 'Host' });
    await waitFor(() => expect(trigger).toHaveTextContent('kind-dev'));
  });
});
