import { CpuIcon, MemoryStickIcon } from 'lucide-react';
import { useUsage } from '@/features/containers';
import { formatBytes, formatPercent } from '@/shared/lib';

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
      <span className="flex items-center gap-1" title="Container CPU use, 100% per CPU">
        <CpuIcon className="size-3.5" /> {formatPercent(cpu)} of {cpus} CPUs
      </span>
      <span className="flex items-center gap-1" title="Container memory use">
        <MemoryStickIcon className="size-3.5" /> {formatBytes(used)} / {formatBytes(memory)}
      </span>
    </>
  );
}
