import type { AlignedData } from 'uplot';
import { formatBytes, formatPercent } from '@/shared/lib';
import { Card, CardContent, CardHeader, CardTitle, TimeChart } from '@/shared/ui';
import { useStats } from '../../../api/streams';
import { rates } from '../../../model/stats';
import { t } from '@/shared/i18n';

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
    return <p className="text-muted-foreground text-sm">{t('containers.stats.startFirst')}</p>;
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
            {t('containers.stats.cpu')}{' '}
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
            series={[{ label: t('containers.stats.cpu'), colour: 'var(--chart-1)' }]}
            format={formatPercent}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            {t('containers.stats.memory')}{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                {t('containers.stats.memoryNow', {
                  used: formatBytes(last.memoryUsage),
                  limit: formatBytes(last.memoryLimit),
                })}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={memory}
            series={[{ label: t('containers.stats.used'), colour: 'var(--chart-2)' }]}
            format={(v) => formatBytes(v)}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            {t('containers.stats.network')}{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                {t('containers.stats.networkNow', {
                  received: formatBytes(last.netRx),
                  sent: formatBytes(last.netTx),
                })}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={network}
            series={[
              { label: t('containers.stats.received'), colour: 'var(--chart-3)' },
              { label: t('containers.stats.sent'), colour: 'var(--chart-4)', dashed: true },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            {t('containers.stats.disk')}{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                {t('containers.stats.processes', { count: last.pids })}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={disk}
            series={[
              { label: t('containers.stats.read'), colour: 'var(--chart-5)', dashed: true },
              { label: t('containers.stats.written'), colour: 'var(--chart-1)' },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
    </div>
  );
}
