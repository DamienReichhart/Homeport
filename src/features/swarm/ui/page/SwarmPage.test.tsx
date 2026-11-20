import { screen } from '@testing-library/react';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const manager2 = hostView('manager-2', { id: 'h2' });
const worker = hostView('worker', { id: 'h3' });
const swarm = { id: 'swarm123456789abc', managers: ['h1', 'h2'], workers: ['h3'], reachable: true };

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
