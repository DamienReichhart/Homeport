import { useClusterStatusUpdates } from '@/core/cluster';
import { useEngineEventUpdates, useHostStatusUpdates } from '@/core/host';
import { useSwarmUpdates } from '@/core/swarm';
import { useForwardUpdates } from '@/features/port-forwards';
import { useSyncedUpdates } from '@/features/stacks';

/** Keeps cached Host, Engine and Cluster data fresh from the backend's pushes. */
export function LiveUpdates() {
  useHostStatusUpdates();
  useClusterStatusUpdates();
  useEngineEventUpdates();
  useSyncedUpdates();
  useSwarmUpdates();
  useForwardUpdates();
  return null;
}
