import { createMemoryRouter, Outlet, type RouteObject } from 'react-router-dom';
import { featureScope, type FeatureManifest } from '@/core/feature';
import { HostCapabilityGuard } from '@/core/host';
import { AppLayout } from '../layout/AppLayout';
import { CrashPage } from './CrashPage';
import { features } from './features';
import { ScopeGuard } from './ScopeGuard';

/**
 * A feature's routes, behind a guard when its pages need a Host or a
 * Cluster, and one more when they need something of the Host's Engine.
 */
function scopedRoutes(feature: FeatureManifest): RouteObject[] {
  const scope = featureScope(feature);
  if (scope === 'app') return feature.routes;
  const outlet =
    scope === 'host' && feature.requires ? (
      <HostCapabilityGuard requires={feature.requires}>
        <Outlet />
      </HostCapabilityGuard>
    ) : (
      <Outlet />
    );
  return [
    {
      element: <ScopeGuard scope={scope}>{outlet}</ScopeGuard>,
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
