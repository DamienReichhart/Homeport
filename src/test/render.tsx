import { render } from '@testing-library/react';
import { RouterProvider } from 'react-router-dom';
import { AppProviders, createQueryClient } from '@/app/providers/AppProviders';
import { createAppRouter } from '@/app/routes/router';

/** Renders the whole app at `path`, as the desktop window would. */
export function renderApp(path: string) {
  return render(
    <AppProviders client={createQueryClient()}>
      <RouterProvider router={createAppRouter([path])} />
    </AppProviders>,
  );
}
