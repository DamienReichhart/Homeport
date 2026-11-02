import { createMemoryRouter, type RouteObject } from 'react-router-dom';
import { AddHostPage, HostDetailPage, HostsPage } from '@/features/hosts';
import { OverviewPage } from '@/features/overview';
import { SettingsPage } from '@/features/settings';
import { AppLayout } from '../AppLayout';

/** Route table. One entry per feature page; keep logic out of here. */
export const routes: RouteObject[] = [
  {
    element: <AppLayout />,
    children: [
      { index: true, element: <OverviewPage /> },
      { path: 'hosts', element: <HostsPage /> },
      { path: 'hosts/new', element: <AddHostPage /> },
      { path: 'hosts/:hostId', element: <HostDetailPage /> },
      { path: 'settings', element: <SettingsPage /> },
    ],
  },
];

// Memory router: a desktop webview has no meaningful URL bar and deep-linking
// into file:// / tauri:// origins is fragile.
export const createAppRouter = (initialEntries: string[] = ['/']) =>
  createMemoryRouter(routes, { initialEntries });
