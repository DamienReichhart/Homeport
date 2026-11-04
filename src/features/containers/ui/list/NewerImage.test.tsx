import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('newer images on the containers page', () => {
  it('marks only the container whose tag moved on, and checks again on demand', async () => {
    const forced: boolean[] = [];
    const web = container('shop-web-1', 'shop');
    const db = container('shop-db-1', 'shop');
    mockCommands({
      ...baseCommands,
      containers_list: () => [web, db],
      usage_subscribe: () => 1,
      images_list: () => [],
      networks_list: () => [],
      images_freshness: ({ force }) => {
        forced.push(force === true);
        return [
          { reference: web.image, imageId: web.imageId, freshness: 'newer' },
          { reference: db.image, imageId: db.imageId, freshness: 'upToDate' },
        ];
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');
    expect(
      await screen.findByRole('img', { name: `A newer ${web.image} was published` }),
    ).toBeInTheDocument();
    expect(screen.queryByRole('img', { name: `A newer ${db.image} was published` })).toBeNull();

    await user.click(screen.getByRole('button', { name: 'Check for newer images' }));
    expect(await screen.findByText('1 image has a newer version')).toBeInTheDocument();
    expect(forced).toEqual([false, true]);
  });

  it('shows nothing when the check knows nothing', async () => {
    const local = container('app', null);
    mockCommands({
      ...baseCommands,
      containers_list: () => [local],
      usage_subscribe: () => 1,
      images_list: () => [],
      networks_list: () => [],
      images_freshness: () => [
        { reference: local.image, imageId: local.imageId, freshness: 'unknown' },
      ],
    });
    renderApp('/containers');
    expect(await screen.findByText('app')).toBeInTheDocument();
    expect(screen.queryByRole('img', { name: /newer/ })).toBeNull();
  });
});
