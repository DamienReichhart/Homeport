import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container, inspectDocument } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

function page(state: string) {
  const web = container('web', null, state);
  const calls: unknown[] = [];
  mockCommands({
    ...baseCommands,
    containers_list: () => [web],
    containers_inspect: () => inspectDocument('web'),
    containers_remove: (args) => {
      calls.push(args);
      return null;
    },
    logs_subscribe: () => 1,
  });
  return { web, calls };
}

describe('a container page’s header', () => {
  it('shows Restart and Stop, with the rest in its menu', async () => {
    const { web } = page('running');
    renderApp(`/containers/${web.id}/inspect`, clientWithoutRetries());
    expect(await screen.findByRole('button', { name: 'Restart' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Stop' })).toBeInTheDocument();
    for (const name of ['Pause', 'Remove', 'Kill'])
      expect(screen.queryByRole('button', { name })).toBeNull();
    await userEvent.click(screen.getByRole('button', { name: 'More actions' }));
    for (const name of ['Pause', 'Kill', 'Rename', 'Remove'])
      expect(await screen.findByRole('menuitem', { name })).toBeInTheDocument();
  });

  it('offers only Start for a stopped container', async () => {
    const { web } = page('exited');
    renderApp(`/containers/${web.id}/inspect`, clientWithoutRetries());
    expect(await screen.findByRole('button', { name: 'Start' })).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Restart' })).toBeNull();
  });

  it('asks once before removing, naming the Host', async () => {
    const { web, calls } = page('exited');
    renderApp(`/containers/${web.id}/inspect`, clientWithoutRetries());
    await userEvent.click(await screen.findByRole('button', { name: 'More actions' }));
    await userEvent.click(await screen.findByRole('menuitem', { name: 'Remove' }));
    const dialog = await screen.findByRole('alertdialog', { name: 'Remove web from local?' });
    await userEvent.click(within(dialog).getByRole('button', { name: 'Remove' }));
    await waitFor(() => expect(calls).toHaveLength(1));
  });
});
