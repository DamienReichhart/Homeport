import { NavLink, Outlet } from 'react-router-dom';
import { navItems } from './routes/navigation';

export function AppLayout() {
  return (
    <div className="shell">
      <nav className="sidebar" aria-label="Main">
        {navItems.map((item) => (
          <NavLink key={item.path} to={item.path} end={item.path === '/'}>
            {item.label}
          </NavLink>
        ))}
      </nav>
      <main className="content">
        <Outlet />
      </main>
    </div>
  );
}
