import { useQuery } from '@tanstack/react-query';
import { settingsQuery } from '@/core/settings';
import { commands } from '@/shared/api';
import { CHECK_INTERVAL_MS } from '../model/updates';

export const updatesKey = ['updates'] as const;

/**
 * The latest update check. Runs at launch and every few hours while
 * `checkForUpdates` is on; `refetch()` checks now either way.
 */
export function useUpdateCheck() {
  const settings = useQuery(settingsQuery());
  return useQuery({
    queryKey: updatesKey,
    queryFn: () => commands.updatesCheck(),
    enabled: settings.data?.checkForUpdates ?? false,
    staleTime: CHECK_INTERVAL_MS,
    refetchInterval: CHECK_INTERVAL_MS,
    refetchOnWindowFocus: false,
    retry: false,
  });
}
