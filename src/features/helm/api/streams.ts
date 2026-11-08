import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { commands, subscribe, type AppError, type CliEvent } from '@/shared/api';
import { helmKeys } from './queries';

/** Starts a streamed helm run; returns its canceller. */
export type HelmRun = (
  onEvent: (event: CliEvent) => void,
  onError: (error: AppError) => void,
) => () => void;

/** Downloads the indexes of `names`, or of every repository; charts are searched afresh after. */
export function useUpdateRepositories(): (names: string[]) => HelmRun {
  const client = useQueryClient();
  return useCallback(
    (names) => (onEvent, onError) =>
      subscribe<CliEvent>(
        (ch) => commands.helmReposUpdateSubscribe(names, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit')
            void client.invalidateQueries({ queryKey: helmKeys.repositories });
        },
        onError,
      ),
    [client],
  );
}
