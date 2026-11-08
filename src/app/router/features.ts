import { boardFeature } from '@/features/board';
import { clustersFeature } from '@/features/clusters';
import { containersFeature } from '@/features/containers';
import { eventsFeature } from '@/features/events';
import { hostsFeature } from '@/features/hosts';
import { imagesFeature } from '@/features/images';
import { networksFeature } from '@/features/networks';
import { overviewFeature } from '@/features/overview';
import { settingsFeature } from '@/features/settings';
import { stacksFeature } from '@/features/stacks';
import { swarmFeature } from '@/features/swarm';
import { updatesFeature } from '@/features/updates';
import { volumesFeature } from '@/features/volumes';
import type { FeatureManifest } from '@/core/feature';

/** Every feature with pages or palette commands, in sidebar order. Add one line per new feature. */
export const features: FeatureManifest[] = [
  overviewFeature,
  boardFeature,
  containersFeature,
  imagesFeature,
  volumesFeature,
  networksFeature,
  stacksFeature,
  swarmFeature,
  eventsFeature,
  settingsFeature,
  hostsFeature,
  clustersFeature,
  updatesFeature,
];

export const navEntries = features.flatMap((f) => (f.nav ? [f.nav] : []));
