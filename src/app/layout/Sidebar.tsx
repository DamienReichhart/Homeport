import { NavLink } from 'react-router-dom';
import { ContainerdNamespacePicker, IncusProjectPicker } from '@/core/host';
import { HostSwitcher } from '@/features/hosts';
import { SwarmNav } from '@/features/swarm';
import { cn } from '@/shared/lib';
import { Logo, WithShortcut } from '@/shared/ui';
import { useNavEntries } from '../shortcuts/navShortcuts';
import { t } from '@/shared/i18n';

export function Sidebar() {
  const { entries, shortcuts } = useNavEntries();
  return (
    <nav
      aria-label={t('app.nav.main')}
      className="bg-sidebar text-sidebar-foreground border-sidebar-border flex w-56 shrink-0 flex-col gap-0.5 border-r p-3"
    >
      <Logo className="mb-3 px-1.5 pt-1" />
      <div className="mb-3 grid gap-1.5">
        <HostSwitcher />
        <ContainerdNamespacePicker />
        <IncusProjectPicker />
      </div>
      {entries.map(({ path, label, icon: Icon }) => {
        const link = (
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
            {t(label)}
          </NavLink>
        );
        const keys = shortcuts.find((s) => s.entry.path === path)?.keys;
        return keys ? (
          <WithShortcut key={path} keys={keys} label={t(label)} side="right">
            {link}
          </WithShortcut>
        ) : (
          link
        );
      })}
      <SwarmNav />
    </nav>
  );
}
