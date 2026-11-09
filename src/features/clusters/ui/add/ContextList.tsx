import type { ContextRow } from '../../model/importRows';
import { importable } from '../../model/importRows';
import { authLabel } from '../../model/labels';
import { Badge, Button, Checkbox, ErrorMessage, Input } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * The contexts of a kubeconfig, each with a checkbox and the name its
 * Cluster gets; one already added offers a refresh instead.
 */
export function ContextList({
  rows,
  onChange,
  onRefresh,
  refreshing,
}: {
  rows: ContextRow[];
  onChange: (rows: ContextRow[]) => void;
  onRefresh: (clusterId: string) => void;
  refreshing: boolean;
}) {
  const update = (i: number, patch: Partial<ContextRow>) =>
    onChange(rows.map((r, j) => (i === j ? { ...r, ...patch } : r)));
  return (
    <ul className="divide-border grid divide-y rounded-md border">
      {rows.map((row, i) => {
        const { context } = row;
        const id = `context-${i}`;
        return (
          <li key={context.name} className="grid gap-2 p-3">
            <div className="flex flex-wrap items-center gap-3">
              <Checkbox
                id={id}
                checked={row.picked}
                disabled={!importable(context)}
                onCheckedChange={(v) => update(i, { picked: v === true })}
                aria-label={t('clusters.import.pick', { context: context.name })}
              />
              <label htmlFor={id} className="font-medium">
                {context.name}
              </label>
              {context.current && <Badge variant="secondary">{t('clusters.import.current')}</Badge>}
              {context.auth && <Badge variant="outline">{authLabel(context.auth)}</Badge>}
              <span className="text-muted-foreground truncate font-mono text-xs">
                {context.server}
                {context.namespace && ` · ${context.namespace}`}
              </span>
            </div>
            {context.added ? (
              <div className="text-muted-foreground flex items-center gap-2 ps-7 text-sm">
                {t('clusters.import.added', { name: context.added.name })}
                <Button
                  size="sm"
                  variant="outline"
                  disabled={refreshing}
                  onClick={() => context.added && onRefresh(context.added.id)}
                >
                  {t('clusters.import.refresh')}
                </Button>
              </div>
            ) : context.problem ? (
              <div className="ps-7">
                <ErrorMessage error={context.problem} />
              </div>
            ) : (
              row.picked && (
                <div className="ps-7">
                  <Input
                    aria-label={t('clusters.import.name', { context: context.name })}
                    value={row.name}
                    onChange={(e) => update(i, { name: e.target.value })}
                    className="max-w-sm"
                  />
                </div>
              )
            )}
          </li>
        );
      })}
    </ul>
  );
}
