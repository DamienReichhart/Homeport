import type { LogStreamer } from '@/features/containers';
import { commands, subscribe, type LogEvent, type PodLogSource } from '@/shared/api';

/** Opens the logs of one or more Pods' `container` (or its previous instance), for the log viewer. */
export function podLogs(
  clusterId: string,
  namespace: string,
  sources: PodLogSource[],
  container: string | null,
  previous: boolean,
): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) =>
        commands.clustersPodLogsSubscribe(
          clusterId,
          namespace,
          sources,
          { container, previous },
          options,
          ch,
        ),
      onEvent,
      onError,
    );
}
