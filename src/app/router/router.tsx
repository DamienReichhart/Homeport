import { createMemoryRouter, type RouteObject } from 'react-router-dom';
import { AppLayout } from '../layout/AppLayout';
import { CrashPage } from './CrashPage';
import { features } from './features';

/** Route table, assembled from the features' manifests; keep logic out of here. */
export const routes: RouteObject[] = [
  {
    element: <AppLayout />,
    errorElement: <CrashPage />,
    children: [
      {
        // Pages crash inside the layout, so the sidebar stays usable.
        errorElement: <CrashPage />,
        children: features.flatMap((f) => f.routes),
      },
    ],
  },
];

// Memory router: a desktop webview has no meaningful URL bar and deep-linking
// into file:// / tauri:// origins is fragile.
export const createAppRouter = (initialEntries: string[] = ['/']) =>
  createMemoryRouter(routes, { initialEntries });
