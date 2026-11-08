import type { ReleaseRevision } from '@/shared/api';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Picks which revision of a release the Values, Notes, Manifest and Resources show. */
export function RevisionPicker({
  history,
  value,
  onChange,
}: {
  history: ReleaseRevision[];
  value: number;
  onChange: (revision: number) => void;
}) {
  return (
    <Select value={String(value)} onValueChange={(v) => onChange(Number(v))}>
      <SelectTrigger size="sm" aria-label={t('helm.revision.pick')} className="w-auto">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {[...history].reverse().map((r) => (
          <SelectItem key={r.revision} value={String(r.revision)}>
            {t('helm.revision.option', {
              revision: r.revision,
              chart: r.chart,
              status: r.status,
            })}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
