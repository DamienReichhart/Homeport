import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusImage, incusProject, remoteImage } from '@/test/incus';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

type Call = { cmd: string; args: Record<string, unknown> };

function setup(calls: Call[] = []) {
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [incusHost],
    incus_projects: () => [incusProject('default', 1, 2)],
    incus_images: () => [
      incusImage('4495673b5944aaaa', { aliases: ['alpine'] }),
      incusImage('0a1b2c3d4e5fbbbb', { type: 'virtual-machine', source: null, cached: true }),
    ],
    incus_remote_images: () => [
      remoteImage('debian/12'),
      remoteImage('alpine/3.21', { os: 'Alpine', release: '3.21', releaseTitle: '3.21' }),
    ],
    incus_image_delete: record('incus_image_delete', null),
    incus_image_refresh: record('incus_image_refresh', null),
    incus_image_alias_remove: record('incus_image_alias_remove', null),
    incus_image_copy_subscribe: record('incus_image_copy_subscribe', 1),
  });
  return userEvent.setup();
}

describe('incus images', () => {
  it('list the project’s images, refresh and delete them, and drop an alias', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/images');
    expect(await screen.findByText('alpine')).toBeInTheDocument();
    expect(screen.getByText('cached')).toBeInTheDocument();
    // An image without a source can't be refreshed.
    expect(screen.getAllByRole('button', { name: /^Refresh / })).toHaveLength(1);
    await user.click(screen.getByRole('button', { name: 'Refresh alpine from its server' }));
    await user.click(screen.getByRole('button', { name: 'Remove the alias alpine' }));
    await user.click(screen.getByRole('button', { name: 'Delete 0a1b2c3d4e5f' }));
    await user.click(await screen.findByRole('button', { name: 'Delete' }));
    await waitFor(() =>
      expect(calls.map((c) => c.cmd)).toEqual([
        'incus_image_refresh',
        'incus_image_alias_remove',
        'incus_image_delete',
      ]),
    );
    expect(calls[2]!.args).toEqual({
      hostId: 'i1',
      project: 'default',
      fingerprint: '0a1b2c3d4e5fbbbb',
    });
  });

  it('get an image from the server’s catalogue and show its download', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/images');
    await user.click(await screen.findByRole('button', { name: 'Get image' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByPlaceholderText(/Filter by system/), 'alpine');
    await user.click(await within(dialog).findByRole('button', { name: /Alpine 3.21/ }));
    expect(within(dialog).getByLabelText('Alias')).toHaveValue('alpine/3.21');
    await user.click(within(dialog).getByRole('button', { name: 'Get image' }));
    await waitFor(() =>
      expect(calls.find((c) => c.cmd === 'incus_image_copy_subscribe')?.args).toMatchObject({
        hostId: 'i1',
        project: 'default',
        copy: {
          server: 'https://images.linuxcontainers.org',
          alias: 'alpine/3.21',
          type: 'container',
          autoUpdate: true,
          addAlias: true,
        },
      }),
    );
    const channel = calls.find((c) => c.cmd === 'incus_image_copy_subscribe')?.args.onEvent;
    sendToChannel(channel, {
      type: 'progress',
      id: null,
      status: 'rootfs: 42% (12.30MB/s)',
      current: 42,
      total: 100,
    });
    expect(await within(dialog).findByText('rootfs: 42% (12.30MB/s)')).toBeInTheDocument();
    sendToChannel(channel, { type: 'done' });
    expect(await within(dialog).findByText('The image is ready.')).toBeInTheDocument();
  });
});
