import type { Image } from '@/shared/api';
import { archiveName, initialTransfer, reduceTransfer, saveNames } from './transfer';

const image = (tags: string[], id = 'sha256:0123456789abcdef'): Image => ({
  id,
  tags,
  digests: [],
  created: 0,
  size: 0,
  containers: 0,
  labels: {},
});

describe('image transfers', () => {
  it('follows bytes, lines and the names at the end', () => {
    let s = reduceTransfer(initialTransfer, { type: 'progress', bytes: 1024, total: 4096 });
    s = reduceTransfer(s, { type: 'progress', bytes: 2048, total: null });
    expect([s.bytes, s.total]).toEqual([2048, 4096]);
    s = reduceTransfer(s, { type: 'line', text: 'Loaded image: nginx:1.27' });
    s = reduceTransfer(s, { type: 'done', images: ['nginx:1.27'] });
    expect(s).toMatchObject({
      lines: ['Loaded image: nginx:1.27'],
      images: ['nginx:1.27'],
      done: true,
    });
  });

  it('names the archive after the image', () => {
    expect(archiveName([image(['ghcr.io/org/app:1.2'])])).toBe('ghcr.io-org-app-1.2.tar');
    expect(archiveName([image([])])).toBe('0123456789ab.tar');
    expect(archiveName([image(['a']), image(['b'])])).toBe('images.tar');
  });

  it('saves every tag, or the ID of an untagged image', () => {
    expect(saveNames([image(['a:1', 'a:latest']), image([], 'sha256:ff')])).toEqual([
      'a:1',
      'a:latest',
      'sha256:ff',
    ]);
  });
});
