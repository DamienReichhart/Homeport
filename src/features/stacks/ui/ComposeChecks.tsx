import { CircleXIcon, TriangleAlertIcon } from 'lucide-react';
import type { ComposeCheck } from '@/shared/api';
import { t } from '@/shared/i18n';

/** The errors and warnings found in a compose definition, if any. */
export function ComposeChecks({ check }: { check: ComposeCheck | null }) {
  if (!check || (check.errors.length === 0 && check.warnings.length === 0)) return null;
  return (
    <ul className="grid gap-2 text-sm" aria-label={t('stacks.editor.problems')}>
      {check.errors.map((e) => (
        <li key={e} className="text-destructive flex gap-2">
          <CircleXIcon className="mt-0.5 size-4 shrink-0" /> {e}
        </li>
      ))}
      {check.warnings.map((w) => (
        <li key={w} className="text-warning flex gap-2">
          <TriangleAlertIcon className="mt-0.5 size-4 shrink-0" /> {w}
        </li>
      ))}
    </ul>
  );
}
