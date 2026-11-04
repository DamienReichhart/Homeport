import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const appInfo = () => ({ name: 'homeport', version: '0.7.0', os: 'linux', arch: 'x86_64' });
const ghcr = {
  id: 'r1',
  address: 'ghcr.io',
  username: 'alice',
  createdAt: '',
  updatedAt: '',
};

describe('registries card', () => {
  it('lists the stored logins without their secrets', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      registries_list: () => [ghcr, { ...ghcr, id: 'r2', address: 'docker.io', username: 'bob' }],
    });
    renderApp('/settings');
    expect(await screen.findByText('ghcr.io')).toBeInTheDocument();
    expect(screen.getByText('Docker Hub (docker.io)')).toBeInTheDocument();
    expect(screen.getByText('bob')).toBeInTheDocument();
  });

  it('checks a new login, then saves it', async () => {
    const calls: { command: string; args: Record<string, unknown> }[] = [];
    let stored: (typeof ghcr)[] = [];
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      registries_list: () => stored,
      registries_check: (args) => {
        calls.push({ command: 'check', args });
        return null;
      },
      registries_save: (args) => {
        calls.push({ command: 'save', args });
        stored = [ghcr];
        return ghcr;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Add login' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Registry'), 'ghcr.io');
    await user.type(within(dialog).getByLabelText('User name'), 'alice');
    await user.type(within(dialog).getByLabelText('Password or access token'), 'ghp_x');
    await user.click(within(dialog).getByRole('button', { name: 'Check login' }));
    expect(await within(dialog).findByText(/accepted this login/)).toBeInTheDocument();

    await user.click(within(dialog).getByRole('button', { name: 'Save' }));
    expect(await screen.findByText('alice')).toBeInTheDocument();
    expect(calls).toEqual([
      {
        command: 'check',
        args: { id: null, draft: { address: 'ghcr.io', username: 'alice', secret: 'ghp_x' } },
      },
      {
        command: 'save',
        args: { id: null, draft: { address: 'ghcr.io', username: 'alice', secret: 'ghp_x' } },
      },
    ]);
  });

  it('shows a refused login in place, and edits keep the stored secret', async () => {
    let saved: unknown = null;
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      registries_list: () => [ghcr],
      registries_check: () => {
        throw {
          code: 'registry_login_failed',
          message: 'ghcr.io refused this user name and password.',
          hint: { text: 'Check them.', commands: [] },
          detail: null,
        };
      },
      registries_save: (args) => {
        saved = args;
        return ghcr;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Edit the login for ghcr.io' }));
    const dialog = await screen.findByRole('dialog');
    await user.click(within(dialog).getByRole('button', { name: 'Check login' }));
    expect(
      await within(dialog).findByText('ghcr.io refused this user name and password.'),
    ).toBeInTheDocument();

    await user.click(within(dialog).getByRole('button', { name: 'Save' }));
    await screen.findByText('Saved the login for ghcr.io');
    expect(saved).toEqual({
      id: 'r1',
      draft: { address: 'ghcr.io', username: 'alice', secret: null },
    });
  });

  it('explains that logins need the OS keychain', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_capabilities: () => ({ ...baseCommands.get_capabilities(), keychain: false }),
      registries_list: () => [],
    });
    renderApp('/settings');
    expect(
      await screen.findByText(/Registry passwords and tokens are encrypted/),
    ).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Add login' })).not.toBeInTheDocument();
  });
});
