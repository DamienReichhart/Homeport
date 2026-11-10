import { cn, formatPercent } from '@/shared/lib';
import type { Measure } from '../../model/metrics';
import { t, type MessageKey } from '@/shared/i18n';

const SHARE: Record<Measure, MessageKey> = {
  limit: 'core.metrics.ofLimit',
  request: 'core.metrics.ofRequest',
  allocatable: 'core.metrics.ofAllocatable',
};

/** From this share of a limit or the allocatable up, use shows as a warning. */
const HIGH = 0.9;

/**
 * A list's cell of what a Pod or Node uses of CPU or memory, and its share
 * of what it's measured against (a limit, a request, the allocatable).
 */
export function UsageCell({
  used,
  measure,
  format,
}: {
  /** `undefined` while there's no sample of it. */
  used: number | undefined;
  measure: { kind: Measure; value: number } | null;
  format: (value: number) => string;
}) {
  if (used === undefined) return <span className="text-muted-foreground">—</span>;
  const ratio = measure && measure.value > 0 ? used / measure.value : null;
  return (
    <span className="flex flex-col leading-tight tabular-nums">
      <span>{format(used)}</span>
      {measure && ratio !== null && (
        <span
          className={cn(
            'text-xs',
            ratio >= HIGH && measure.kind !== 'request' ? 'text-warning' : 'text-muted-foreground',
          )}
        >
          {t(SHARE[measure.kind], { percent: formatPercent(ratio * 100) })}
        </span>
      )}
    </span>
  );
}
