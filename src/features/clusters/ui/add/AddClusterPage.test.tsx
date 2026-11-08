import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const context = (name: string, overrides: Record<string, unknown> = {}) => ({
  name,
  server: 'https://127.0.0.1:6443',
  user: name,
  namespace: null,
  auth: { method: 'clientCertificate' },
  current: false,
  added: null,
  problem: null,
  ...overrides,
});

function setup(extra: Record<string, (args: Record<string, unknown>) => unknown> = {}) {
  const imported: unknown[] = [];
  const refreshed: unknown[] = [];
  mockCommands({
    ...baseCommands,
    clusters_kubeconfig_contexts: ({ text }) => ({
      paths: text ? [] : ['/home/me/.kube/config'],
      contexts: [
        context('kind-dev', { current: true }),
        context('prod', { added: { id: 'c9', name: 'Production' } }),
        context('old-gke', {
          auth: null,
          problem: {
            code: 'kubeconfig_auth_provider_unsupported',
            message: 'Its user signs in with the gcp auth provider.',
          },
        }),
      ],
    }),
    clusters_import: (args) => {
      imported.push(args);
      return [clusterView('kind-dev')];
    },
    clusters_refresh: (args) => {
      refreshed.push(args);
      return clusterView('Production');
    },
    ...extra,
  });
  return { imported, refreshed };
}

describe('adding a cluster', () => {
  it('imports the current context of this machine’s kubeconfig under the name given', async () => {
    const { imported } = setup();
    const user = userEvent.setup();
    renderApp('/clusters/new');
    expect(await screen.findByText('Read from /home/me/.kube/config')).toBeInTheDocument();
    const name = screen.getByRole('textbox', { name: 'Name of the cluster made from kind-dev' });
    await user.clear(name);
    await user.type(name, 'Dev');
    await user.click(screen.getByRole('button', { name: 'Import 1 cluster' }));
    await waitFor(() =>
      expect(imported).toEqual([{ contexts: [{ context: 'kind-dev', name: 'Dev' }], text: null }]),
    );
  });

  it('offers to refresh a context that is already a cluster, and says why one can’t be imported', async () => {
    const { refreshed } = setup();
    const user = userEvent.setup();
    renderApp('/clusters/new');
    expect(await screen.findByText('Already added as Production.')).toBeInTheDocument();
    expect(screen.getByRole('checkbox', { name: 'Import prod' })).toBeDisabled();
    expect(screen.getByText('Its user signs in with the gcp auth provider.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Refresh it' }));
    await waitFor(() => expect(refreshed).toEqual([{ id: 'c9', text: null }]));
  });

  it('reads a pasted kubeconfig in memory and imports from it', async () => {
    const { imported } = setup();
    const user = userEvent.setup();
    renderApp('/clusters/new');
    await user.click(await screen.findByRole('tab', { name: 'Paste a kubeconfig' }));
    await user.type(screen.getByRole('textbox', { name: 'Kubeconfig' }), 'apiVersion: v1');
    await user.click(screen.getByRole('button', { name: 'Read contexts' }));
    await user.click(await screen.findByRole('button', { name: 'Import 1 cluster' }));
    await waitFor(() =>
      expect(imported).toEqual([
        { contexts: [{ context: 'kind-dev', name: 'kind-dev' }], text: 'apiVersion: v1' },
      ]),
    );
  });

  it('explains why there is nothing to read without a kubeconfig', async () => {
    setup({
      clusters_kubeconfig_contexts: () => {
        throw { code: 'not_found', message: 'There’s no kubeconfig at /home/me/.kube/config.' };
      },
    });
    renderApp('/clusters/new');
    expect(
      await screen.findByText('There’s no kubeconfig at /home/me/.kube/config.'),
    ).toBeInTheDocument();
  });
});
