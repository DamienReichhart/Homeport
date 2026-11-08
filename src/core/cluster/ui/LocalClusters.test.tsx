import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, clusterView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { LocalClusters } from './LocalClusters';

const local = (overrides: Record<string, unknown>) => ({
  tool: 'dockerDesktop',
  context: 'docker-desktop',
  suggestedName: 'Docker Desktop',
  server: 'https://kubernetes.docker.internal:6443',
  serverVersion: 'v1.34.1',
  added: null,
  ...overrides,
});

describe('local clusters', () => {
  it('adds Docker Desktop’s cluster in one click, and says which aren’t running', async () => {
    const imported: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_detect_local: () => ({
        found: [
          local({}),
          local({
            tool: 'kind',
            context: 'kind-old',
            suggestedName: 'kind-old',
            serverVersion: null,
          }),
        ],
        missing: [{ tool: 'k3d', name: 'shop', command: 'k3d kubeconfig merge shop' }],
      }),
      clusters_import: (args) => {
        imported.push(args);
        return [clusterView('Docker Desktop')];
      },
    });
    const user = userEvent.setup();
    renderWithProviders(<LocalClusters />);
    expect(await screen.findByText('Docker Desktop')).toBeInTheDocument();
    expect(screen.getByText('Not running')).toBeInTheDocument();
    expect(
      screen.getByText(
        'The k3d cluster shop is running, but your kubeconfig doesn’t reach it. Add it with:',
      ),
    ).toBeInTheDocument();
    expect(screen.getByText('k3d kubeconfig merge shop')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Add' }));
    await waitFor(() =>
      expect(imported).toEqual([
        { contexts: [{ context: 'docker-desktop', name: 'Docker Desktop' }], text: null },
      ]),
    );
  });

  it('shows nothing when no local cluster is found', async () => {
    let asked = false;
    mockCommands({
      ...baseCommands,
      clusters_detect_local: () => {
        asked = true;
        return { found: [], missing: [] };
      },
    });
    renderWithProviders(<LocalClusters />);
    await waitFor(() => expect(asked).toBe(true));
    expect(screen.queryByText('Clusters on this machine')).not.toBeInTheDocument();
  });
});
