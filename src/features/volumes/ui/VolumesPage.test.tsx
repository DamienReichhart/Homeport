import { screen } from '@testing-library/react';
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
});
