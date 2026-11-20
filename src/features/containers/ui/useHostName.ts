import { useHosts } from '@/core/host';

/** A Host's name, for confirmations that say where something goes; its ID until the list loads. */
export function useHostName(hostId: string): string {
  const hosts = useHosts();
  return hosts.data?.find((h) => h.host.id === hostId)?.host.name ?? hostId;
}
