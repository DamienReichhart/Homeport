import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { BackupVolumeDialog } from './BackupVolumeDialog';
import { RestoreVolumeDialog } from './RestoreVolumeDialog';

const volume = (name: string) => ({
  name,
  driver: 'local',
  mountpoint: '/x',
  createdAt: null,
  labels: {},
  stack: null,
  usedBy: [],
});

function engine(users: string[]) {
  const calls = {
    acts: [] as string[],
    backups: [] as Record<string, unknown>[],
    restores: [] as Record<string, unknown>[],
  };
  mockCommands({
    ...baseCommands,
    volumes_running_users: () => users,
    volumes_helper_image: () => ({ image: 'busybox:1.37', present: true, size: 2_200_000 }),
    containers_act: ({ id, action }) => {
      calls.acts.push(`${String(action)} ${String(id)}`);
      return null;
    },
    volumes_backup_subscribe: (args) => {
      calls.backups.push(args);
      setTimeout(() =>
        sendToChannel(
          args.onEvent,
          { type: 'progress', bytes: 10, total: 10 },
          { type: 'done', images: [] },
        ),
      );
      return 1;
    },
    volumes_restore_subscribe: (args) => {
      calls.restores.push(args);
      const { replace } = args.spec as { replace: boolean };
      setTimeout(() =>
        sendToChannel(
          args.onEvent,
          replace
            ? { type: 'done', images: [] }
            : {
                type: 'error',
                error: {
                  code: 'volume_not_empty',
                  message: 'db already holds files.',
                  hint: null,
                  detail: null,
                },
              },
        ),
      );
      return 2;
    },
  });
  return calls;
}

describe('backing up and restoring a volume', () => {
  it('stops the running users during a backup, and starts them again', async () => {
    const calls = engine(['db-1']);
    const user = userEvent.setup();
    const onClose = vi.fn();
    renderWithProviders(
      <BackupVolumeDialog
        hostId="h1"
        volume="db"
        destination="/home/me/db.tar"
        onClose={onClose}
      />,
    );

    expect(await screen.findByText(/db-1 is running and uses db/)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Stop it during the backup' }));
    expect(await screen.findByText('Done.')).toBeInTheDocument();
    expect(calls.acts).toEqual(['stop db-1']);
    expect(calls.backups[0]).toMatchObject({
      volume: 'db',
      destination: '/home/me/db.tar',
      pulledImage: false,
    });

    // The footer's Close, not the dialog's corner button.
    const close = screen.getAllByRole('button', { name: 'Close' });
    await user.click(close[close.length - 1]!);
    await waitFor(() => expect(calls.acts).toEqual(['stop db-1', 'start db-1']));
    expect(onClose).toHaveBeenCalled();
  });

  it('backs up while they run when asked to, leaving them alone', async () => {
    const calls = engine(['db-1']);
    const user = userEvent.setup();
    renderWithProviders(
      <BackupVolumeDialog hostId="h1" volume="db" destination="/tmp/db.tar" onClose={vi.fn()} />,
    );
    await user.click(await screen.findByRole('button', { name: 'Back up anyway' }));
    expect(await screen.findByText('Done.')).toBeInTheDocument();
    expect(calls.acts).toEqual([]);
  });

  it('names a new volume after the file, and refuses a taken name', async () => {
    const calls = engine([]);
    const user = userEvent.setup();
    renderWithProviders(
      <RestoreVolumeDialog
        hostId="h1"
        source="/home/me/db.tar"
        volumes={[volume('db')]}
        onClose={vi.fn()}
      />,
    );
    const name = screen.getByLabelText('Name');
    expect(name).toHaveValue('db');
    expect(screen.getByText('A volume named db already exists.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Restore' })).toBeDisabled();

    await user.clear(name);
    await user.type(name, 'db-copy');
    await user.click(screen.getByRole('button', { name: 'Restore' }));
    await waitFor(() => expect(calls.restores).toHaveLength(1));
    expect(calls.restores[0]).toMatchObject({
      source: '/home/me/db.tar',
      spec: { volume: 'db-copy', create: true, replace: false },
    });
  });

  it('asks before replacing what an existing volume holds', async () => {
    const calls = engine([]);
    const user = userEvent.setup();
    renderWithProviders(
      <RestoreVolumeDialog
        hostId="h1"
        source="/home/me/db.tar"
        volumes={[volume('db')]}
        existing="db"
        onClose={vi.fn()}
      />,
    );
    await user.click(screen.getByRole('button', { name: 'Restore' }));
    expect(
      await screen.findByText(/db already holds files. Restoring deletes them/),
    ).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Replace its contents' }));
    expect(await screen.findByText('Done.')).toBeInTheDocument();
    expect(calls.restores.map((r) => (r.spec as { replace: boolean }).replace)).toEqual([
      false,
      true,
    ]);
  });
});
