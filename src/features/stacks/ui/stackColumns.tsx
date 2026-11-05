import { TriangleAlertIcon } from 'lucide-react';
import type { Stack } from '@/shared/api';
import { Badge, type ColumnDef, type ConfirmOptions, StatusDot } from '@/shared/ui';
import type { ComposeJob } from '../model/composeJob';
import type { ServiceFreshness } from '../model/freshness';
import { KIND_BADGE, KIND_LABEL } from '../model/stackKind';
import { STATE_LABEL, STATE_TONE } from '../model/stackState';
import { StackActions } from './StackActions';
import { UpdateBadge } from './UpdateBadge';
import { t } from '@/shared/i18n';

/** The Stacks table's columns; the last one holds the row's actions. */
export function stackColumns({
  hostId,
  confirm,
  onCompose,
  newerServices,
}: {
  hostId: string;
  confirm: (o: ConfirmOptions) => void;
  onCompose: (job: ComposeJob) => void;
  /** The services of each Stack, by name, that run an outdated image. */
  newerServices: (stack: Stack) => ServiceFreshness[];
}): ColumnDef<Stack, unknown>[] {
  return [
    {
      id: 'name',
      header: t('stacks.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => (
        <span className="grid">
          <span className="font-medium">{row.original.name}</span>
          {row.original.kind === 'linked' && row.original.workingDir && (
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
        <span className="flex items-center gap-1.5">
          <Badge variant={KIND_BADGE[row.original.kind]}>{t(KIND_LABEL[row.original.kind])}</Badge>
          {row.original.problem === 'filesMissing' && (
            <TriangleAlertIcon
              role="img"
              aria-label={t('stacks.problem.filesMissing')}
              className="text-warning size-4"
            >
              <title>{t('stacks.problem.filesMissing')}</title>
            </TriangleAlertIcon>
          )}
        </span>
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
          {newerServices(row.original).length > 0 && (
            <UpdateBadge newer={newerServices(row.original)} />
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
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <StackActions
          compact
          hostId={hostId}
          stack={row.original}
          newer={newerServices(row.original)}
          confirm={confirm}
          onCompose={onCompose}
        />
      ),
    },
  ];
}
