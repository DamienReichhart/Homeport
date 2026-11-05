import { CpuIcon, MemoryStickIcon } from 'lucide-react';
import { useUsage } from '@/features/containers';
import { formatBytes, formatPercent } from '@/shared/lib';
import { t } from '@/shared/i18n';

/** Live CPU and memory use of a Host's running containers against its capacity. */
export function HostUsage({
  hostId,
  cpus,
  memory,
}: {
  hostId: string;
  cpus: number;
  memory: number;
}) {
  const usage = [...useUsage(hostId).values()];
  const cpu = usage.reduce((sum, u) => sum + (u.cpuPercent ?? 0), 0);
  const used = usage.reduce((sum, u) => sum + u.memoryUsage, 0);
  return (
    <>
      <span className="flex items-center gap-1" title={t('overview.host.cpuTitle')}>
        <CpuIcon className="size-3.5" />{' '}
        {t('overview.host.cpu', { percent: formatPercent(cpu), cpus })}
      </span>
      <span className="flex items-center gap-1" title={t('overview.host.memoryTitle')}>
        <MemoryStickIcon className="size-3.5" /> {formatBytes(used)} / {formatBytes(memory)}
      </span>
    </>
  );
}
