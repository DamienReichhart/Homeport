import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ContainerdImage } from '@/shared/api';
import { containerdHost } from '@/test/containerd';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const image = (name: string, over: Partial<ContainerdImage> = {}): ContainerdImage => ({
  name,
  digest: 'sha256:49340b346798875b5592e5d420241f2f9cfba5f47e62ebb1446a1b34734e898f',
  mediaType: 'application/vnd.oci.image.index.v1+json',
  size: 3_800_000,
  platforms: ['linux/amd64', 'linux/arm64/v8'],
  createdAt: '2026-11-08T10:00:00Z',
  labels: {},
  containers: 0,
  ...over,
});

function setup(calls: { cmd: string; args: Record<string, unknown> }[] = []) {
  mockCommands({
    ...baseCommands,
    hosts_list: () => [containerdHost],
    containerd_namespaces: () => [{ name: 'default', labels: {}, containers: 1, images: 2 }],
    containerd_images: () => [
      image('docker.io/library/alpine:3.22', { containers: 1 }),
      image('docker.io/library/busybox:1.37', { size: null }),
    ],
    containerd_image_remove: (args) => {
      calls.push({ cmd: 'containerd_image_remove', args });
      return null;
    },
    containerd_images_pull_subscribe: (args) => {
      calls.push({ cmd: 'containerd_images_pull_subscribe', args });
      return 1;
    },
    images_search: (args) => {
      calls.push({ cmd: 'images_search', args });
      return [];
    },
  });
  return userEvent.setup();
}

describe('containerd images', () => {
  it('list the namespace’s images and keep those in use', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/containerd/images');
    expect(await screen.findByText('docker.io/library/alpine:3.22')).toBeInTheDocument();
    expect(screen.getAllByText('linux/arm64/v8')).toHaveLength(2);
    expect(
      screen.getByRole('button', { name: 'Remove docker.io/library/alpine:3.22' }),
    ).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Remove docker.io/library/busybox:1.37' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'containerd_image_remove',
        args: { hostId: 'c1', namespace: 'default', name: 'docker.io/library/busybox:1.37' },
      }),
    );
  });

  it('pull into the namespace through containerd, without Docker Hub’s search', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/containerd/images');
    await user.click(await screen.findByRole('button', { name: 'Pull image' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Image reference'), 'nginx');
    await user.click(within(dialog).getByRole('button', { name: 'Pull' }));
    await waitFor(() =>
      expect(calls.find((c) => c.cmd === 'containerd_images_pull_subscribe')?.args).toMatchObject({
        hostId: 'c1',
        namespace: 'default',
        reference: 'nginx',
      }),
    );
    expect(calls.some((c) => c.cmd === 'images_search')).toBe(false);
    const channel = calls.find((c) => c.cmd === 'containerd_images_pull_subscribe')?.args.onEvent;
    sendToChannel(channel, {
      type: 'progress',
      id: '7761870a5aa9',
      status: 'downloading',
      current: 512,
      total: 2048,
    });
    expect(await within(dialog).findByText('7761870a5aa9')).toBeInTheDocument();
  });
});
