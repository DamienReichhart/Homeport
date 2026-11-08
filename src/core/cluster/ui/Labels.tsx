import { Badge } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A resource's labels or annotations, as `key=value` chips. */
export function Labels({ values }: { values: Partial<Record<string, string>> }) {
  const entries = Object.entries(values);
  if (entries.length === 0)
    return <span className="text-muted-foreground text-sm">{t('core.labels.none')}</span>;
  return (
    <div className="flex flex-wrap gap-1">
      {entries.map(([k, v]) => (
        <Badge key={k} variant="outline" className="max-w-full font-mono text-xs break-all">
          {k}={v}
        </Badge>
      ))}
    </div>
  );
}
