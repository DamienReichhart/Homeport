import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useEffect, useState, type ReactNode } from 'react';
import { HostProvider } from '@/features/hosts';
import { settingsApi } from '@/features/settings';
import { applyTheme } from '@/shared/lib';
import { ErrorDetailsHost } from '@/shared/ui';
import { Toaster } from '@/shared/ui/sonner';
import { TooltipProvider } from '@/shared/ui/tooltip';
import { LiveUpdates } from './LiveUpdates';

export function createQueryClient() {
  return new QueryClient({
    defaultOptions: {
      // Engine data is refreshed by pushed events, so don't refetch on focus.
      queries: { refetchOnWindowFocus: false, retry: 1, staleTime: 10_000 },
    },
  });
}

/** Global providers and bootstrapping. */
export function AppProviders({ children, client }: { children: ReactNode; client?: QueryClient }) {
  const [queryClient] = useState(() => client ?? createQueryClient());
  useEffect(() => {
    settingsApi
      .get()
      .then((s) => applyTheme(s.theme ?? 'system'))
      .catch(() => applyTheme('system'));
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <HostProvider>
          <LiveUpdates />
          {children}
        </HostProvider>
        <Toaster richColors />
        <ErrorDetailsHost />
      </TooltipProvider>
    </QueryClientProvider>
  );
}
