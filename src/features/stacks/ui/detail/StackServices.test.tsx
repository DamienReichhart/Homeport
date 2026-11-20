import { screen, within } from '@testing-library/react';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const api1 = container('shop-api-1', 'shop');
const api2 = container('shop-api-2', 'shop', 'exited');

describe('a Stack’s services', () => {
  it('lists each container under its service, with its state and a link to its page', async () => {
    mockCommands({
      ...baseCommands,
      containers_list: () => [api1, api2],
      images_list: () => [],
      networks_list: () => [],
      usage_subscribe: () => 1,
      logs_subscribe: () => 2,
      images_freshness: () => [],
      stacks_list: () => [
        {
          name: 'shop',
          kind: 'managed',
          id: 'm1',
          state: 'partial',
          services: [
            { name: 'api', containerIds: [api1.id, api2.id], running: 1 },
            { name: 'jobs', containerIds: [], running: 0 },
          ],
          running: 1,
          total: 2,
          configFiles: null,
          workingDir: null,
          problem: null,
          updateAvailable: false,
        },
      ],
    });
    renderApp('/stacks/shop');
    const link = await screen.findByRole('link', { name: 'shop-api-2' });
    expect(link).toHaveAttribute('href', `/containers/${api2.id}/logs`);
    // The service is named once, on its first container's row.
    expect(screen.getAllByText('api')).toHaveLength(1);
    const exited = link.closest('tr');
    expect(exited && within(exited).getByText('Exited (0)')).toBeInTheDocument();
    expect(screen.getByText('No container')).toBeInTheDocument();
  });
});
