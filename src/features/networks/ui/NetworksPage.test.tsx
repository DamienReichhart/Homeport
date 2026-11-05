import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, network } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

describe('networks page', () => {
  it('lists networks and offers no removal for built-in ones', async () => {
    mockCommands({
      ...baseCommands,
      networks_list: () => [
        network('bridge', [], { builtIn: true, subnets: ['172.17.0.0/16'] }),
        network('backend', ['api-1'], { stack: 'shop', internal: true }),
      ],
    });
    renderApp('/networks');
    expect(await screen.findByText('backend')).toBeInTheDocument();
    expect(screen.getByText('2 networks')).toBeInTheDocument();
    expect(screen.getByText('built-in')).toBeInTheDocument();
    expect(screen.getByText('internal')).toBeInTheDocument();
    expect(screen.getByText('shop')).toBeInTheDocument();
    expect(screen.getByLabelText('Remove backend')).toBeInTheDocument();
    expect(screen.getByLabelText('Inspect bridge')).toBeInTheDocument();
    expect(screen.queryByLabelText('Remove bridge')).toBeNull();
  });

  it('creates a network through the dialog, sending only what was typed', async () => {
    const created: unknown[] = [];
    mockCommands({
      ...baseCommands,
      networks_list: () => [],
      networks_create: ({ input }) => {
        created.push(input);
        return 'new-id';
      },
    });
    const user = userEvent.setup();
    renderApp('/networks');
    await user.click(await screen.findByRole('button', { name: 'Create network' }));
    const dialog = await screen.findByRole('dialog');
    const create = within(dialog).getByRole('button', { name: 'Create' });
    expect(create).toBeDisabled();

    await user.type(within(dialog).getByLabelText('Name'), ' backend ');
    await user.type(within(dialog).getByLabelText('Subnet'), '10.10.0.0/16');
    await user.click(within(dialog).getByLabelText('Internal — no access to outside networks'));
    await user.click(create);

    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
    expect(created).toEqual([
      {
        name: 'backend',
        driver: null,
        subnet: '10.10.0.0/16',
        gateway: null,
        internal: true,
        attachable: false,
      },
    ]);
  });

  it('shows why a network can’t be created in the dialog', async () => {
    mockCommands({
      ...baseCommands,
      networks_list: () => [],
      networks_create: () => {
        throw {
          code: 'validation',
          message: 'The subnet overlaps another network.',
          hint: null,
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/networks');
    await user.click(await screen.findByRole('button', { name: 'Create network' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Name'), 'backend');
    await user.click(within(dialog).getByRole('button', { name: 'Create' }));
    expect(
      await within(dialog).findByText('The subnet overlaps another network.'),
    ).toBeInTheDocument();
  });

  it('shows a list that fails to load in place, with a retry', async () => {
    let attempts = 0;
    mockCommands({
      ...baseCommands,
      networks_list: () => {
        attempts += 1;
        if (attempts === 1) {
          throw { code: 'timeout', message: 'The Engine didn’t answer.', hint: null, detail: null };
        }
        return [network('backend')];
      },
    });
    const user = userEvent.setup();
    renderApp('/networks', clientWithoutRetries());
    expect(await screen.findByText('The Engine didn’t answer.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /Retry/ }));
    expect(await screen.findByText('backend')).toBeInTheDocument();
  });

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
    expect(removed).toEqual([]);
    await user.click(within(confirmation).getByRole('button', { name: 'Remove' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(removed).toEqual(['backend-id']);
  });

  it('removes nothing when the confirmation is cancelled', async () => {
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
    await user.click(within(confirmation).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(removed).toEqual([]);
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
