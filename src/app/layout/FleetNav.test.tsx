import { screen, waitFor, within } from '@testing-library/react';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const manager2 = hostView('manager-2', { id: 'h2' });
const worker = hostView('worker', { id: 'h3' });
const swarm = { id: 'swarm123456789abc', managers: ['h1', 'h2'], workers: ['h3'], reachable: true };

function sidebar() {
  return screen.getByRole('navigation', { name: 'Main' });
}

describe('the Swarms in the sidebar', () => {
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
    await waitFor(() =>
      expect(screen.getByRole('link', { name: /Containers/ })).toBeInTheDocument(),
    );
    expect(within(sidebar()).queryByText('Swarms')).not.toBeInTheDocument();
  });
});
