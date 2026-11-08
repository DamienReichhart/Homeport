import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

const swarm = { id: 's1', managers: ['h1'], workers: [], reachable: true };
const object = (name: string, services: string[]) => ({
  id: `${name}-id`,
  name,
  created: null,
  updated: null,
  labels: {},
  services,
});

function setup(created: unknown[]) {
  mockCommands({
    ...baseCommands,
    swarms_list: () => [swarm],
    swarm_inspect: () => ({
      id: 's1',
      created: null,
      updated: null,
      nodes: 1,
      managers: 1,
      services: 1,
    }),
    swarm_secrets_list: () => [object('db_password', ['shop_db'])],
    swarm_configs_list: () => [object('nginx_conf', [])],
    swarm_configs_inspect: () => ({ text: 'server { listen 80; }', size: 21 }),
    swarm_secrets_create: ({ input }) => {
      created.push(input);
      return 'new-id';
    },
  });
  return userEvent.setup();
}

describe('secrets and configs', () => {
  it('lists the services using a secret, and never asks for its value', async () => {
    setup([]);
    renderApp('/swarms/s1/secrets');
    const row = (await screen.findByText('db_password')).closest('tr')!;
    expect(within(row).getByText('shop_db')).toBeInTheDocument();
    expect(within(row).queryByRole('button', { name: /View/ })).not.toBeInTheDocument();
  });

  it('forgets a typed secret value once the dialog closes', async () => {
    const user = setup([]);
    renderApp('/swarms/s1/secrets');
    await user.click(await screen.findByRole('button', { name: 'Create secret' }));
    await user.type(screen.getByLabelText('Value'), 'hunter2');
    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
    await user.click(screen.getByRole('button', { name: 'Create secret' }));
    expect(screen.getByLabelText('Value')).toHaveValue('');
  });

  it('keeps no secret value in the mutation cache after creating it', async () => {
    const created: unknown[] = [];
    const user = setup(created);
    const client = clientWithoutRetries();
    renderApp('/swarms/s1/secrets', client);
    await user.click(await screen.findByRole('button', { name: 'Create secret' }));
    await user.type(screen.getByLabelText('Name'), 'api_token');
    await user.type(screen.getByLabelText('Value'), 'hunter2');
    await user.click(screen.getByRole('button', { name: 'Create' }));
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
    expect(created).toEqual([
      { name: 'api_token', value: { kind: 'text', text: 'hunter2' }, labels: {} },
    ]);
    await waitFor(() =>
      expect(
        JSON.stringify(
          client
            .getMutationCache()
            .getAll()
            .map((m) => m.state),
        ),
      ).not.toContain('hunter2'),
    );
  });

  it('shows a config’s content', async () => {
    const user = setup([]);
    renderApp('/swarms/s1/configs');
    await user.click(await screen.findByRole('button', { name: 'View nginx_conf' }));
    expect(await screen.findByText('server { listen 80; }')).toBeInTheDocument();
  });
});
