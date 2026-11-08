import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp, renderWithProviders } from '@/test/render';
import { ForwardDialog } from './ForwardDialog';

const forward = (localPort: number) => ({
  id: 1,
  clusterId: 'c1',
  clusterName: 'kind-dev',
  target: { kind: 'service', namespace: 'shop', name: 'api' },
  remotePort: 80,
  localPort,
  addresses: [`127.0.0.1:${localPort}`],
  connections: 2,
  pod: 'api-1',
  started: '2026-11-08T10:00:00Z',
});

describe('port forwards', () => {
  it('proposes the same local port when it is free', async () => {
    const started: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_forward_port_free: () => true,
      clusters_forward_start: (args) => {
        started.push(args);
        return forward(80);
      },
    });
    const user = userEvent.setup();
    const onClose = vi.fn();
    renderWithProviders(
      <ForwardDialog
        clusterId="c1"
        target={{ kind: 'service', namespace: 'shop', name: 'api' }}
        port={80}
        onClose={onClose}
      />,
    );
    expect(await screen.findByLabelText('Local port')).toHaveValue('80');
    await user.click(screen.getByRole('button', { name: 'Forward' }));
    await waitFor(() => expect(onClose).toHaveBeenCalled());
    expect(started).toEqual([
      {
        id: 'c1',
        target: { kind: 'service', namespace: 'shop', name: 'api' },
        remotePort: 80,
        localPort: 80,
      },
    ]);
  });

  it('leaves the local port empty, for any free one, when that port is taken', async () => {
    const started: unknown[] = [];
    mockCommands({
      ...baseCommands,
      clusters_forward_port_free: () => false,
      clusters_forward_start: (args) => {
        started.push(args);
        return forward(51234);
      },
    });
    const user = userEvent.setup();
    renderWithProviders(
      <ForwardDialog
        clusterId="c1"
        target={{ kind: 'pod', namespace: 'shop', name: 'db-0' }}
        port={5432}
        onClose={() => {}}
      />,
    );
    expect(await screen.findByLabelText('Local port')).toHaveValue('');
    await user.click(screen.getByRole('button', { name: 'Forward' }));
    await waitFor(() => expect(started).toMatchObject([{ localPort: null }]));
  });

  it('lists the forwards with their connections and stops one', async () => {
    const stopped: unknown[] = [];
    let list = [forward(8080)];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_forward_list: () => list,
      clusters_forward_stop: ({ forward: id }) => {
        stopped.push(id);
        list = [];
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/port-forwards');
    expect(await screen.findByText('127.0.0.1:8080')).toBeInTheDocument();
    expect(screen.getByText('svc/api:80')).toBeInTheDocument();
    expect(screen.getAllByText('kind-dev').length).toBeGreaterThan(0);
    await user.click(screen.getByRole('button', { name: 'Stop' }));
    await waitFor(() => expect(stopped).toEqual([1]));
    expect(await screen.findByText('No port forwards')).toBeInTheDocument();
  });
});
