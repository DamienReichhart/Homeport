import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { NetworkView } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const network = (name: string, containers: string[] = []): NetworkView => ({
  id: `${name}-id`,
  name,
  driver: 'bridge',
  scope: 'local',
  internal: false,
  attachable: false,
  subnets: ['172.20.0.0/16'],
  created: null,
  labels: {},
  stack: null,
  builtIn: false,
  containers,
});

describe('networks page', () => {
  it('removes a network once confirmed', async () => {
    const removed: string[] = [];
    mockCommands({
      ...baseCommands,
      networks_list: () => [network('backend')],
      networks_remove: ({ id }) => {
        removed.push(String(id));
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/networks');
    await user.click(await screen.findByLabelText('Remove backend'));
    const confirmation = await screen.findByRole('alertdialog');
    expect(within(confirmation).getByText(/No container uses it/)).toBeInTheDocument();
    await user.click(within(confirmation).getByRole('button', { name: 'Remove' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(removed).toEqual(['backend-id']);
  });

  it('keeps the confirmation open when the network is in use, and reports it once', async () => {
    mockCommands({
      ...baseCommands,
      networks_list: () => [network('backend', ['api-1'])],
      networks_remove: () => {
        throw {
          code: 'in_use',
          message: 'backend still has containers attached.',
          hint: { text: 'Disconnect them first.', commands: [] },
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/networks');
    await user.click(await screen.findByLabelText('Remove backend'));
    const confirmation = await screen.findByRole('alertdialog');
    expect(within(confirmation).getByText(/1 attached container/)).toBeInTheDocument();
    const remove = within(confirmation).getByRole('button', { name: 'Remove' });
    await user.click(remove);

    expect(await screen.findByText('backend still has containers attached.')).toBeInTheDocument();
    expect(screen.getAllByText('backend still has containers attached.')).toHaveLength(1);
    await waitFor(() => expect(remove).toBeEnabled());

    await user.click(within(confirmation).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
  });
});
