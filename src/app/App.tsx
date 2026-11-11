import { useEffect, useState } from 'react';
import { RouterProvider } from 'react-router-dom';
import { commands, type StartupProblem } from '@/shared/api';
import { ErrorDetailsHost, Toaster, TooltipProvider } from '@/shared/ui';
import { AppProviders } from './providers/AppProviders';
import { I18nProvider } from './providers/I18nProvider';
import { createAppRouter } from './router/router';
import { StartupProblemPage } from './StartupProblemPage';

export function App() {
  const [router] = useState(() => createAppRouter());
  // `undefined` while the backend is asked whether it started.
  const [problem, setProblem] = useState<StartupProblem | null>();
  useEffect(() => {
    commands
      .startupProblem()
      .then(setProblem)
      // An answer is only missing when the command itself failed: start the
      // app, whose pages report what they can't reach.
      .catch(() => setProblem(null));
  }, []);

  if (problem === undefined) return null;
  if (problem)
    return (
      <I18nProvider>
        <TooltipProvider>
          <StartupProblemPage problem={problem} />
          <Toaster richColors />
          <ErrorDetailsHost />
        </TooltipProvider>
      </I18nProvider>
    );
  return (
    <AppProviders>
      <RouterProvider router={router} />
    </AppProviders>
  );
}
