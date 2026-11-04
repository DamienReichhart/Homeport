import type { PruneSelection, SystemPruneReport } from '@/shared/api';
import { formatBytes } from '@/shared/lib';

export interface PruneOption {
  key: keyof PruneSelection;
  label: string;
  detail: string;
  destructive?: boolean;
}

export const PRUNE_OPTIONS: PruneOption[] = [
  {
    key: 'containers',
    label: 'Stopped containers',
    detail: 'Removes every container that is not running.',
  },
  {
    key: 'images',
    label: 'Dangling images',
    detail: 'Untagged layers left behind by builds and pulls.',
  },
  {
    key: 'allImages',
    label: 'All unused images',
    detail: 'Also removes tagged images no container uses.',
  },
  {
    key: 'networks',
    label: 'Unused networks',
    detail: 'Custom networks no container is attached to.',
  },
  { key: 'buildCache', label: 'Build cache', detail: 'Cached build layers.' },
  {
    key: 'volumes',
    label: 'Unused volumes',
    detail: 'Deletes the data in every volume no container uses, named volumes included.',
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
  return `Removed ${r.containers} containers, ${r.images} images, ${r.networks} networks and ${r.volumes} volumes — ${formatBytes(r.spaceReclaimed)} reclaimed.`;
}
