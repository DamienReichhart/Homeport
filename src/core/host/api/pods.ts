import { useQuery } from '@tanstack/react-query';
import { useMemo } from 'react';
import type { PodmanPod } from '@/shared/api';
import { supports } from '../model/engine';
import { useHostSelection } from '../selection/context';
import { resourceQueries } from './queries';

/** The pod a container belongs to, by name and id. */
export interface PodRef {
  id: string;
  name: string;
}

/** Each container of `pods`, by id, with the pod it is in. */
export function podsByContainer(pods: PodmanPod[]): Map<string, PodRef> {
  const index = new Map<string, PodRef>();
  for (const pod of pods)
    for (const c of pod.containers) index.set(c.id, { id: pod.id, name: pod.name });
  return index;
}

/**
 * The pod of each container on the selected Host when it runs Podman, so
 * container lists can say which pod a container belongs to; empty otherwise.
 */
export function usePodsByContainer(hostId: string): Map<string, PodRef> {
  const { host } = useHostSelection();
  const podman = host?.host.id === hostId && supports(host, 'podman');
  const pods = useQuery({ ...resourceQueries.pods(hostId), enabled: podman });
  return useMemo(
    () => (podman && pods.data ? podsByContainer(pods.data) : new Map()),
    [podman, pods.data],
  );
}
