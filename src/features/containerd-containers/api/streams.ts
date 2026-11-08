import type { LogStreamer } from '@/features/containers';
import { commands, subscribe, type LogEvent } from '@/shared/api';

/** The logs of a containerd container, read from the file its creator wrote. */
export function containerdLogs(hostId: string, namespace: string, id: string): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) => commands.containerdLogsSubscribe(hostId, namespace, id, options, ch),
      onEvent,
      onError,
    );
}
