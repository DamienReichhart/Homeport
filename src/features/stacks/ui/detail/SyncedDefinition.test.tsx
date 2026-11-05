import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Stack, SyncedStack } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const site: Stack = {
  name: 'site',
  kind: 'synced',
  id: 'y1',
  state: 'running',
  services: [],
  running: 1,
  total: 1,
  configFiles: null,
  workingDir: null,
  problem: null,
  updateAvailable: true,
};

const record: SyncedStack = {
  id: 'y1',
  hostId: 'h1',
  name: 'site',
  repositoryId: 'r1',
  refKind: 'branch',
  refName: 'main',
  composePath: 'deploy/compose.yaml',
  hasEnv: true,
  profiles: [],
  deployedCommit: 'aaaaaaa1111',
  fetchedCommit: 'bbbbbbb2222',
  poll: true,
  autoDeploy: false,
  lastPolledAt: new Date().toISOString(),
  lastPollError: null,
  updateAvailable: true,
  autoDeployFailedCommit: null,
  autoDeployError: null,
  createdAt: '',
  updatedAt: 'u1',
};

const repository = {
  id: 'r1',
  url: 'git@github.com:acme/site.git',
  auth: 'sshKey',
  publicKey: 'ssh-ed25519 AAAA homeport-site',
  keyFingerprint: 'SHA256:abc',
  username: null,
  hasToken: false,
  hostKeyFingerprint: null,
  createdAt: '',
  updatedAt: '',
};

function mock(calls: Record<string, unknown>[]) {
  mockCommands({
    ...baseCommands,
    stacks_list: () => [site],
    containers_list: () => [],
    repositories_list: () => [repository],
    stacks_synced_get: () => record,
    stacks_synced_get_env: () => 'TOKEN=hunter2\n',
    stacks_synced_set_sync: (args) => {
      calls.push(args);
      return { ...record, autoDeploy: args.autoDeploy as boolean };
    },
    stacks_synced_files: () => ({
      commit: {
        hash: 'aaaaaaa1111',
        shortHash: 'aaaaaaa',
        subject: 'First',
        author: 'Ada',
        date: '',
      },
      files: [],
    }),
  });
}

describe('a Synced Stack', () => {
  it('shows its source, when it was checked, and that nothing syncs while closed', async () => {
    mock([]);
    renderApp('/stacks/site');
    await userEvent.click(await screen.findByRole('tab', { name: 'Definition' }));
    expect(await screen.findByText('acme/site')).toBeInTheDocument();
    expect(screen.getByText('deploy/compose.yaml')).toBeInTheDocument();
    expect(screen.getByText('bbbbbbb')).toBeInTheDocument();
    expect(screen.getByText(/Last checked/)).toBeInTheDocument();
    expect(screen.getAllByText(/Nothing syncs while it’s closed/).length).toBeGreaterThan(0);
    expect(screen.getAllByText('Update available').length).toBeGreaterThan(0);
  });

  it('turns auto-deploy on, saying it deploys without review', async () => {
    const calls: Record<string, unknown>[] = [];
    mock(calls);
    renderApp('/stacks/site');
    await userEvent.click(await screen.findByRole('tab', { name: 'Definition' }));
    expect(await screen.findByText(/without showing what changed first/)).toBeInTheDocument();
    await userEvent.click(screen.getByRole('switch', { name: 'Deploy new commits automatically' }));
    await waitFor(() => expect(calls).toEqual([{ id: 'y1', poll: true, autoDeploy: true }]));
  });
});
