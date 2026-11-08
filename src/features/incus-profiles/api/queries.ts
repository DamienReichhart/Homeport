import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const incusProfileQueries = {
  /** The profiles of `project`, with the Instances using each; profile events refresh them. */
  list: (hostId: string, project: string) =>
    queryOptions({
      queryKey: hostKeys.incus(hostId, 'profiles', project),
      queryFn: () => commands.incusProfiles(hostId, project),
    }),
  /** One profile with its YAML and ETag; under the list, so the same events refresh it. */
  detail: (hostId: string, project: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'profiles', project), 'detail', name],
      queryFn: () => commands.incusProfile(hostId, project, name),
    }),
};
