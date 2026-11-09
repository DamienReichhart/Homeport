import { useQueryClient, type QueryKey } from '@tanstack/react-query';
import { useEffect } from 'react';
import { commands, subscribe, type NomadWatch } from '@/shared/api';

/**
 * While mounted, follows what `topic` covers through Nomad's blocking
 * queries and refreshes the queries under `queryKey` when it changes,
 * instead of polling. The watch ends with the page, and waits while
 * `enabled` is false.
 */
export function useNomadWatch(
  clusterId: string,
  topic: NomadWatch,
  queryKey: QueryKey,
  enabled = true,
) {
  const client = useQueryClient();
  // The topic and key as text, so a new object with the same content doesn't restart it.
  const topicText = JSON.stringify(topic);
  const keyText = JSON.stringify(queryKey);
  useEffect(
    () =>
      !enabled
        ? undefined
        : subscribe<boolean>(
            (channel) =>
              commands.clustersNomadWatchSubscribe(
                clusterId,
                JSON.parse(topicText) as NomadWatch,
                channel,
              ),
            () => void client.invalidateQueries({ queryKey: JSON.parse(keyText) as QueryKey }),
          ),
    [client, clusterId, topicText, keyText, enabled],
  );
}
