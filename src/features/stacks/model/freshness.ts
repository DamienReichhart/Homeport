import { aggregateFreshness, containerFreshness, type FreshnessIndex } from '@/core/host';
import type { Container, Freshness, Stack } from '@/shared/api';

export interface ServiceFreshness {
  service: string;
  freshness: Freshness;
  /** The images its containers run, for messages. */
  images: string[];
}

/** Each service of a Stack, from the containers it runs. Works for any kind of Stack. */
export function serviceFreshness(
  stack: Stack,
  containers: Map<string, Container>,
  index: FreshnessIndex,
): ServiceFreshness[] {
  return stack.services.map((s) => {
    const running = s.containerIds.flatMap((id) => containers.get(id) ?? []);
    return {
      service: s.name,
      freshness: aggregateFreshness(running.map((c) => containerFreshness(index, c))),
      images: [...new Set(running.map((c) => c.image))],
    };
  });
}

/** The Stack as a whole, and the services that run an outdated image. */
export function stackFreshness(services: ServiceFreshness[]): {
  freshness: Freshness;
  newer: ServiceFreshness[];
} {
  return {
    freshness: aggregateFreshness(services.map((s) => s.freshness)),
    newer: services.filter((s) => s.freshness === 'newer'),
  };
}

/** "Newer images for web, worker". */
export const newerServicesLabel = (newer: ServiceFreshness[]) =>
  `Newer images for ${newer.map((s) => s.service).join(', ')}`;
