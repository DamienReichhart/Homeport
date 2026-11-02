import { useEffect, type ReactNode } from 'react';
import { settingsApi } from '@/features/settings';
import { applyTheme } from '@/shared/lib';

/** Global providers/bootstrapping. Add context providers here as the app grows. */
export function AppProviders({ children }: { children: ReactNode }) {
  useEffect(() => {
    settingsApi
      .get()
      .then((s) => applyTheme(s.theme ?? 'system'))
      .catch(() => applyTheme('system'));
  }, []);

  return <>{children}</>;
}
