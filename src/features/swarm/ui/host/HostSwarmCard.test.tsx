import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { HostStatus, SwarmMembership } from '@/shared/api';
import { baseCommands, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const manager: SwarmMembership = {
  swarmId: 's1',
  nodeId: 'n1',
  role: 'manager',
  state: 'active',
  nodeAddr: '10.0.0.1',
  managers: [],
  error: null,
};

const connected = (swarm: SwarmMembership | null): HostStatus => ({
  state: 'connected',
  engineVersion: '29.1',
  apiVersion: '1.52',
  os: 'linux',
  arch: 'amd64',
  podmanVersion: null,
  containerdVersion: null,
  swarm,
});

function setup(swarm: SwarmMembership | null, managers = 1) {
  const calls: { cmd: string; args: unknown }[] = [];
  const host = hostView('prod', {
    status: connected(swarm),
    connection: { kind: 'tcp', address: '192.168.1.10:2375' },
  });
  mockCommands({
    ...baseCommands,
    hosts_list: () => [host],
    swarms_list: () =>
      swarm ? [{ id: 's1', managers: [host.host.id], workers: [], reachable: true }] : [],
    swarm_inspect: () => ({
      id: 's1',
      created: null,
      updated: null,
      nodes: 1,
      managers,
      services: 0,
    }),
    swarm_init: (args) => {
      calls.push({ cmd: 'init', args });
      return 's1';
    },
    swarm_leave: (args) => {
      calls.push({ cmd: 'leave', args });
      return null;
    },
  });
  renderApp(`/hosts/${host.host.id}`);
  return { calls, user: userEvent.setup() };
}

describe('the Swarm card of a host', () => {
  it('offers to start a Swarm, advertising the address Homeport reaches the host at', async () => {
    const { calls, user } = setup(null);
    expect(await screen.findByText('This host isn’t in a Swarm.')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Init a Swarm' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByLabelText('Advertise address')).toHaveValue('192.168.1.10');
    await user.click(within(dialog).getByRole('button', { name: 'Start the Swarm' }));
    expect(calls).toEqual([
      {
        cmd: 'init',
        args: { hostId: 'prod-id', advertiseAddr: '192.168.1.10', listenAddr: '0.0.0.0:2377' },
      },
    ]);
  });

  it('makes the last manager type the host’s name to leave', async () => {
    const { calls, user } = setup(manager, 1);
    expect(
      await screen.findAllByRole('link', { name: 'Manager of Swarm of prod' }),
    ).not.toHaveLength(0);
    await user.click(screen.getByRole('button', { name: 'Leave the Swarm' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByText(/last manager/)).toBeInTheDocument();
    const leave = within(dialog).getByRole('button', { name: 'Leave' });
    expect(leave).toBeDisabled();
    await user.type(within(dialog).getByLabelText('Type prod to confirm'), 'prod');
    await user.click(leave);
    expect(calls).toEqual([{ cmd: 'leave', args: { hostId: 'prod-id', force: true } }]);
  });

  it('asks a manager among others for Force only', async () => {
    const { user } = setup(manager, 3);
    await user.click(await screen.findByRole('button', { name: 'Leave the Swarm' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).queryByLabelText(/to confirm/)).not.toBeInTheDocument();
    await user.click(within(dialog).getByLabelText(/Force/));
    expect(within(dialog).getByRole('button', { name: 'Leave' })).toBeEnabled();
  });
});
