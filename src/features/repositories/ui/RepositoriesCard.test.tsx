import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Repository } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const appInfo = () => ({ name: 'homeport', version: '0.11.0', os: 'linux', arch: 'x86_64' });
const shop: Repository = {
  id: 'r1',
  url: 'git@github.com:acme/shop.git',
  auth: 'sshKey',
  publicKey:
    'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl homeport-shop',
  keyFingerprint: 'SHA256:abc',
  username: null,
  hasToken: false,
  hostKeyFingerprint: null,
  createdAt: '',
  updatedAt: '',
};

describe('repositories card', () => {
  it('adds a repository with a deploy key, shows the key, then tests access', async () => {
    const created: unknown[] = [];
    let stored: Repository[] = [];
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      repositories_list: () => stored,
      repositories_create: ({ draft }) => {
        created.push(draft);
        stored = [shop];
        return shop;
      },
      repositories_test: () => {
        stored = [{ ...shop, hostKeyFingerprint: 'SHA256:server' }];
        return { branches: ['main', 'dev'], tags: ['v1'], defaultBranch: 'main' };
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Add repository' }));
    const dialog = await screen.findByRole('dialog');
    const add = within(dialog).getByRole('button', { name: 'Add and make a deploy key' });
    expect(add).toBeDisabled();

    await user.type(
      within(dialog).getByLabelText('Repository URL'),
      'https://github.com/acme/shop',
    );
    expect(within(dialog).getByRole('radio', { name: 'Deploy key' })).toBeDisabled();
    await user.clear(within(dialog).getByLabelText('Repository URL'));
    await user.type(within(dialog).getByLabelText('Repository URL'), shop.url);
    await user.click(add);

    expect(await within(dialog).findByText(shop.publicKey ?? '')).toBeInTheDocument();
    expect(within(dialog).getByText(/GitLab: the project’s/)).toBeInTheDocument();
    expect(created).toEqual([{ url: shop.url, auth: 'sshKey', username: null, token: null }]);

    await user.click(within(dialog).getByRole('button', { name: 'Test access' }));
    expect(await within(dialog).findByText(/2 branches and 1 tag/)).toBeInTheDocument();
    expect(await within(dialog).findByText('SHA256:server')).toBeInTheDocument();
  });

  it('asks for a user name and token over HTTPS', async () => {
    mockCommands({ ...baseCommands, get_app_info: appInfo });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Add repository' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(
      within(dialog).getByLabelText('Repository URL'),
      'https://gitlab.com/acme/shop.git',
    );
    expect(within(dialog).getByRole('radio', { name: 'Token' })).toBeChecked();
    expect(within(dialog).getByRole('button', { name: 'Add' })).toBeDisabled();
    await user.type(within(dialog).getByLabelText('User name'), 'bot');
    await user.type(within(dialog).getByLabelText('Access token'), 'glpat-x');
    expect(within(dialog).getByRole('button', { name: 'Add' })).toBeEnabled();
  });
});
