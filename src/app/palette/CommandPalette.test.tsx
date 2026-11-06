import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { baseCommands, connectedHost, container, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const commands = {
  ...baseCommands,
  containers_list: () => [container('web', null)],
  volumes_list: () => [],
  volumes_sizes: () => [],
  usage_subscribe: () => null,
  get_app_info: () => ({ name: 'homeport', version: '0.13.0', os: 'linux', arch: 'x86_64' }),
};

const palette = () => screen.findByRole('dialog', { name: 'Command palette' });

describe('command palette', () => {
  it('opens with Ctrl+K, and Esc closes it and gives focus back', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    const search = screen.getByRole('textbox', { name: 'Search containers' });
    search.focus();
    await user.keyboard('{Control>}k{/Control}');
    expect(await palette()).toBeInTheDocument();
    expect(screen.getByRole('combobox')).toHaveFocus();
    await user.keyboard('{Escape}');
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
    await waitFor(() => expect(search).toHaveFocus());
  });

  it('opens a page from what is typed, with Enter', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    await user.keyboard('{Control>}k{/Control}');
    await palette();
    await user.keyboard('vol');
    await user.keyboard('{Enter}');
    expect(await screen.findByRole('heading', { name: 'Volumes' })).toBeInTheDocument();
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('switches host by name', async () => {
    const saved: Settings[] = [];
    mockCommands({
      ...commands,
      hosts_list: () => [connectedHost, hostView('prod-server', { id: 'h2' })],
      update_settings: ({ settings }) => {
        saved.push(settings as Settings);
        return settings;
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    await user.keyboard('{Control>}k{/Control}');
    await palette();
    await user.keyboard('prod{Enter}');
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h2'));
  });

  it('runs a feature’s command, and lists it first next time', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/settings');
    await screen.findByRole('heading', { name: 'Settings' });
    await user.keyboard('{Control>}k{/Control}');
    await palette();
    await user.keyboard('run a con{Enter}');
    expect(await screen.findByRole('dialog', { name: 'Run a container' })).toBeInTheDocument();
    await user.keyboard('{Escape}');
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
    await user.keyboard('{Control>}k{/Control}');
    const recent = within(await palette()).getByRole('group', { name: 'Recent' });
    expect(within(recent).getByRole('option', { name: 'Run a container' })).toBeInTheDocument();
  });

  it('shows each command’s shortcut', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    await user.keyboard('{Control>}k{/Control}');
    const dialog = await palette();
    const goTo = within(dialog).getByRole('option', { name: /Go to Images/ });
    expect(within(goTo).getByText('Ctrl')).toBeInTheDocument();
    expect(within(goTo).getByText('4')).toBeInTheDocument();
  });
});
