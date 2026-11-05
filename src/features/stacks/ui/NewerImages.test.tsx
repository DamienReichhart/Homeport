import { screen } from '@testing-library/react';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web = container('shop-web-1', 'shop');
const db = container('shop-db-1', 'shop');

function mock() {
  mockCommands({
    ...baseCommands,
    containers_list: () => [web, db],
    images_list: () => [],
    networks_list: () => [],
    usage_subscribe: () => 1,
    logs_subscribe: () => 2,
    stacks_list: () => [
      {
        name: 'shop',
        kind: 'managed',
        id: 'm1',
        state: 'running',
        services: [
          { name: 'web', containerIds: [web.id], running: 1 },
          { name: 'db', containerIds: [db.id], running: 1 },
        ],
        running: 2,
        total: 2,
        configFiles: null,
        workingDir: null,
        problem: null,
      },
    ],
    images_freshness: () => [
      { reference: web.image, imageId: web.imageId, freshness: 'newer' },
      { reference: db.image, imageId: db.imageId, freshness: 'upToDate' },
    ],
  });
}

describe('newer images on Stacks', () => {
  it('marks a Stack with an outdated service on the list', async () => {
    mock();
    renderApp('/stacks');
    expect(await screen.findByRole('img', { name: 'Newer images for web' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Check for newer images' })).toBeInTheDocument();
  });

  it('names the outdated service on the detail page, with how to update', async () => {
    mock();
    renderApp('/stacks/shop');
    expect(await screen.findByText('web runs an outdated image.')).toBeInTheDocument();
    expect(screen.getByText('Pull images, then Redeploy to update.')).toBeInTheDocument();
    expect(
      screen.getByRole('img', { name: `A newer ${web.image} was published` }),
    ).toBeInTheDocument();
    expect(screen.queryByRole('img', { name: `A newer ${db.image} was published` })).toBeNull();
  });
});
