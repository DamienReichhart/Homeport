import { MetricsNotice, UsageChart, type ChartLine, type MetricsState } from '@/core/cluster';
import type { ClusterNodeView } from '@/shared/api';
import { t } from '@/shared/i18n';

const allocatable = (value: number | null): ChartLine[] =>
  value ? [{ label: t('core.metrics.allocatable'), value }] : [];

/**
 * The Metrics tab: the Node's CPU and memory since the page opened, against
 * what it can allocate to Pods, from metrics-server.
 */
export function NodeMetrics({ node, metrics }: { node: ClusterNodeView; metrics: MetricsState }) {
  const samples = metrics.samples;
  const notice = (
    <MetricsNotice metrics={metrics} waiting={samples === null && !metrics.unavailable} />
  );
  if (metrics.unavailable || !samples) return notice;
  return (
    <div className="grid gap-4">
      {notice}
      <div className="grid gap-4 lg:grid-cols-2">
        <UsageChart
          title={t('core.metrics.cpu')}
          samples={samples}
          resource="cpu"
          lines={allocatable(node.cpuMillicores)}
        />
        <UsageChart
          title={t('core.metrics.memory')}
          samples={samples}
          resource="memory"
          lines={allocatable(node.memoryBytes)}
        />
      </div>
      <p className="text-muted-foreground text-xs">{t('core.metrics.kept')}</p>
    </div>
  );
}
