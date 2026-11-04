import { QueryClientProvider, type QueryClient } from '@tanstack/react-query';
import { useEffect, useState, type ReactNode } from 'react';
import { HostProvider } from '@/core/host';
import { settingsQuery } from '@/core/settings';
import { applyTheme } from '@/shared/lib';
import { ErrorDetailsHost, Toaster, TooltipProvider } from '@/shared/ui';
import { LiveUpdates } from './LiveUpdates';
import { createQueryClient } from './queryClient';

/** Global providers and bootstrapping. */
export function AppProviders({ children, client }: { children: ReactNode; client?: QueryClient }) {
  const [queryClient] = useState(() => client ?? createQueryClient());
  useEffect(() => {
    queryClient
      .fetchQuery(settingsQuery())
      .then((s) => applyTheme(s.theme ?? 'system'))
      .catch(() => applyTheme('system'));
  }, [queryClient]);

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
