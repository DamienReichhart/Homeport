import type { PodContainer, PodDetail } from '@/shared/api';

/** The annotation that names the container kubectl picks by default. */
export const DEFAULT_CONTAINER = 'kubectl.kubernetes.io/default-container';

/** The container a Pod's logs and terminal open on: the one the annotation names, else its first app container. */
export function defaultContainer(pod: PodDetail): string {
  const named = pod.annotations[DEFAULT_CONTAINER];
  if (named && pod.containers.some((c) => c.name === named)) return named;
  return (pod.containers.find((c) => !c.init) ?? pod.containers[0])?.name ?? '';
}

/** Whether a container ran before, so its previous instance has logs. */
export function hasPrevious(container: PodContainer | undefined): boolean {
  return (container?.restarts ?? 0) > 0;
}
