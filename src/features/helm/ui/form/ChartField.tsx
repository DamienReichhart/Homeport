import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import {
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  Spinner,
} from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

const OTHER = '__other__';

/**
 * The chart a release is upgraded from: the repository charts with its
 * chart's name (those that have its current version first), or another
 * typed as `repository/chart` or `oci://…`, which is what an OCI release
 * needs since helm doesn't record where a chart came from.
 */
export function ChartField({
  chartName,
  chartVersion,
  value,
  onChange,
}: {
  chartName: string;
  chartVersion: string;
  value: string;
  onChange: (chart: string) => void;
}) {
  const candidates = useQuery(helmQueries.chartCandidates(chartName, chartVersion));
  const [typing, setTyping] = useState(false);
  if (candidates.isPending) return <Spinner />;
  const found = candidates.data ?? [];
  const other = typing || !found.includes(value);
  const chosen = other ? OTHER : value;
  return (
    <div className="grid gap-2">
      <Label htmlFor="helm-chart">{t('helm.upgrade.chart')}</Label>
      {found.length > 0 && (
        <Select
          value={chosen}
          onValueChange={(v) => {
            setTyping(v === OTHER);
            onChange(v === OTHER ? '' : v);
          }}
        >
          <SelectTrigger id={other ? undefined : 'helm-chart'} className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {found.map((c) => (
              <SelectItem key={c} value={c}>
                {c}
              </SelectItem>
            ))}
            <SelectSeparator />
            <SelectItem value={OTHER}>{t('helm.upgrade.otherChart')}</SelectItem>
          </SelectContent>
        </Select>
      )}
      {other && (
        <Input
          id="helm-chart"
          value={value}
          placeholder={t('helm.upgrade.chartPlaceholder', { chart: chartName })}
          onChange={(e) => onChange(e.target.value.trim())}
        />
      )}
      {found.length === 0 && (
        <p className="text-muted-foreground text-sm">
          {t('helm.upgrade.noCandidate', { chart: chartName })}
        </p>
      )}
    </div>
  );
}
