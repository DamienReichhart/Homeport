import { screen, within } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { clusterView, nomadClusterView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('clusters page', () => {
  it('lists every Cluster, naming only the states that need a look', async () => {
    const broken = nomadClusterView('nomad-prod', {
      status: {
        state: 'unauthorized',
        error: {
          code: 'unauthorized',
          key: '',
          params: {},
          message: 'The token was refused.',
          hint: null,
          detail: null,
        },
      },
    });
    mockCommands({
      ...baseCommands,
      clusters_list: () => [clusterView('kind-dev'), broken],
    });
    renderApp('/clusters');
    const page = within(screen.getByRole('main'));
    expect(await page.findByText('kind-dev')).toBeInTheDocument();
    expect(page.getByText('https://127.0.0.1:6443')).toBeInTheDocument();
    // A connected Cluster shows what it runs; what's wrong is named.
    expect(page.getByText('Kubernetes v1.33.1')).toBeInTheDocument();
    expect(page.getByText('Access refused')).toHaveAttribute(
      'title',
      'Access refused: The token was refused.',
    );
  });

  it('offers to add the first Cluster when there is none', async () => {
    mockCommands({ ...baseCommands, clusters_list: () => [] });
    renderApp('/clusters');
    const page = within(screen.getByRole('main'));
    expect(await page.findByText('No clusters yet')).toBeInTheDocument();
    expect(page.getAllByRole('link', { name: 'Add a cluster' }).length).toBe(2);
  });
});
