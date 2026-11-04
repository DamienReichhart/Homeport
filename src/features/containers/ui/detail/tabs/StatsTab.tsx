import type { AlignedData } from 'uplot';
import { formatBytes, formatPercent } from '@/shared/lib';
import { Card, CardContent, CardHeader, CardTitle, TimeChart } from '@/shared/ui';
import { useStats } from '../../../api/streams';
import { rates } from '../../../model/stats';

export function StatsTab({
  hostId,
  containerId,
  running,
}: {
  hostId: string;
  containerId: string;
  running: boolean;
}) {
  const points = useStats(hostId, containerId, running);

  if (!running)
    return (
      <p className="text-muted-foreground text-sm">
        Start the container to see its resource usage.
      </p>
    );
  const last = points.at(-1)?.sample;
  const times = points.map((p) => p.time);
  const cpu: AlignedData = [times, points.map((p) => p.sample.cpuPercent ?? 0)];
  const memory: AlignedData = [times, points.map((p) => p.sample.memoryUsage)];
  const network: AlignedData = [
    times,
    rates(points, (s) => s.netRx),
    rates(points, (s) => s.netTx),
  ];
  const disk: AlignedData = [
    times,
    rates(points, (s) => s.blockRead),
    rates(points, (s) => s.blockWrite),
  ];

  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>
            CPU{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatPercent(last.cpuPercent ?? 0)}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={cpu}
            series={[{ label: 'CPU', colour: 'var(--chart-1)' }]}
            format={formatPercent}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Memory{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatBytes(last.memoryUsage)} of {formatBytes(last.memoryLimit)}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={memory}
            series={[{ label: 'Used', colour: 'var(--chart-2)' }]}
            format={(v) => formatBytes(v)}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Network{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatBytes(last.netRx)} in, {formatBytes(last.netTx)} out
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={network}
            series={[
              { label: 'Received', colour: 'var(--chart-3)' },
              { label: 'Sent', colour: 'var(--chart-4)', dashed: true },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Disk I/O{' '}
            {last && (
              <span className="text-muted-foreground font-normal">· {last.pids} processes</span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={disk}
            series={[
              { label: 'Read', colour: 'var(--chart-5)', dashed: true },
              { label: 'Written', colour: 'var(--chart-1)' },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
    </div>
  );
}
