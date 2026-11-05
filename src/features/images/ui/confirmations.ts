import type { Image } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { imageName } from '../model/imageName';
import { t } from '@/shared/i18n';

/** Removing an image, warning when it goes by several names. */
export function removeImageConfirmation(
  image: Image,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: t('images.remove.title', { name: imageName(image) }),
    description:
      image.tags.length > 1
        ? t('images.remove.everyName', { names: image.tags.join(', ') })
        : t('images.remove.description'),
    confirmLabel: t('common.remove'),
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
    title: all ? t('images.prune.allTitle') : t('images.prune.danglingTitle'),
    description: all ? t('images.prune.allDescription') : t('images.prune.danglingDescription'),
    confirmLabel: t('images.cleanUp'),
    destructive: true,
    action,
  };
}
