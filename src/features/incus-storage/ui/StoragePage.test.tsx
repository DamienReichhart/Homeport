import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusProject } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

type Call = { cmd: string; args: Record<string, unknown> };
const user = (kind: string, name: string) => ({ kind, name, project: null });

function setup(calls: Call[] = []) {
  const record = (cmd: string) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return null;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [incusHost],
    incus_projects: () => [incusProject('default')],
    incus_storage_pools: () => [
      {
        name: 'default',
        description: '',
        driver: 'zfs',
        status: 'Created',
        used: 25 * 1024 ** 3,
        total: 100 * 1024 ** 3,
        usedBy: [user('instance', 't1')],
      },
    ],
    incus_storage_volumes: () => [
      {
        name: 't1',
        type: 'container',
        contentType: 'filesystem',
        description: '',
        size: null,
        createdAt: null,
        usedBy: [user('instance', 't1')],
      },
      {
        name: 'data',
        type: 'custom',
        contentType: 'filesystem',
        description: '',
        size: '10GiB',
        createdAt: null,
        usedBy: [user('instance', 'web')],
      },
      {
        name: 'spare',
        type: 'custom',
        contentType: 'block',
        description: '',
        size: null,
        createdAt: null,
        usedBy: [],
      },
    ],
    incus_volume_create: record('create'),
    incus_volume_delete: record('delete'),
  });
  return userEvent.setup();
}

describe('incus storage', () => {
  it('show how full each pool is, and delete only unused custom volumes', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/storage');
    expect(await screen.findByText('25.0 GB of 100.0 GB used (25.0%)')).toBeInTheDocument();
    expect(await screen.findByText('spare')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Delete data' })).toBeDisabled();
    // Instances' own volumes aren't deleted here.
    expect(screen.queryByRole('button', { name: 'Delete t1' })).toBeNull();
    await user.click(screen.getByRole('button', { name: 'Delete spare' }));
    await user.click(await screen.findByRole('button', { name: 'Delete' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'delete',
        args: { hostId: 'i1', project: 'default', pool: 'default', name: 'spare' },
      }),
    );
  });

  it('make a custom volume', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/storage/default');
    await user.click(await screen.findByRole('button', { name: 'New volume' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Name'), 'backups');
    await user.type(within(dialog).getByLabelText('Size'), '20GiB');
    await user.click(within(dialog).getByRole('radio', { name: 'Block device' }));
    await user.click(within(dialog).getByRole('button', { name: 'Create' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'create',
        args: {
          hostId: 'i1',
          project: 'default',
          pool: 'default',
          request: { name: 'backups', size: '20GiB', contentType: 'block' },
        },
      }),
    );
  });
});
