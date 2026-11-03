import { createMemoryRouter, type RouteObject } from 'react-router-dom';
import { ContainerDetailPage, ContainersPage } from '@/features/containers';
import { EventsPage } from '@/features/events';
import { AddHostPage, HostDetailPage, HostsPage } from '@/features/hosts';
import { ImagesPage } from '@/features/images';
import { NetworksPage } from '@/features/networks';
import { OverviewPage } from '@/features/overview';
import { SettingsPage } from '@/features/settings';
import { NewStackPage, StackDetailPage, StacksPage } from '@/features/stacks';
import { VolumesPage } from '@/features/volumes';
import { AppLayout } from '../AppLayout';
import { CrashPage } from './CrashPage';

/** Route table. One entry per feature page; keep logic out of here. */
export const routes: RouteObject[] = [
  {
    element: <AppLayout />,
    errorElement: <CrashPage />,
    children: [
      {
        // Pages crash inside the layout, so the sidebar stays usable.
        errorElement: <CrashPage />,
        children: [
          { index: true, element: <OverviewPage /> },
          { path: 'hosts', element: <HostsPage /> },
          { path: 'hosts/new', element: <AddHostPage /> },
          { path: 'hosts/:hostId', element: <HostDetailPage /> },
          { path: 'containers', element: <ContainersPage /> },
          { path: 'containers/:id/:tab?', element: <ContainerDetailPage /> },
          { path: 'images', element: <ImagesPage /> },
          { path: 'volumes', element: <VolumesPage /> },
          { path: 'networks', element: <NetworksPage /> },
          { path: 'stacks', element: <StacksPage /> },
          { path: 'stacks/new', element: <NewStackPage /> },
          { path: 'stacks/:name', element: <StackDetailPage /> },
          { path: 'events', element: <EventsPage /> },
          { path: 'settings', element: <SettingsPage /> },
        ],
      },
    ],
  },
];

// Memory router: a desktop webview has no meaningful URL bar and deep-linking
// into file:// / tauri:// origins is fragile.
export const createAppRouter = (initialEntries: string[] = ['/']) =>
  createMemoryRouter(routes, { initialEntries });
