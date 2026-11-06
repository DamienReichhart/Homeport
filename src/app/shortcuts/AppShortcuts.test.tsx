import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { baseCommands, connectedHost, container, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const commands = {
  ...baseCommands,
  containers_list: () => [container('web', null)],
  usage_subscribe: () => null,
  images_list: () => [],
  get_app_info: () => ({ name: 'homeport', version: '0.13.0', os: 'linux', arch: 'x86_64' }),
};

describe('keyboard shortcuts', () => {
  it('opens pages with Ctrl and their number, and Settings with Ctrl+,', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    expect(await screen.findByRole('heading', { name: 'Containers' })).toBeInTheDocument();
    await user.keyboard('{Control>}4{/Control}');
    expect(await screen.findByRole('heading', { name: 'Images' })).toBeInTheDocument();
    await user.keyboard('{Control>},{/Control}');
    expect(await screen.findByRole('heading', { name: 'Settings' })).toBeInTheDocument();
  });

  it('lists the active shortcuts with ?, the page’s among them', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByRole('heading', { name: 'Containers' });
    await user.keyboard('?');
    const sheet = await screen.findByRole('dialog', { name: 'Keyboard shortcuts' });
    const page = within(sheet).getByRole('region', { name: 'This page' });
    expect(within(page).getByText('Run container')).toBeInTheDocument();
    expect(within(page).getByText('Search this page')).toBeInTheDocument();
    expect(within(sheet).getByText('Go to Images')).toBeInTheDocument();
    expect(within(sheet).getByText('Next host')).toBeInTheDocument();
  });

  it('drops a page’s shortcuts once it’s left', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByRole('heading', { name: 'Containers' });
    await user.keyboard('{Control>},{/Control}');
    await screen.findByRole('heading', { name: 'Settings' });
    await user.keyboard('?');
    const sheet = await screen.findByRole('dialog', { name: 'Keyboard shortcuts' });
    expect(within(sheet).queryByText('Run container')).not.toBeInTheDocument();
  });

  it('starts the page’s action with n and focuses its search with /', async () => {
    mockCommands(commands);
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    await user.keyboard('/');
    const search = screen.getByRole('textbox', { name: 'Search containers' });
    expect(search).toHaveFocus();
    // Typing in the field types, even the shortcuts' keys.
    await user.keyboard('n/');
    expect(search).toHaveValue('n/');
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
    search.blur();
    await user.keyboard('n');
    expect(await screen.findByRole('dialog', { name: 'Run a container' })).toBeInTheDocument();
  });

  it('refreshes the selected host’s lists with Ctrl+R', async () => {
    let listed = 0;
    mockCommands({
      ...commands,
      containers_list: () => {
        listed += 1;
        return [container('web', null)];
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    const before = listed;
    await user.keyboard('{Control>}r{/Control}');
    await waitFor(() => expect(listed).toBeGreaterThan(before));
  });

  it('selects the next and previous host with Alt and the arrows', async () => {
    const saved: Settings[] = [];
    mockCommands({
      ...commands,
      hosts_list: () => [connectedHost, hostView('prod', { id: 'h2' })],
      update_settings: ({ settings }) => {
        saved.push(settings as Settings);
        return settings;
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');
    await screen.findByText('web');
    await user.keyboard('{Alt>}{ArrowDown}{/Alt}');
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h2'));
    // It wraps around.
    await user.keyboard('{Alt>}{ArrowDown}{/Alt}');
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h1'));
    await user.keyboard('{Alt>}{ArrowUp}{/Alt}');
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h2'));
  });
});
