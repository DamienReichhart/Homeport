import { queryOptions, useQuery, useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { patchSettings, settingsQuery } from '@/core/settings';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { chosenContainerdNamespace } from '../model/containerd';
import { hostKeys } from './keys';
import { t } from '@/shared/i18n';

export const containerdQueries = {
  /** The Host's namespaces, with how many containers and images each holds. */
  namespaces: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.containerdNamespaces(hostId),
      queryFn: () => commands.containerdNamespaces(hostId),
    }),
};

/**
 * The namespace a containerd Host's pages show (`null` until it's known),
 * remembered per Host across launches, and a way to pick another.
 */
export function useContainerdNamespace(hostId: string): [string | null, (ns: string) => void] {
  const client = useQueryClient();
  const settings = useQuery(settingsQuery());
  const remembered = settings.data?.containerdNamespaces;
  const namespaces = useQuery({
    ...containerdQueries.namespaces(hostId),
    enabled: !!settings.data,
  });
  const namespace = settings.isPending
    ? null
    : chosenContainerdNamespace(
        hostId,
        remembered,
        namespaces.data ?? (namespaces.error ? [] : undefined),
      );
  const choose = useCallback(
    (next: string) => {
      const current = client.getQueryData(settingsQuery().queryKey)?.containerdNamespaces ?? {};
      const containerdNamespaces = { ...current, [hostId]: next };
      // Shown at once; saved behind.
      client.setQueryData(settingsQuery().queryKey, (s) => s && { ...s, containerdNamespaces });
      patchSettings(client, { containerdNamespaces }).catch((e: unknown) =>
        toastError(e, t('core.containerd.rememberFailed')),
      );
    },
    [client, hostId],
  );
  return [namespace, choose];
}
