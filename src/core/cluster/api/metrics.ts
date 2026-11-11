import {
  commands,
  subscribe,
  type AppError,
  type MetricsEvent,
  type MetricsTarget,
} from '@/shared/api';

/**
 * Polls the Metrics API for `target` every 15 s; the returned function
 * stops it, on the backend too.
 */
export function streamMetrics(
  clusterId: string,
  target: MetricsTarget,
  onEvent: (event: MetricsEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<MetricsEvent>(
    (channel) => commands.clustersMetricsSubscribe(clusterId, target, channel),
    onEvent,
    onError,
  );
}
