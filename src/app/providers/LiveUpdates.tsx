import { useEngineEventUpdates, useHostStatusUpdates } from '@/core/host';
import { useSyncedUpdates } from '@/features/stacks';

/** Keeps cached Host and Engine data fresh from the backend's pushes. */
export function LiveUpdates() {
  useHostStatusUpdates();
  useEngineEventUpdates();
  useSyncedUpdates();
  return null;
}
