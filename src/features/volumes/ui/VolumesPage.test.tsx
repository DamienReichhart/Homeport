import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('volumes page', () => {
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
