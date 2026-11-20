import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { resourceQueries } from '@/core/host';
import { useUsage } from '@/features/containers';
import { formatBytes, formatPercent } from '@/shared/lib';
import { t } from '@/shared/i18n';

/**
 * The four numbers people open a Host's Overview for, each a link to the
 * page that explains it: containers running, CPU, memory and images.
 */
export function HostVitals({ hostId }: { hostId: string }) {
  const info = useQuery(resourceQueries.systemInfo(hostId));
  const usage = [...useUsage(hostId).values()];
  if (!info.data) return null;
  const cpu = usage.reduce((sum, u) => sum + (u.cpuPercent ?? 0), 0);
  const memory = usage.reduce((sum, u) => sum + u.memoryUsage, 0);
  return (
    <ul
      aria-label={t('overview.vitals.title')}
      className="bg-card grid grid-cols-2 divide-y rounded-md border sm:grid-cols-4 sm:divide-x sm:divide-y-0 rtl:sm:divide-x-reverse"
    >
      <Vital to="/containers" label={t('overview.vitals.containers')}>
        {t('overview.vitals.running', { running: info.data.running, total: info.data.containers })}
      </Vital>
      <Vital to="/containers" label={t('overview.vitals.cpu')} title={t('overview.host.cpuTitle')}>
        {t('overview.host.cpu', { percent: formatPercent(cpu), cpus: info.data.cpus })}
      </Vital>
      <Vital
        to="/containers"
        label={t('overview.vitals.memory')}
        title={t('overview.host.memoryTitle')}
      >
        {formatBytes(memory)} / {formatBytes(info.data.memory)}
      </Vital>
      <Vital to="/images" label={t('overview.vitals.images')}>
        {info.data.images}
      </Vital>
    </ul>
  );
}

function Vital({
  to,
  label,
  title,
  children,
}: {
  to: string;
  label: string;
  title?: string;
  children: ReactNode;
}) {
  return (
    <li className="grid">
      <Link
        to={to}
        title={title}
        className="hover:bg-selected focus-visible:ring-ring/50 grid gap-0.5 px-4 py-3 transition-colors outline-none focus-visible:ring-3 focus-visible:ring-inset"
      >
        <span className="text-muted-foreground text-xs">{label}</span>
        <span className="text-base font-medium tabular-nums">{children}</span>
      </Link>
    </li>
  );
}
