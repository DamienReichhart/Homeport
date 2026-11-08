import { apiResourcesFeature } from '@/features/api-resources';
import { applyFeature } from '@/features/apply';
import { boardFeature } from '@/features/board';
import { clusterEventsFeature } from '@/features/cluster-events';
import { clustersFeature } from '@/features/clusters';
import { configFeature } from '@/features/config';
import { containerdContainersFeature } from '@/features/containerd-containers';
import { containerdImagesFeature } from '@/features/containerd-images';
import { containerdNamespacesFeature } from '@/features/containerd-namespaces';
import { containerdSnapshotsFeature } from '@/features/containerd-snapshots';
import { containersFeature } from '@/features/containers';
import { eventsFeature } from '@/features/events';
import { helmFeature } from '@/features/helm';
import { hostsFeature } from '@/features/hosts';
import { imagesFeature } from '@/features/images';
import { incusImagesFeature } from '@/features/incus-images';
import { incusInstancesFeature } from '@/features/incus-instances';
import { incusProfilesFeature } from '@/features/incus-profiles';
import { networkFeature } from '@/features/network';
import { networksFeature } from '@/features/networks';
import { nodesFeature } from '@/features/nodes';
import { overviewFeature } from '@/features/overview';
import { podmanPodsFeature } from '@/features/podman-pods';
import { podsFeature } from '@/features/pods';
import { quadletsFeature } from '@/features/quadlets';
import { portForwardsFeature } from '@/features/port-forwards';
import { settingsFeature } from '@/features/settings';
import { stacksFeature } from '@/features/stacks';
import { swarmFeature } from '@/features/swarm';
import { updatesFeature } from '@/features/updates';
import { volumesFeature } from '@/features/volumes';
import { workloadsFeature } from '@/features/workloads';
import { navEntriesOf, type FeatureManifest } from '@/core/feature';

/** Every feature with pages or palette commands, in sidebar order. Add one line per new feature. */
export const features: FeatureManifest[] = [
  overviewFeature,
  boardFeature,
  containersFeature,
  imagesFeature,
  volumesFeature,
  networksFeature,
  stacksFeature,
  podmanPodsFeature,
  quadletsFeature,
  containerdContainersFeature,
  containerdImagesFeature,
  containerdSnapshotsFeature,
  containerdNamespacesFeature,
  incusInstancesFeature,
  incusImagesFeature,
  incusProfilesFeature,
  swarmFeature,
  workloadsFeature,
  podsFeature,
  networkFeature,
  configFeature,
  nodesFeature,
  clusterEventsFeature,
  apiResourcesFeature,
  applyFeature,
  helmFeature,
  portForwardsFeature,
  eventsFeature,
  settingsFeature,
  hostsFeature,
  clustersFeature,
  updatesFeature,
];

export const navEntries = navEntriesOf(features);
