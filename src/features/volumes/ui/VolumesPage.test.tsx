import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

function volume(name: string, driver = 'local') {
  const createdAt = '2026-01-02T03:04:05Z';
  return { name, driver, mountpoint: '/x', createdAt, labels: {}, stack: null, usedBy: [] };
}

describe('volumes page', () => {
  it('fills in sizes after the list, shows unknown ones as —, and sorts by size', async () => {
    let measure: (sizes: Record<string, number | null>) => void = () => undefined;
    let asked = 0;
    mockCommands({
      ...baseCommands,
      volumes_list: () => [volume('db'), volume('cache'), volume('remote', 'rclone')],
      volumes_sizes: () => {
        asked += 1;
        return new Promise((resolve) => (measure = resolve));
      },
    });
    const user = userEvent.setup();
    renderApp('/volumes');

    // The list doesn't wait for the sizes.
    expect(await screen.findByText('remote')).toBeInTheDocument();
    expect(screen.getAllByLabelText('Measuring')).toHaveLength(3);

    measure({ db: 3_000_000, cache: 2048, remote: null });
    expect(await screen.findByText('2.9 MB')).toBeInTheDocument();
    expect(screen.getByText('2.0 KB')).toBeInTheDocument();
    expect(screen.getByText('—')).toBeInTheDocument();

    await user.click(screen.getByRole('columnheader', { name: 'Size' }));
    const names = () =>
      screen
        .getAllByRole('row')
        .slice(1)
        .map((r) => within(r).getAllByRole('cell')[0]?.textContent);
    // Largest first, unknown last.
    await waitFor(() => expect(names()).toEqual(['db', 'cache', 'remote']));

    await user.click(screen.getByRole('button', { name: 'Refresh sizes' }));
    await waitFor(() => expect(asked).toBe(2));
  });

  it('warns that deleting a volume destroys its data', async () => {
    const removed: string[] = [];
    mockCommands({
      ...baseCommands,
      volumes_list: () => [
        {
          name: 'shop_db',
          driver: 'local',
          mountpoint: '/x',
          createdAt: null,
          labels: {},
          stack: 'shop',
          usedBy: ['shop-db-1'],
        },
        {
          name: 'scratch',
          driver: 'local',
          mountpoint: '/y',
          createdAt: null,
          labels: {},
          stack: null,
          usedBy: [],
        },
      ],
      volumes_remove: ({ name }) => {
        removed.push(String(name));
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/volumes');
    expect(await screen.findByText('shop-db-1')).toBeInTheDocument();
    expect(screen.getByText('2 volumes · 1 unused')).toBeInTheDocument();

    await user.click(screen.getByLabelText('Delete scratch'));
    expect(await screen.findByText(/permanently deleted/)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Delete volume and data' }));
    await vi.waitFor(() => expect(removed).toEqual(['scratch']));
  });

  it('keeps the confirmation open when the volume is in use, and reports it once', async () => {
    let attempts = 0;
    mockCommands({
      ...baseCommands,
      volumes_list: () => [
        {
          name: 'scratch',
          driver: 'local',
          mountpoint: '/y',
          createdAt: null,
          labels: {},
          stack: null,
          usedBy: [],
        },
      ],
      volumes_remove: () => {
        attempts += 1;
        throw {
          code: 'in_use',
          message: 'scratch is used by a container.',
          hint: { text: 'Remove the container first.', commands: [] },
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/volumes');
    await user.click(await screen.findByLabelText('Delete scratch'));
    const confirmation = await screen.findByRole('alertdialog');
    const remove = within(confirmation).getByRole('button', { name: 'Delete volume and data' });
    await user.click(remove);

    expect(await screen.findByText('scratch is used by a container.')).toBeInTheDocument();
    expect(screen.getAllByText('scratch is used by a container.')).toHaveLength(1);
    await waitFor(() => expect(remove).toBeEnabled());
    expect(attempts).toBe(1);

    await user.click(within(confirmation).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
  });
});
