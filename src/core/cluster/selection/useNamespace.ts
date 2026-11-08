import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { patchSettings, settingsQuery } from '@/core/settings';
import type { ClusterView } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { ALL_NAMESPACES, chosenNamespace } from '../model/namespace';
import { t } from '@/shared/i18n';

/**
 * The Namespace the selected Cluster's pages show (`null` for every one),
 * remembered per Cluster, and a way to pick another.
 */
export function useNamespace(cluster: ClusterView): [string | null, (ns: string | null) => void] {
  const client = useQueryClient();
  const settings = useQuery(settingsQuery());
  const namespace = chosenNamespace(cluster, settings.data?.clusterNamespaces);
  const id = cluster.cluster.id;
  const choose = useCallback(
    (next: string | null) => {
      const current = client.getQueryData(settingsQuery().queryKey)?.clusterNamespaces ?? {};
      const clusterNamespaces = { ...current, [id]: next ?? ALL_NAMESPACES };
      // Shown at once; saved behind.
      client.setQueryData(settingsQuery().queryKey, (s) => s && { ...s, clusterNamespaces });
      patchSettings(client, { clusterNamespaces }).catch((e: unknown) =>
        toastError(e, t('core.namespace.rememberFailed')),
      );
    },
    [client, id],
  );
  return [namespace, choose];
}
