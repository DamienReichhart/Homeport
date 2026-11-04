import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const image = (id: string, tags: string[]) => ({
  id,
  tags,
  digests: [],
  created: 1_700_000_000,
  size: 1024,
  containers: 0,
  labels: {},
});

describe('saving and loading images', () => {
  it('loads an archive and names the images it held', async () => {
    let source: unknown = null;
    mockCommands({
      ...baseCommands,
      images_list: () => [image('sha256:a', ['alpine:3.22'])],
      'plugin:dialog|open': () => '/home/me/nginx.tar',
      images_load_subscribe: (args) => {
        source = args.source;
        setTimeout(() =>
          sendToChannel(
            args.onEvent,
            { type: 'progress', bytes: 2 * 1024 ** 2, total: 4 * 1024 ** 2 },
            { type: 'line', text: 'Loaded image: nginx:1.27' },
            { type: 'done', images: ['nginx:1.27'] },
          ),
        );
        return 4;
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'Load…' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText('Done: nginx:1.27')).toBeInTheDocument();
    expect(within(dialog).getByText('Loaded image: nginx:1.27')).toBeInTheDocument();
    expect(source).toBe('/home/me/nginx.tar');
  });

  it('saves the selected images into one archive', async () => {
    let saved: Record<string, unknown> | null = null;
    mockCommands({
      ...baseCommands,
      images_list: () => [image('sha256:a', ['alpine:3.22']), image('sha256:b', ['nginx:1.27'])],
      'plugin:dialog|save': () => '/home/me/images.tar',
      images_save_subscribe: (args) => {
        saved = args;
        setTimeout(() =>
          sendToChannel(
            args.onEvent,
            { type: 'progress', bytes: 1024, total: 1024 },
            { type: 'done', images: ['alpine:3.22', 'nginx:1.27'] },
          ),
        );
        return 5;
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('checkbox', { name: 'Select all' }));
    await user.click(screen.getByRole('button', { name: 'Save 2 images…' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText('Done.')).toBeInTheDocument();
    expect(saved).toMatchObject({
      names: ['alpine:3.22', 'nginx:1.27'],
      destination: '/home/me/images.tar',
    });
  });
});
