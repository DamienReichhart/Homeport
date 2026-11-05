import type { Image } from '@/shared/api';
import { t } from '@/shared/i18n';

/** The names `docker save` should write for a selection: tags, or IDs for untagged images. */
export const saveNames = (images: Image[]) =>
  images.flatMap((i) => (i.tags.length > 0 ? i.tags : [i.id]));

/** `nginx-1.27.tar` for one image, `images.tar` for several. */
export function archiveName(images: Image[]): string {
  const [only] = images;
  if (images.length !== 1 || !only) return 'images.tar';
  const name = only.tags[0] ?? only.id.replace(/^sha256:/, '').slice(0, 12);
  return `${name.replace(/[/:@\\]+/g, '-')}.tar`;
}

/** Archive files the Engine loads: plain and compressed tars. */
export const loadFilters = () => [
  { name: t('images.files.imageArchives'), extensions: ['tar', 'gz', 'tgz'] },
];
/** What `docker save` writes. */
export const saveFilters = () => [{ name: t('images.files.imageArchive'), extensions: ['tar'] }];

/** Tars `docker import` reads: plain and compressed. */
export const importFilters = () => [
  { name: t('images.files.filesystemArchives'), extensions: ['tar', 'gz', 'tgz', 'xz', 'bz2'] },
];

/** `debian-rootfs:latest` for `/home/me/Debian RootFS.tar.gz`. */
export function importName(path: string): string {
  const file = path.split(/[\\/]/).pop() ?? '';
  const base = file.replace(/\.(tar(\.(gz|xz|bz2))?|tgz)$/i, '');
  const repo = base
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, '-')
    .replace(/^[-._]+|[-._]+$/g, '');
  return `${repo || 'imported'}:latest`;
}
