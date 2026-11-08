import { KIND_BADGE, KIND_LABEL, STATE_LABEL, STATE_TONE } from '@/features/stacks';
import type { Stack } from '@/shared/api';
import { Badge, StatusDot, type ColumnDef } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The columns of a Swarm's Stacks, like the stacks list's. */
export function swarmStackColumns(): ColumnDef<Stack, unknown>[] {
  return [
    {
      id: 'name',
      header: t('stacks.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => (
        <span className="grid">
          <span className="font-medium">{row.original.name}</span>
          {row.original.workingDir && (
            <span className="text-muted-foreground font-mono text-xs">
              {row.original.workingDir}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'kind',
      header: t('stacks.column.kind'),
      accessorFn: (s) => s.kind,
      cell: ({ row }) => (
        <Badge variant={KIND_BADGE[row.original.kind]}>{t(KIND_LABEL[row.original.kind])}</Badge>
      ),
    },
    {
      id: 'state',
      header: t('stacks.column.state'),
      accessorFn: (s) => s.state,
      cell: ({ row }) => (
        <span className="flex items-center gap-1.5 text-sm">
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
        <span className="text-muted-foreground text-sm">
          {row.original.services.map((x) => x.name).join(', ') || '—'}
        </span>
      ),
    },
  ];
}
