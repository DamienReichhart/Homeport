import { useEffect } from 'react';
import { useUsage } from '@/features/containers';
import type { ContainerUsage } from '@/shared/api';

/** Streams one Host's container usage to `onUsage`; one per Host on the board. */
export function UsageFeed({
  hostId,
  onUsage,
}: {
  hostId: string;
  onUsage: (hostId: string, usage: Map<string, ContainerUsage>) => void;
}) {
  const usage = useUsage(hostId);
  useEffect(() => onUsage(hostId, usage), [hostId, usage, onUsage]);
  return null;
}
