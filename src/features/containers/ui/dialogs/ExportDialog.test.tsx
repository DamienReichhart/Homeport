import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web = container('web', null);

function page(destination: string | null, fail = false) {
  const exported: Record<string, unknown>[] = [];
  const cancelled: unknown[] = [];
  mockCommands({
    ...baseCommands,
    containers_list: () => [web],
    usage_subscribe: () => 1,
    images_list: () => [],
    networks_list: () => [],
    'plugin:dialog|save': () => destination,
    containers_export_subscribe: (args) => {
      exported.push(args);
      setTimeout(() =>
        sendToChannel(
          args.onEvent,
          { type: 'progress', bytes: 3 * 1024 ** 2, total: null },
          ...(fail
            ? [
                {
                  type: 'error',
                  error: {
                    code: 'io',
                    message: 'Homeport can’t write /home/me/web.tar.',
                    hint: null,
                    detail: null,
                  },
                },
              ]
            : []),
        ),
      );
      return 9;
    },
    stream_cancel: ({ id }) => {
      cancelled.push(id);
      return null;
    },
  });
  renderApp('/containers');
  return { exported, cancelled };
}

async function openExport(user: ReturnType<typeof userEvent.setup>) {
  const row = (await screen.findByText('web')).closest('tr')!;
  await user.click(within(row).getByRole('button', { name: 'More actions' }));
  await user.click(await screen.findByRole('menuitem', { name: 'Export filesystem…' }));
}

describe('exporting a container', () => {
  it('streams the filesystem to the chosen file, and cancels', async () => {
    const { exported, cancelled } = page('/home/me/web.tar');
    const user = userEvent.setup();
    await openExport(user);
    const dialog = await screen.findByRole('dialog', { name: 'Exporting web' });
    expect(await within(dialog).findByText('3.0 MB')).toBeInTheDocument();
    expect(exported[0]).toMatchObject({ id: web.id, destination: '/home/me/web.tar' });

    await user.click(within(dialog).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(cancelled).toEqual([9]));
  });

  it('does nothing when no file is chosen', async () => {
    const { exported } = page(null);
    const user = userEvent.setup();
    await openExport(user);
    await waitFor(() => expect(screen.queryByRole('menuitem')).toBeNull());
    expect(screen.queryByRole('dialog')).toBeNull();
    expect(exported).toHaveLength(0);
  });

  it('shows a failed export', async () => {
    page('/home/me/web.tar', true);
    const user = userEvent.setup();
    await openExport(user);
    const dialog = await screen.findByRole('dialog', { name: 'Exporting web' });
    expect(
      await within(dialog).findByText('Homeport can’t write /home/me/web.tar.'),
    ).toBeInTheDocument();
    expect(within(dialog).queryByRole('button', { name: 'Cancel' })).toBeNull();
  });
});
