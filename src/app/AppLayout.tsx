import { AnchorIcon, TriangleAlertIcon } from 'lucide-react';
import { NavLink, Outlet } from 'react-router-dom';
import { HostSwitcher, useCapabilities } from '@/features/hosts';
import { cn } from '@/shared/lib';
import { navItems } from './routes/navigation';

export function AppLayout() {
  const capabilities = useCapabilities();
  return (
    <div className="flex h-screen overflow-hidden">
      <nav aria-label="Main" className="bg-sidebar flex w-56 shrink-0 flex-col gap-1 border-r p-3">
        <div className="mb-2 flex items-center gap-2 px-2 py-1 font-semibold">
          <AnchorIcon className="text-primary size-5" /> Homeport
        </div>
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
                'flex items-center gap-2 rounded-md px-3 py-1.5 text-sm',
                isActive ? 'bg-primary text-primary-foreground' : 'hover:bg-sidebar-accent',
              )
            }
          >
            <Icon className="size-4" />
            {label}
          </NavLink>
        ))}
      </nav>
      <main className="flex min-w-0 flex-1 flex-col overflow-hidden">
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
