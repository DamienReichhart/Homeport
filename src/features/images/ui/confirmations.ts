import type { Image } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { imageName } from '../model/imageName';
import { t } from '@/shared/i18n';

/** Removing an image from `host`, warning when it goes by several names. */
export function removeImageConfirmation(
  image: Image,
  host: string,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: t('images.remove.title', { name: imageName(image), host }),
    description:
      image.tags.length > 1
        ? t('images.remove.everyName', { names: image.tags.join(', ') })
        : t('images.remove.description'),
    confirmLabel: t('common.remove'),
    destructive: true,
    action,
  };
}

/** Removing dangling images from `host`, or every unused one. */
export function pruneImagesConfirmation(
  all: boolean,
  host: string,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: all ? t('images.prune.allTitle', { host }) : t('images.prune.danglingTitle', { host }),
    description: all ? t('images.prune.allDescription') : t('images.prune.danglingDescription'),
    confirmLabel: t('images.cleanUp'),
    destructive: true,
    action,
  };
}
