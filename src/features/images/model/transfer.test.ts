import type { Image } from '@/shared/api';
import { archiveName, importName, saveNames } from './transfer';

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

  it('names an imported filesystem after its file', () => {
    expect(importName('/home/me/Debian RootFS.tar.gz')).toBe('debian-rootfs:latest');
    expect(importName('C:\\exports\\web.tar')).toBe('web:latest');
    expect(importName('/tmp/.tar')).toBe('imported:latest');
  });
});
