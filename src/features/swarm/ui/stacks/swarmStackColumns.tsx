import { KIND_BADGE, KIND_LABEL, STATE_LABEL, STATE_TONE } from '@/features/stacks';
import type { Stack } from '@/shared/api';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A Stack's kind, outlined: naming it isn't a state that needs colour. */
export const kindBadge = (kind: Stack['kind']) =>
  KIND_BADGE[kind] === 'default' ? 'outline' : KIND_BADGE[kind];

/** The columns of a Swarm's Stacks, like the stacks list's. */
export function swarmStackColumns(): ColumnDef<Stack, unknown>[] {
  return [
    {
      id: 'name',
      header: t('stacks.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => (
        <span className="font-medium" title={row.original.workingDir ?? undefined}>
          {row.original.name}
        </span>
      ),
    },
    {
      id: 'kind',
      header: t('stacks.column.kind'),
      accessorFn: (s) => s.kind,
      cell: ({ row }) => (
        <Badge variant={kindBadge(row.original.kind)}>{t(KIND_LABEL[row.original.kind])}</Badge>
      ),
    },
    {
      id: 'state',
      header: t('stacks.column.state'),
      accessorFn: (s) => s.state,
      cell: ({ row }) => (
        <span className="flex items-center gap-1.5 text-sm whitespace-nowrap">
          <StatusDot tone={STATE_TONE[row.original.state]} />
          {t(STATE_LABEL[row.original.state])}
          {row.original.total > 0 && (
            <span className="text-muted-foreground">
              · {row.original.running}/{row.original.total}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'services',
      header: t('stacks.column.services'),
      accessorFn: (s) => s.services.map((x) => x.name).join(', '),
      cell: ({ row }) => (
        <span
          className="text-muted-foreground block max-w-72 truncate text-sm"
          title={row.original.services.map((x) => x.name).join(', ') || undefined}
        >
          {row.original.services.map((x) => x.name).join(', ') || '—'}
        </span>
      ),
    },
  ];
}
