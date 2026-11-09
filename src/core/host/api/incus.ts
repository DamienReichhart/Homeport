import { queryOptions, useQuery, useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { patchSettings, settingsQuery } from '@/core/settings';
import { commands } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { chosenIncusProject } from '../model/incus';
import { hostKeys } from './keys';
import { t } from '@/shared/i18n';

export const incusQueries = {
  /** The projects the Host's connection may use, with what each holds. */
  projects: (hostId: string) =>
    queryOptions({
      queryKey: hostKeys.incusProjects(hostId),
      queryFn: () => commands.incusProjects(hostId),
    }),
};

/**
 * The project an Incus Host's pages show (`null` until it's known),
 * remembered per Host across launches, and a way to pick another.
 */
export function useIncusProject(hostId: string): [string | null, (project: string) => void] {
  const client = useQueryClient();
  const settings = useQuery(settingsQuery());
  const remembered = settings.data?.incusProjects;
  const projects = useQuery({ ...incusQueries.projects(hostId), enabled: !!settings.data });
  const project = settings.isPending
    ? null
    : chosenIncusProject(hostId, remembered, projects.data ?? (projects.error ? [] : undefined));
  const choose = useCallback(
    (next: string) => {
      const current = client.getQueryData(settingsQuery().queryKey)?.incusProjects ?? {};
      const incusProjects = { ...current, [hostId]: next };
      // Shown at once; saved behind.
      client.setQueryData(settingsQuery().queryKey, (s) => s && { ...s, incusProjects });
      patchSettings(client, { incusProjects }).catch((e: unknown) =>
        toastError(e, t('core.incus.rememberFailed')),
      );
    },
    [client, hostId],
  );
  return [project, choose];
}
