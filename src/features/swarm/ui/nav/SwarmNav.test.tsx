import { screen, within } from '@testing-library/react';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const manager2 = hostView('manager-2', { id: 'h2' });
const worker = hostView('worker', { id: 'h3' });
const swarm = { id: 'swarm123456789abc', managers: ['h1', 'h2'], workers: ['h3'], reachable: true };

function sidebar() {
  return screen.getByRole('navigation', { name: 'Main' });
}

describe('the Swarms section of the sidebar', () => {
  it('lists a Swarm with two manager hosts once', async () => {
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost, manager2, worker],
      swarms_list: () => [swarm],
      system_info: () => {
        throw new Error('not needed');
      },
    });
    renderApp('/hosts');
    expect(await within(sidebar()).findByText('Swarms')).toBeInTheDocument();
    expect(within(sidebar()).getAllByRole('link', { name: 'Swarm of local' })).toHaveLength(1);
  });

  it('is absent while no host is in a Swarm', async () => {
    mockCommands({ ...baseCommands });
    renderApp('/hosts');
    expect(await screen.findByRole('link', { name: /Containers/ })).toBeInTheDocument();
    expect(within(sidebar()).queryByText('Swarms')).not.toBeInTheDocument();
  });
});

describe('the Swarm page', () => {
  it('explains that no manager is connected', async () => {
    mockCommands({
      ...baseCommands,
      swarms_list: () => [{ ...swarm, reachable: false }],
      swarm_inspect: () => {
        throw {
          code: 'swarm_unreachable',
          key: 'errors.swarm.unreachable',
          params: {},
          message: 'No manager of this Swarm is connected.',
          hint: {
            key: 'errors.swarm.unreachable.hint',
            params: { hosts: 'local, manager-2' },
            text: 'Connect one of its managers: local, manager-2.',
            commands: [],
          },
          detail: null,
        };
      },
    });
    renderApp(`/swarms/${swarm.id}`);
    expect(await screen.findByText('No manager of this Swarm is connected.')).toBeInTheDocument();
    expect(screen.getByText('Connect one of its managers: local, manager-2.')).toBeInTheDocument();
  });

  it('shows the hosts of a reachable Swarm', async () => {
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost, manager2, worker],
      swarms_list: () => [swarm],
      swarm_inspect: () => ({
        id: swarm.id,
        created: '2026-11-01T10:00:00Z',
        updated: null,
        nodes: 3,
        managers: 2,
        services: 4,
      }),
    });
    renderApp(`/swarms/${swarm.id}`);
    expect(await screen.findByText('3 nodes, 2 managers')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'worker' })).toHaveAttribute('href', '/hosts/h3');
  });
});
