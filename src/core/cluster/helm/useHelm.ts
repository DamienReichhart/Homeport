import { useQuery } from '@tanstack/react-query';
import { appQueries, type HelmSupport } from '@/shared/api';

/**
 * The helm CLI Helm pages run (ADR 0009), or why there's none. Checked again
 * each time a Helm page opens, so a helm installed since launch is found.
 */
export function useHelm(): { helm: HelmSupport | undefined; isPending: boolean } {
  const capabilities = useQuery({ ...appQueries.capabilities(), staleTime: 0 });
  return { helm: capabilities.data?.helm, isPending: capabilities.isPending };
}
