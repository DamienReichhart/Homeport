import { useEngineEventUpdates, useHostStatusUpdates } from '@/core/host';

/** Keeps cached Host and Engine data fresh from the backend's pushes. */
export function LiveUpdates() {
  useHostStatusUpdates();
  useEngineEventUpdates();
  return null;
}
