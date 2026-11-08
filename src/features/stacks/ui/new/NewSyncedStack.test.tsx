import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const repository = {
  id: 'r1',
  url: 'git@github.com:acme/shop.git',
  auth: 'sshKey',
  publicKey: 'ssh-ed25519 AAAA homeport-shop',
  keyFingerprint: 'SHA256:abc',
  username: null,
  hasToken: false,
  hostKeyFingerprint: 'SHA256:srv',
  createdAt: '',
  updatedAt: '',
};

describe('creating a Synced Stack', () => {
  it('picks a repository, a branch from its server and a compose file', async () => {
    const created: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      stacks_list: () => [],
      repositories_list: () => [repository],
      repositories_test: () => ({
        branches: ['dev', 'main'],
        tags: ['v1'],
        defaultBranch: 'main',
      }),
      stacks_synced_create: (args) => {
        created.push(args);
        return { id: 's1', name: 'shop' };
      },
      containers_list: () => [],
    });
    const user = userEvent.setup();
    renderApp('/stacks/new');
    await user.click(await screen.findByRole('radio', { name: 'Use a git repository' }));
    expect(screen.getByText(/Nothing syncs while it’s closed/)).toBeInTheDocument();
    await user.click(screen.getByRole('combobox', { name: 'Repository' }));
    await user.click(await screen.findByRole('option', { name: 'acme/shop' }));
    expect(screen.getByLabelText('Name')).toHaveValue('shop');
    expect(await screen.findByRole('combobox', { name: 'Branch or tag' })).toHaveTextContent(
      'main',
    );
    await user.clear(screen.getByLabelText('Compose file'));
    await user.type(screen.getByLabelText('Compose file'), 'deploy/compose.yaml');
    await user.click(screen.getByRole('button', { name: 'Save' }));
    await waitFor(() => expect(created).toHaveLength(1));
    expect(created[0]).toEqual({
      owner: { kind: 'host', id: 'h1' },
      draft: {
        name: 'shop',
        repositoryId: 'r1',
        refKind: 'branch',
        refName: 'main',
        composePath: 'deploy/compose.yaml',
      },
      env: '',
    });
  });

  it('says how to install git when it is missing', async () => {
    mockCommands({
      ...baseCommands,
      get_capabilities: () => ({
        keychain: true,
        stackDeploy: {
          state: 'available',
          cli: { program: 'docker', prefix: [], version: '29.1.0' },
        },
        compose: {
          state: 'available',
          cli: { program: 'docker', prefix: ['compose'], version: '5' },
        },
        git: {
          state: 'unavailable',
          problem: {
            code: 'git_missing',
            message: 'git isn’t installed on this computer. Synced stacks need it.',
            hint: {
              text: 'Install git. On Debian or Ubuntu:',
              commands: ['sudo apt-get install git'],
            },
            detail: null,
          },
        },
      }),
      stacks_list: () => [],
    });
    renderApp('/stacks/new');
    await userEvent.click(await screen.findByRole('radio', { name: 'Use a git repository' }));
    expect(await screen.findByText('sudo apt-get install git')).toBeInTheDocument();
  });
});
