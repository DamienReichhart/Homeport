import type { PruneSelection, SystemPruneReport } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { t, type MessageKey } from '@/shared/i18n';

export interface PruneOption {
  key: keyof PruneSelection;
  label: MessageKey;
  detail: MessageKey;
  destructive?: boolean;
}

export const PRUNE_OPTIONS: PruneOption[] = [
  {
    key: 'containers',
    label: 'overview.prune.containers',
    detail: 'overview.prune.containers.detail',
  },
  {
    key: 'images',
    label: 'overview.prune.images',
    detail: 'overview.prune.images.detail',
  },
  {
    key: 'allImages',
    label: 'overview.prune.allImages',
    detail: 'overview.prune.allImages.detail',
  },
  {
    key: 'networks',
    label: 'overview.prune.networks',
    detail: 'overview.prune.networks.detail',
  },
  {
    key: 'buildCache',
    label: 'overview.prune.buildCache',
    detail: 'overview.prune.buildCache.detail',
  },
  {
    key: 'volumes',
    label: 'overview.prune.volumes',
    detail: 'overview.prune.volumes.detail',
    destructive: true,
  },
];

/** What the clean-up removes unless the user changes it: nothing that holds data. */
export const defaultPruneSelection: PruneSelection = {
  containers: true,
  images: true,
  allImages: false,
  networks: true,
  volumes: false,
  buildCache: true,
};

/** The selection to send: removing all unused images removes the dangling ones too. */
export function toPruneRequest(selection: PruneSelection): PruneSelection {
  return { ...selection, images: selection.images || selection.allImages };
}

/** One line on what a clean-up removed. */
export function describePruneReport(r: SystemPruneReport): string {
  return t('overview.pruned', {
    containers: r.containers,
    images: r.images,
    networks: r.networks,
    volumes: r.volumes,
    space: formatBytes(r.spaceReclaimed),
  });
}
