import { render } from '@testing-library/react';
import { RouterProvider } from 'react-router-dom';
import { AppProviders } from '@/app/providers/AppProviders';
import { createQueryClient } from '@/app/providers/queryClient';
import { createAppRouter } from '@/app/router/router';

/** Renders the whole app at `path`, as the desktop window would. */
export function renderApp(path: string) {
  return render(
    <AppProviders client={createQueryClient()}>
      <RouterProvider router={createAppRouter([path])} />
    </AppProviders>,
  );
}
