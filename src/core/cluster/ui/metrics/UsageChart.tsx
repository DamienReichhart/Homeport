import { useMemo } from 'react';
import type { ResourceUsage, UsageSample } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, TimeChart } from '@/shared/ui';
import { chartData, usageFormat } from '../../model/metrics';
import { t } from '@/shared/i18n';

/** A flat line on a chart: a request, a limit, the allocatable. */
export interface ChartLine {
  label: string;
  value: number;
}

const USED_COLOUR: Record<keyof ResourceUsage, string> = {
  cpu: 'var(--chart-1)',
  memory: 'var(--chart-2)',
};
const LINE_COLOURS = ['var(--chart-3)', 'var(--chart-5)'];

/**
 * A card charting what was used of CPU or memory over the samples read
 * since the page opened, with a dashed line for each of `lines`.
 */
export function UsageChart({
  title,
  samples,
  resource,
  lines,
}: {
  title: string;
  samples: UsageSample[];
  resource: keyof ResourceUsage;
  lines: ChartLine[];
}) {
  const format = usageFormat(resource);
  const last = samples.at(-1);
  const data = useMemo(
    () =>
      chartData(
        samples,
        resource,
        lines.map((l) => l.value),
      ),
    [samples, resource, lines],
  );
  return (
    <Card>
      <CardHeader>
        <CardTitle>
          {title}{' '}
          {last && (
            <span className="text-muted-foreground font-normal">
              · {format(last.usage[resource] ?? 0)}
            </span>
          )}
        </CardTitle>
      </CardHeader>
      <CardContent>
        <TimeChart
          data={data}
          series={[
            { label: t('core.metrics.used'), colour: USED_COLOUR[resource] },
            ...lines.map((line, i) => ({
              label: line.label,
              colour: LINE_COLOURS[i % LINE_COLOURS.length] ?? 'var(--chart-4)',
              dashed: true,
            })),
          ]}
          format={format}
        />
      </CardContent>
    </Card>
  );
}
