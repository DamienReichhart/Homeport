import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { incusInstance } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { SnapshotsTab } from './SnapshotsTab';

type Call = { cmd: string; args: Record<string, unknown> };

function setup(calls: Call[]) {
  const record = (cmd: string) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return null;
  };
  mockCommands({
    ...baseCommands,
    incus_snapshots: () => [
      {
        name: 'before-upgrade',
        createdAt: '2026-11-08T10:00:00Z',
        expiresAt: null,
        stateful: false,
        size: null,
      },
    ],
    incus_snapshot_create: record('create'),
    incus_snapshot_restore: record('restore'),
    incus_snapshot_delete: record('delete'),
  });
  return userEvent.setup();
}

describe('an instance’s snapshots', () => {
  it('take one, stateful only when the instance runs and allows it', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderWithProviders(
      <SnapshotsTab
        hostId="i1"
        project="default"
        instance={incusInstance('web')}
        statefulAllowed
      />,
    );
    await user.click(await screen.findByRole('button', { name: 'Take snapshot' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Name'), 'snap-test');
    await user.click(within(dialog).getByLabelText('Include its running state (memory)'));
    await user.click(within(dialog).getByRole('button', { name: 'Take snapshot' }));
    await waitFor(() =>
      expect(calls).toEqual([
        {
          cmd: 'create',
          args: {
            hostId: 'i1',
            project: 'default',
            instance: 'web',
            request: { name: 'snap-test', stateful: true, expiresAt: null },
          },
        },
      ]),
    );
  });

  it('restore and delete only once confirmed, saying what’s lost', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderWithProviders(
      <SnapshotsTab
        hostId="i1"
        project="default"
        instance={incusInstance('web', { status: 'stopped' })}
        statefulAllowed={false}
      />,
    );
    await user.click(await screen.findByRole('button', { name: 'Restore' }));
    const confirm = await screen.findByRole('alertdialog');
    expect(within(confirm).getByText(/is lost/)).toBeInTheDocument();
    expect(calls).toEqual([]);
    await user.click(within(confirm).getByRole('button', { name: 'Restore' }));
    await user.click(screen.getByRole('button', { name: 'Delete before-upgrade' }));
    await user.click(await screen.findByRole('button', { name: 'Delete' }));
    await waitFor(() => expect(calls.map((c) => c.cmd)).toEqual(['restore', 'delete']));
    expect(calls[0]!.args).toMatchObject({ snapshot: 'before-upgrade', stateful: false });
  });
});
