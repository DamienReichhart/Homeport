import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Stack, SyncedStack } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

// CodeMirror doesn't run in jsdom: the editor is a text area that keeps the props.
vi.mock('../YamlEditor', () => ({
  YamlEditor: ({
    value,
    label,
    readOnly = false,
    onChange,
  }: {
    value: string;
    label: string;
    readOnly?: boolean;
    onChange?: (value: string) => void;
  }) => (
    <textarea
      aria-label={label}
      value={value}
      readOnly={readOnly}
      onChange={(e) => onChange?.(e.target.value)}
    />
  ),
}));

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
  registryAuth: false,
};

const record: SyncedStack = {
  id: 'y1',
  owner: { kind: 'host', id: 'h1' },
  registryAuth: false,
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
    stacks_list: () => [
      site,
      { ...site, name: 'shop', kind: 'managed', id: 'm1', updateAvailable: false },
    ],
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
        hash: 'bbbbbbb2222',
        shortHash: 'bbbbbbb',
        subject: 'Bump',
        author: 'Ada',
        date: '',
      },
      files: [
        { path: 'deploy/compose.yaml', content: 'services:\n  web:\n    image: nginx:1.28\n' },
        { path: 'deploy/Dockerfile', content: 'FROM nginx\n' },
      ],
    }),
    stacks_get: () => ({
      id: 'm1',
      owner: { kind: 'host', id: 'h1' },
      registryAuth: false,
      name: 'shop',
      files: [{ name: 'compose.yaml', content: 'services: {}' }],
      hasEnv: false,
      profiles: [],
      createdAt: '',
      updatedAt: 'u',
    }),
    stacks_get_env: () => '',
    stacks_check: () => ({ errors: [], warnings: [], profiles: [] }),
    stacks_history: () => [],
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

  it('shows its compose file read-only, unlike a Managed Stack’s, and only what Homeport owns is editable', async () => {
    mock([]);
    renderApp('/stacks/site');
    await userEvent.click(await screen.findByRole('tab', { name: 'Definition' }));
    const editor = await screen.findByLabelText('deploy/compose.yaml, read-only');
    expect(editor).toHaveAttribute('readonly');
    await userEvent.type(editor, 'x');
    expect(editor).toHaveValue('services:\n  web:\n    image: nginx:1.28\n');
    expect(screen.getByText('deploy/Dockerfile')).toBeInTheDocument();
    expect(screen.getByText(/Edit it in git/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Open on the forge' })).toBeInTheDocument();
    for (const name of ['Save and redeploy', 'Forget definition', 'Adopt']) {
      expect(screen.queryByRole('button', { name })).not.toBeInTheDocument();
    }
    expect(screen.getByLabelText('Environment')).toHaveValue('TOKEN=hunter2\n');
    expect(screen.getByLabelText('Environment')).toBeEnabled();
  });

  it('keeps a Managed Stack’s compose file editable', async () => {
    mock([]);
    renderApp('/stacks/shop');
    await userEvent.click(await screen.findByRole('tab', { name: 'Definition' }));
    await screen.findByRole('button', { name: 'Save and redeploy' });
    const editor = screen.getByDisplayValue('services: {}');
    expect(editor).not.toHaveAttribute('readonly');
  });
});
