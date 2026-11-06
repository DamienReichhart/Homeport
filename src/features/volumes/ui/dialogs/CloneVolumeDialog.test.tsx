import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const volume = (name: string, stack: string | null = null) => ({
  name,
  driver: 'local',
  mountpoint: '/x',
  createdAt: '2026-01-02T03:04:05Z',
  labels: {},
  stack,
  usedBy: [],
});

describe('cloning a volume', () => {
  it('copies into <name>-copy and selects the new volume', async () => {
    const volumes = [volume('shop_db', 'shop')];
    const clones: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      volumes_list: () => [...volumes],
      volumes_sizes: () => ({}),
      volumes_running_users: () => [],
      volumes_helper_image: () => ({ image: 'busybox:1.37', present: true, size: 2_200_000 }),
      volumes_clone: (args) => {
        clones.push(args);
        volumes.push(volume(String(args.target)));
        return args.target;
      },
    });
    const user = userEvent.setup();
    renderApp('/volumes');

    await user.click(await screen.findByRole('button', { name: 'More actions for shop_db' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Clone…' }));
    expect(screen.getByLabelText('Name of the copy')).toHaveValue('shop_db-copy');
    await user.click(screen.getByRole('button', { name: 'Clone' }));

    await waitFor(() =>
      expect(clones).toEqual([
        { hostId: 'h1', source: 'shop_db', target: 'shop_db-copy', pulledImage: false },
      ]),
    );
    const copy = await screen.findByText('shop_db-copy');
    await waitFor(() => expect(copy.closest('tr')).toHaveAttribute('data-state', 'selected'));
  });

  it('shows why a copy failed, in place', async () => {
    mockCommands({
      ...baseCommands,
      volumes_list: () => [volume('db')],
      volumes_sizes: () => ({}),
      volumes_running_users: () => [],
      volumes_helper_image: () => ({ image: 'busybox:1.37', present: true, size: 2_200_000 }),
      volumes_clone: () => {
        throw {
          code: 'engine_error',
          message: "Homeport couldn't copy db.",
          hint: { text: 'The incomplete copy, db-copy, was removed.', commands: [] },
          detail: 'cp: write error: No space left on device',
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/volumes');
    await user.click(await screen.findByRole('button', { name: 'More actions for db' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Clone…' }));
    await user.click(screen.getByRole('button', { name: 'Clone' }));
    expect(await screen.findByText("Homeport couldn't copy db.")).toBeInTheDocument();
    expect(screen.getByText('The incomplete copy, db-copy, was removed.')).toBeInTheDocument();
  });
});
