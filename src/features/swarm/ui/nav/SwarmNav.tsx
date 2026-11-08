import { NetworkIcon } from 'lucide-react';
import { NavLink } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { swarmLabel, useSwarms } from '@/core/swarm';
import { cn } from '@/shared/lib';
import { t } from '@/shared/i18n';

/** The sidebar's Swarms section, shown only once a Host belongs to one. */
export function SwarmNav() {
  const swarms = useSwarms();
  const hosts = useHosts();
  if (!swarms.data?.length) return null;
  return (
    <div className="mt-3 grid gap-0.5">
      <p className="text-muted-foreground px-3 pb-1 text-xs font-medium">{t('swarm.nav')}</p>
      {swarms.data.map((swarm) => (
        <NavLink
          key={swarm.id}
          to={`/swarms/${swarm.id}`}
          className={({ isActive }) =>
            cn(
              'group flex items-center gap-2.5 rounded-md px-3 py-1.5 text-sm transition-colors',
              isActive
                ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
                : 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground',
            )
          }
        >
          <NetworkIcon className="text-muted-foreground group-aria-[current=page]:text-sidebar-primary size-4" />
          <span className="truncate">{swarmLabel(swarm, hosts.data)}</span>
        </NavLink>
      ))}
    </div>
  );
}
