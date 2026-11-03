import { TriangleAlertIcon } from 'lucide-react';
import { NavLink, Outlet } from 'react-router-dom';
import { HostSwitcher } from '@/features/hosts';
import { useCapabilities } from '@/shared/api';
import { UpdateNotice } from '@/features/updates';
import { cn } from '@/shared/lib';
import { Logo } from '@/shared/ui';
import { navItems } from './routes/navigation';

export function AppLayout() {
  const capabilities = useCapabilities();
  return (
    <div className="flex h-screen overflow-hidden">
      <nav
        aria-label="Main"
        className="bg-sidebar text-sidebar-foreground border-sidebar-border flex w-56 shrink-0 flex-col gap-0.5 border-r p-3"
      >
        <Logo className="mb-3 px-1.5 pt-1" />
        <div className="mb-3">
          <HostSwitcher />
        </div>
        {navItems.map(({ path, label, icon: Icon }) => (
          <NavLink
            key={path}
            to={path}
            end={path === '/'}
            className={({ isActive }) =>
              cn(
                'group flex items-center gap-2.5 rounded-md px-3 py-1.5 text-sm transition-colors',
                isActive
                  ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
                  : 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground',
              )
            }
          >
            <Icon className="text-muted-foreground group-aria-[current=page]:text-sidebar-primary size-4" />
            {label}
          </NavLink>
        ))}
      </nav>
      <main className="flex min-w-0 flex-1 flex-col overflow-hidden">
        <UpdateNotice />
        {capabilities.data && !capabilities.data.keychain && (
          <div
            role="alert"
            className="bg-destructive/10 text-destructive flex items-center gap-2 border-b px-6 py-2 text-sm"
          >
            <TriangleAlertIcon className="size-4" />
            No OS keychain is available, so SSH keys, TLS certificates and stack environments can’t
            be stored. Local, TCP and ssh-agent hosts still work.
          </div>
        )}
        <div className="min-h-0 flex-1 overflow-auto p-6">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
