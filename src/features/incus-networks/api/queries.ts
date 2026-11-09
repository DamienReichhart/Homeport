import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const incusNetworkQueries = {
  /** The networks the project sees; network events refresh them. */
  list: (hostId: string, project: string) =>
    queryOptions({
      queryKey: hostKeys.incus(hostId, 'networks', project),
      queryFn: () => commands.incusNetworks(hostId, project),
    }),
  /** A network's addresses on the host and its DHCP leases, read again every 10 s. */
  state: (hostId: string, project: string, name: string) =>
    queryOptions({
      queryKey: [...hostKeys.incus(hostId, 'networks', project), 'state', name],
      queryFn: () => commands.incusNetworkState(hostId, project, name),
      refetchInterval: 10_000,
    }),
};
