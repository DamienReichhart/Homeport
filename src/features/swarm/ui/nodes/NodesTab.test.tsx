import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, connectedHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const swarm = { id: 's1', managers: ['h1'], workers: [], reachable: true };

function node(id: string, hostname: string, role: 'manager' | 'worker', state = 'ready') {
  return {
    id,
    version: 7,
    hostname,
    role,
    availability: 'active',
    state,
    message: null,
    managerStatus: role === 'manager' ? 'leader' : null,
    engineVersion: '29.1.0',
    address: '10.0.0.1',
    os: 'linux',
    arch: 'x86_64',
    labels: { zone: 'a' },
    created: null,
    hostId: role === 'manager' ? 'h1' : null,
  };
}

function setup(calls: { cmd: string; args: unknown }[]) {
  mockCommands({
    ...baseCommands,
    swarms_list: () => [swarm],
    swarm_inspect: () => ({
      id: 's1',
      created: null,
      updated: null,
      nodes: 2,
      managers: 1,
      services: 0,
    }),
    swarm_nodes_list: () => [node('n1', 'alpha', 'manager'), node('n2', 'beta', 'worker', 'down')],
    swarm_nodes_update: (args) => {
      calls.push({ cmd: 'update', args });
      return null;
    },
    swarm_nodes_remove: (args) => {
      calls.push({ cmd: 'remove', args });
      return null;
    },
  });
  return userEvent.setup();
}

describe('the Nodes tab', () => {
  it('drains a node from its menu, sending the version it was read at', async () => {
    const calls: { cmd: string; args: unknown }[] = [];
    const user = setup(calls);
    renderApp('/swarms/s1/nodes');
    const row = (await screen.findByText('alpha')).closest('tr')!;
    expect(within(row).getByRole('link', { name: 'host local' })).toHaveAttribute(
      'href',
      `/hosts/${connectedHost.host.id}`,
    );
    await user.click(within(row).getByRole('button', { name: 'Actions for alpha' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Drain' }));
    expect(calls).toEqual([
      {
        cmd: 'update',
        args: {
          swarmId: 's1',
          id: 'n1',
          version: 7,
          change: { availability: 'drain', role: null, labels: null },
        },
      },
    ]);
  });

  it('removes a down node without asking for Force', async () => {
    const calls: { cmd: string; args: unknown }[] = [];
    const user = setup(calls);
    renderApp('/swarms/s1/nodes');
    const row = (await screen.findByText('beta')).closest('tr')!;
    await user.click(within(row).getByRole('button', { name: 'Actions for beta' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Remove' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).queryByLabelText(/Force/)).not.toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Remove' }));
    expect(calls).toEqual([{ cmd: 'remove', args: { swarmId: 's1', id: 'n2', force: false } }]);
  });

  it('needs Force to remove a node that is up', async () => {
    const calls: { cmd: string; args: unknown }[] = [];
    const user = setup(calls);
    renderApp('/swarms/s1/nodes');
    const row = (await screen.findByText('alpha')).closest('tr')!;
    await user.click(within(row).getByRole('button', { name: 'Actions for alpha' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Remove' }));
    const dialog = await screen.findByRole('dialog');
    const remove = within(dialog).getByRole('button', { name: 'Remove' });
    expect(remove).toBeDisabled();
    await user.click(within(dialog).getByLabelText(/Force/));
    await user.click(remove);
    expect(calls).toEqual([{ cmd: 'remove', args: { swarmId: 's1', id: 'n1', force: true } }]);
  });
});
