import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ContainerdSnapshot } from '@/shared/api';
import { containerdHost } from '@/test/containerd';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const layer = `sha256:${'a'.repeat(64)}`;
const snap = (key: string, over: Partial<ContainerdSnapshot> = {}): ContainerdSnapshot => ({
  key,
  parent: layer,
  kind: 'active',
  size: 4096,
  inodes: 3,
  createdAt: null,
  labels: {},
  users: [],
  children: 0,
  ...over,
});

describe('containerd snapshots', () => {
  it('show usage and users, and remove only orphaned ones', async () => {
    const calls: Record<string, unknown>[] = [];
    const listed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      hosts_list: () => [containerdHost],
      containerd_namespaces: () => [{ name: 'default', labels: {}, containers: 1, images: 1 }],
      containerd_snapshots: ({ snapshotter }) => {
        listed.push(snapshotter);
        return {
          snapshotter: (snapshotter as string | null) ?? 'overlayfs',
          snapshotters: ['overlayfs', 'native'],
          total: 8_400_000,
          snapshots: [
            snap(layer, {
              parent: '',
              kind: 'committed',
              size: 8_000_000,
              children: 2,
              users: [{ kind: 'image', name: 'docker.io/library/alpine:3.22' }],
            }),
            snap('web', { users: [{ kind: 'container', id: 'web' }] }),
            snap('orphan'),
          ],
        };
      },
      containerd_snapshot_remove: (args) => {
        calls.push(args);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/containerd/snapshots');
    expect(await screen.findByText('sha256:aaaaaaaaaaaa')).toBeInTheDocument();
    expect(screen.getByText(/use 8\.0 MB; 1 is unused/)).toBeInTheDocument();
    expect(screen.getByText('image docker.io/library/alpine:3.22')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Remove web' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Remove sha256:aaaaaaaaaaaa' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Remove orphan' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        hostId: 'c1',
        namespace: 'default',
        snapshotter: 'overlayfs',
        key: 'orphan',
      }),
    );
    await user.click(screen.getByLabelText('Only unused'));
    expect(screen.queryByText('web')).toBeNull();
    // The Host’s default snapshotter until another is picked.
    expect(new Set(listed)).toEqual(new Set([null]));
  });
});
