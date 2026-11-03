import type { Image } from '@/shared/api';

/** The name an image is shown under: its first tag. */
export const imageName = (image: Image) => image.tags[0] ?? '<none>';
