import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, fileEntries } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { BrowseVolumeDialog } from './BrowseVolumeDialog';

const volume = {
  name: 'shop_db',
  driver: 'local',
  mountpoint: '/x',
  createdAt: null,
  labels: {},
  stack: 'shop',
  usedBy: ['shop-db-1'],
};

function browse({ present, pull }: { present: boolean; pull?: 'done' | 'error' }) {
  const calls = {
    pulls: 0,
    started: [] as unknown[],
    ended: [] as unknown[],
    listed: [] as unknown[],
  };
  mockCommands({
    ...baseCommands,
    volumes_helper_image: () => ({ image: 'busybox:1.37', present, size: 2_200_000 }),
    volumes_helper_pull_subscribe: ({ onEvent }) => {
      calls.pulls += 1;
      setTimeout(() =>
        sendToChannel(
          onEvent,
          { type: 'progress', id: 'l1', status: 'Downloading', current: 1, total: 2 },
          pull === 'error'
            ? {
                type: 'error',
                error: {
                  code: 'helper_image_unavailable',
                  message: "Homeport couldn't get busybox:1.37 onto this Host to open the volume.",
                  hint: { text: 'Pull it on the Host.', commands: ['docker pull busybox:1.37'] },
                  detail: 'toomanyrequests',
                },
              }
            : { type: 'done' },
        ),
      );
      return 7;
    },
    volumes_browse_start: (args) => {
      calls.started.push(args);
      return 'helper-1';
    },
    volumes_browse_list: (args) => {
      calls.listed.push(args.path);
      return fileEntries(args.path as string);
    },
    volumes_browse_end: (args) => {
      calls.ended.push(args.session);
      return null;
    },
  });
  return calls;
}

describe('browsing a volume', () => {
  it('opens at once when the Host has the helper image, and ends the session on close', async () => {
    const calls = browse({ present: true });
    const user = userEvent.setup();
    const onClose = vi.fn();
    const { unmount } = renderWithProviders(
      <BrowseVolumeDialog hostId="h1" volume={volume} onClose={onClose} />,
    );

    expect(await screen.findByText('hello.txt')).toBeInTheDocument();
    expect(calls.started).toEqual([{ hostId: 'h1', volume: 'shop_db', pulledImage: false }]);
    await user.click(screen.getByText('etc'));
    await waitFor(() => expect(calls.listed).toEqual(['/', '/etc']));

    unmount();
    await waitFor(() => expect(calls.ended).toEqual(['helper-1']));
    expect(calls.pulls).toBe(0);
  });

  it('asks before pulling the helper image, then removes it with the session', async () => {
    const calls = browse({ present: false, pull: 'done' });
    const user = userEvent.setup();
    renderWithProviders(<BrowseVolumeDialog hostId="h1" volume={volume} onClose={vi.fn()} />);

    expect(
      await screen.findByText(/needs the busybox:1.37 image \(about 2 MB\)/),
    ).toBeInTheDocument();
    expect(calls.pulls).toBe(0);
    expect(calls.started).toEqual([]);

    await user.click(screen.getByRole('button', { name: 'Pull busybox:1.37' }));
    expect(await screen.findByText('hello.txt')).toBeInTheDocument();
    expect(calls.pulls).toBe(1);
    expect(calls.started).toEqual([{ hostId: 'h1', volume: 'shop_db', pulledImage: true }]);
  });

  it('says how to get the image when the pull fails, and cancelling opens nothing', async () => {
    const calls = browse({ present: false, pull: 'error' });
    const user = userEvent.setup();
    const onClose = vi.fn();
    renderWithProviders(<BrowseVolumeDialog hostId="h1" volume={volume} onClose={onClose} />);

    await user.click(await screen.findByRole('button', { name: 'Pull busybox:1.37' }));
    expect(
      await screen.findByText(
        "Homeport couldn't get busybox:1.37 onto this Host to open the volume.",
      ),
    ).toBeInTheDocument();
    expect(screen.getByText('docker pull busybox:1.37')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onClose).toHaveBeenCalled();
    expect(calls.started).toEqual([]);
  });
});
