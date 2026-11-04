import type { QueryClient } from '@tanstack/react-query';
import { render } from '@testing-library/react';
import type { ReactNode } from 'react';
import { RouterProvider } from 'react-router-dom';
import { AppProviders } from '@/app/providers/AppProviders';
import { createQueryClient } from '@/app/providers/queryClient';
import { createAppRouter } from '@/app/router/router';

/** Renders the whole app at `path`, as the desktop window would. */
export function renderApp(path: string, client: QueryClient = createQueryClient()) {
  return render(
    <AppProviders client={client}>
      <RouterProvider router={createAppRouter([path])} />
    </AppProviders>,
  );
}

/** The app's query client without its one retry, so a failure shows at once. */
export function clientWithoutRetries(): QueryClient {
  const client = createQueryClient();
  client.setDefaultOptions({
    ...client.getDefaultOptions(),
    queries: { ...client.getDefaultOptions().queries, retry: false },
  });
  return client;
}

/** Renders one component with the app's providers, outside the router. */
export function renderWithProviders(ui: ReactNode, client: QueryClient = clientWithoutRetries()) {
  return render(<AppProviders client={client}>{ui}</AppProviders>);
}
