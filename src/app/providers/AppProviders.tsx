import { QueryClientProvider, type QueryClient } from '@tanstack/react-query';
import { useEffect, useState, type ReactNode } from 'react';
import { settingsQuery } from '@/core/settings';
import { ShortcutsProvider } from '@/shared/hooks';
import { applyTheme } from '@/shared/lib';
import { ErrorDetailsHost, Toaster, TooltipProvider } from '@/shared/ui';
import { I18nProvider } from './I18nProvider';
import { LiveUpdates } from './LiveUpdates';
import { SelectionRoot } from './SelectionRoot';
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
    <I18nProvider>
      <QueryClientProvider client={queryClient}>
        <TooltipProvider>
          <ShortcutsProvider>
            <SelectionRoot>
              <LiveUpdates />
              {children}
            </SelectionRoot>
          </ShortcutsProvider>
          <Toaster richColors />
          <ErrorDetailsHost />
        </TooltipProvider>
      </QueryClientProvider>
    </I18nProvider>
  );
}
