import { MetricsNotice, UsageChart, type ChartLine, type MetricsState } from '@/core/cluster';
import type { PodContainer, PodDetail, ResourceBounds } from '@/shared/api';
import { t } from '@/shared/i18n';

/** A container's request and limit, as lines on its chart. */
function lines(bounds: ResourceBounds | undefined): ChartLine[] {
  const out: ChartLine[] = [];
  if (bounds?.request) out.push({ label: t('core.metrics.request'), value: bounds.request });
  if (bounds?.limit) out.push({ label: t('core.metrics.limit'), value: bounds.limit });
  return out;
}

/**
 * The Metrics tab: each container's CPU and memory since the page opened,
 * against its request and limit, from metrics-server.
 */
export function PodMetrics({ pod, metrics }: { pod: PodDetail; metrics: MetricsState }) {
  const history = metrics.containers;
  const notice = (
    <MetricsNotice metrics={metrics} waiting={history === null && !metrics.unavailable} />
  );
  if (metrics.unavailable || !history) return notice;
  const spec = new Map<string, PodContainer>(pod.containers.map((c) => [c.name, c]));
  // In the Pod's order, the containers metrics-server reports.
  const order = pod.containers.map((c) => c.name);
  const containers = [...history].sort(
    (a, b) =>
      (order.indexOf(a.name) + 1 || order.length + 1) -
      (order.indexOf(b.name) + 1 || order.length + 1),
  );
  return (
    <div className="grid gap-4">
      {notice}
      {containers.length === 0 && (
        <p className="text-muted-foreground text-sm">{t('pods.metrics.noSample')}</p>
      )}
      {containers.map((c) => (
        <section key={c.name} className="grid gap-2" aria-label={c.name}>
          {containers.length > 1 && <h3 className="text-sm font-medium">{c.name}</h3>}
          <div className="grid gap-4 lg:grid-cols-2">
            <UsageChart
              title={t('core.metrics.cpu')}
              samples={c.samples}
              resource="cpu"
              lines={lines(spec.get(c.name)?.cpu)}
            />
            <UsageChart
              title={t('core.metrics.memory')}
              samples={c.samples}
              resource="memory"
              lines={lines(spec.get(c.name)?.memory)}
            />
          </div>
        </section>
      ))}
      <p className="text-muted-foreground text-xs">{t('core.metrics.kept')}</p>
    </div>
  );
}
