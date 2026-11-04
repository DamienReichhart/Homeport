import type { Image } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { imageName } from '../model/imageName';

/** Removing an image, warning when it goes by several names. */
export function removeImageConfirmation(
  image: Image,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: `Remove ${imageName(image)}?`,
    description:
      image.tags.length > 1
        ? `This removes every name of the image (${image.tags.join(', ')}) and its layers that nothing else uses.`
        : 'The image and the layers no other image uses are deleted. It can be pulled again later.',
    confirmLabel: 'Remove',
    destructive: true,
    action,
  };
}

/** Removing dangling images, or every unused one. */
export function pruneImagesConfirmation(
  all: boolean,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: all ? 'Remove all unused images?' : 'Remove dangling images?',
    description: all
      ? 'Every image no container uses is deleted, tagged ones included.'
      : 'Untagged images left behind by builds and pulls are deleted.',
    confirmLabel: 'Clean up',
    destructive: true,
    action,
  };
}
