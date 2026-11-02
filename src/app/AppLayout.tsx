import { NavLink, Outlet } from 'react-router-dom';
import { cn } from '@/shared/lib';
import { navItems } from './routes/navigation';

export function AppLayout() {
  return (
    <div className="flex min-h-screen">
      <nav aria-label="Main" className="bg-sidebar flex w-52 flex-col gap-1 border-r p-3">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            end={item.path === '/'}
            className={({ isActive }) =>
              cn(
                'rounded-md px-3 py-2 text-sm',
                isActive ? 'bg-primary text-primary-foreground' : 'hover:bg-sidebar-accent',
              )
            }
          >
            {item.label}
          </NavLink>
        ))}
      </nav>
      <main className="flex-1 p-6">
        <Outlet />
      </main>
    </div>
  );
}
