import { describe, expect, it } from 'vitest';
import type { PullEvent, RemoteImage } from '@/shared/api';
import { initialProgress, reduceProgress } from './progress';
import { filterRemote, remoteLabel } from './remote';

const image = (over: Partial<RemoteImage>): RemoteImage => ({
  alias: 'debian/12',
  aliases: ['debian/12', 'debian/bookworm'],
  os: 'Debian',
  release: 'bookworm',
  releaseTitle: 'bookworm',
  variant: 'default',
  arch: 'amd64',
  container: true,
  virtualMachine: true,
  serial: '20261108_05:24',
  containerSize: 100,
  virtualMachineSize: 400,
  ...over,
});

describe('remote images', () => {
  it('filter by every word and by what the image comes as', () => {
    const list = [
      image({}),
      image({
        alias: 'alpine/3.21',
        aliases: ['alpine/3.21'],
        os: 'Alpine',
        release: '3.21',
        releaseTitle: '3.21',
        virtualMachine: false,
      }),
    ];
    expect(filterRemote(list, 'debian 12', 'container').map((i) => i.alias)).toEqual(['debian/12']);
    expect(filterRemote(list, '', 'virtual-machine').map((i) => i.alias)).toEqual(['debian/12']);
    expect(filterRemote(list, 'alpine', 'container')).toHaveLength(1);
  });

  it('name an image by its system, release and variant', () => {
    expect(remoteLabel(image({ release: '12', releaseTitle: 'bookworm' }))).toBe(
      'Debian 12 (bookworm) · default',
    );
    expect(remoteLabel(image({ release: '3.21', releaseTitle: '3.21', os: 'Alpine' }))).toBe(
      'Alpine 3.21 · default',
    );
  });
});

describe('operation progress', () => {
  it('reads Incus’s percentage and ends done or failed', () => {
    const progress: PullEvent = {
      type: 'progress',
      id: null,
      status: 'rootfs: 42% (12.30MB/s)',
      current: 42,
      total: 100,
    };
    const running = reduceProgress(initialProgress, progress);
    expect(running.percent).toBe(42);
    expect(running.status).toContain('rootfs');
    expect(reduceProgress(running, { type: 'done' }).done).toBe(true);
  });
});
