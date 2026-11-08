import type { LogStreamer } from '@/features/containers';
import { commands, subscribe, type LogEvent } from '@/shared/api';

/** Opens the logs of every task of a service, for the log viewer. */
export function serviceLogs(swarmId: string, serviceId: string): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) => commands.swarmServiceLogsSubscribe(swarmId, serviceId, options, ch),
      onEvent,
      onError,
    );
}
