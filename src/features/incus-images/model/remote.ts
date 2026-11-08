import type { InstanceType, RemoteImage } from '@/shared/api';

/** The public image server Incus's `images:` remote points to. */
export const DEFAULT_SERVER = 'https://images.linuxcontainers.org';

/** The images that come as `type`, and whose names or aliases hold every word of `query`. */
export function filterRemote(
  images: RemoteImage[],
  query: string,
  type: InstanceType,
): RemoteImage[] {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean);
  return images.filter((image) => {
    if (type === 'container' ? !image.container : !image.virtualMachine) return false;
    const text = [image.os, image.release, image.releaseTitle, image.variant, ...image.aliases]
      .join(' ')
      .toLowerCase();
    return words.every((w) => text.includes(w));
  });
}

/** "Debian 12 (bookworm) · cloud" */
export function remoteLabel(image: RemoteImage): string {
  const release =
    image.releaseTitle && image.releaseTitle !== image.release
      ? `${image.release} (${image.releaseTitle})`
      : image.release;
  return `${image.os} ${release} · ${image.variant}`;
}
