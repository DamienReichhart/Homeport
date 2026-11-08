import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, serviceView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const swarm = { id: 's1', managers: ['h1'], workers: [], reachable: true };

function setup(calls: unknown[]) {
  mockCommands({
    ...baseCommands,
    swarms_list: () => [swarm],
    swarm_inspect: () => ({
      id: 's1',
      created: null,
      updated: null,
      nodes: 1,
      managers: 1,
      services: 2,
    }),
    swarm_services_list: () => [
      serviceView({ id: 'a', name: 'web', running: 1, desired: 3 }),
      serviceView({
        id: 'b',
        name: 'agent',
        mode: 'global',
        replicas: null,
        running: 2,
        desired: 2,
      }),
    ],
    swarm_services_scale: (args) => {
      calls.push(args);
      return null;
    },
  });
  return userEvent.setup();
}

describe('the Services tab', () => {
  it('lists services with running/desired, warning when under-replicated', async () => {
    setup([]);
    renderApp('/swarms/s1/services');
    const web = (await screen.findByText('web')).closest('tr')!;
    expect(within(web).getByText('1/3')).toHaveClass('text-warning');
    const agent = screen.getByText('agent').closest('tr')!;
    expect(within(agent).getByText('2/2')).not.toHaveClass('text-warning');
  });

  it('scales a replicated service, and offers no scaling for a global one', async () => {
    const calls: unknown[] = [];
    const user = setup(calls);
    renderApp('/swarms/s1/services');
    const agent = (await screen.findByText('agent')).closest('tr')!;
    await user.click(within(agent).getByRole('button', { name: 'Actions for agent' }));
    expect(screen.queryByRole('menuitem', { name: 'Scale' })).not.toBeInTheDocument();
    await user.keyboard('{Escape}');

    const web = screen.getByText('web').closest('tr')!;
    await user.click(within(web).getByRole('button', { name: 'Actions for web' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Scale' }));
    const dialog = await screen.findByRole('dialog');
    const input = within(dialog).getByLabelText('Replicas');
    await user.clear(input);
    await user.type(input, 'x');
    expect(within(dialog).getByRole('button', { name: 'Scale' })).toBeDisabled();
    await user.clear(input);
    await user.type(input, '5');
    await user.click(within(dialog).getByRole('button', { name: 'Scale' }));
    expect(calls).toEqual([{ swarmId: 's1', id: 'a', version: 3, replicas: 5 }]);
  });
});
