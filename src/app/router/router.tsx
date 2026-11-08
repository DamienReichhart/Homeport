import { createMemoryRouter, Outlet, type RouteObject } from 'react-router-dom';
import { featureScope, type FeatureManifest } from '@/core/feature';
import { AppLayout } from '../layout/AppLayout';
import { CrashPage } from './CrashPage';
import { features } from './features';
import { ScopeGuard } from './ScopeGuard';

/** A feature's routes, behind a guard when its pages need a Host or a Cluster. */
function scopedRoutes(feature: FeatureManifest): RouteObject[] {
  const scope = featureScope(feature);
  if (scope === 'app') return feature.routes;
  return [
    {
      element: (
        <ScopeGuard scope={scope}>
          <Outlet />
        </ScopeGuard>
      ),
      children: feature.routes,
    },
  ];
}

/** Route table, assembled from the features' manifests; keep logic out of here. */
export const routes: RouteObject[] = [
  {
    element: <AppLayout />,
    errorElement: <CrashPage />,
    children: [
      {
        // Pages crash inside the layout, so the sidebar stays usable.
        errorElement: <CrashPage />,
        children: features.flatMap(scopedRoutes),
      },
    ],
  },
];

// Memory router: a desktop webview has no meaningful URL bar and deep-linking
// into file:// / tauri:// origins is fragile.
export const createAppRouter = (initialEntries: string[] = ['/']) =>
  createMemoryRouter(routes, { initialEntries });
