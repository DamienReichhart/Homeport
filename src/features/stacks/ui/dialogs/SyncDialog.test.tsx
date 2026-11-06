import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Stack, SyncReview, SyncedStack } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
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
  updateAvailable: false,
};

const record: SyncedStack = {
  id: 'y1',
  hostId: 'h1',
  name: 'site',
  repositoryId: 'r1',
  refKind: 'branch',
  refName: 'main',
  composePath: 'compose.yaml',
  hasEnv: false,
  profiles: [],
  deployedCommit: 'aaaaaaa1111',
  fetchedCommit: 'bbbbbbb2222',
  poll: true,
  autoDeploy: false,
  lastPolledAt: null,
  lastPollError: null,
  updateAvailable: true,
  autoDeployFailedCommit: null,
  autoDeployError: null,
  createdAt: '',
  updatedAt: '',
};

const commit = (hash: string, subject: string) => ({
  hash,
  shortHash: hash.slice(0, 7),
  subject,
  author: 'Ada',
  date: '2026-11-05T10:00:00Z',
});

function review(overrides: Partial<SyncReview> = {}): SyncReview {
  return {
    stack: record,
    deployed: commit('aaaaaaa1111', 'First'),
    fetched: commit('bbbbbbb2222', 'Bump nginx'),
    commits: [commit('bbbbbbb2222', 'Bump nginx')],
    files: [
      {
        path: 'compose.yaml',
        before: 'services:\n  web:\n    image: nginx:1.27\n',
        after: 'services:\n  web:\n    image: nginx:1.28\n',
        binary: false,
      },
    ],
    upToDate: false,
    tagMoved: false,
    builds: false,
    ...overrides,
  };
}

const capabilities = () => ({
  keychain: true,
  git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
  compose: { state: 'available', cli: { program: 'docker', prefix: ['compose'], version: '5' } },
});

describe('syncing a Synced Stack', () => {
  it('shows the new commit and its change, then deploys that commit', async () => {
    const deployed: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      get_capabilities: capabilities,
      stacks_list: () => [site],
      containers_list: () => [],
      stacks_synced_sync: () => review(),
      stacks_synced_deploy_subscribe: (args) => {
        deployed.push(args);
        setTimeout(() =>
          sendToChannel(args.onEvent, { type: 'exit', success: true, code: 0, error: null }),
        );
        return 3;
      },
    });
    const user = userEvent.setup();
    renderApp('/stacks');
    await user.click(await screen.findByRole('button', { name: 'Sync' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText('Bump nginx')).toBeInTheDocument();
    expect(within(dialog).getByText('Deploying bbbbbbb in place of aaaaaaa.')).toBeInTheDocument();
    expect(within(dialog).getByText(/Nothing syncs while it’s closed/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Deploy bbbbbbb' }));
    await waitFor(() => expect(deployed).toHaveLength(1));
    expect(deployed[0]).toMatchObject({ id: 'y1', commit: 'bbbbbbb2222', action: 'up' });
  });

  it('says when nothing changed, and still offers to redeploy', async () => {
    mockCommands({
      ...baseCommands,
      get_capabilities: capabilities,
      stacks_list: () => [site],
      containers_list: () => [],
      stacks_synced_sync: () => review({ upToDate: true, commits: [], files: [] }),
    });
    const user = userEvent.setup();
    renderApp('/stacks');
    await user.click(await screen.findByRole('button', { name: 'Sync' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText(/Up to date/)).toBeInTheDocument();
    expect(within(dialog).getByRole('button', { name: 'Redeploy anyway' })).toBeInTheDocument();
  });

  it('shows a failed fetch with its hint and deploys nothing', async () => {
    mockCommands({
      ...baseCommands,
      get_capabilities: capabilities,
      stacks_list: () => [site],
      containers_list: () => [],
      stacks_synced_sync: () => {
        throw {
          code: 'git_auth_failed',
          message: 'github.com refused Homeport’s deploy key for this repository.',
          hint: {
            text: 'Add the deploy key to the repository with read access, then test again.',
            commands: [],
          },
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/stacks');
    await user.click(await screen.findByRole('button', { name: 'Sync' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText(/refused Homeport’s deploy key/)).toBeInTheDocument();
    expect(within(dialog).getByText(/Add the deploy key/)).toBeInTheDocument();
    expect(within(dialog).queryByRole('button', { name: /Deploy/ })).not.toBeInTheDocument();
  });
});
