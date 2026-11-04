import type { Stack } from '@/shared/api';
import { Badge, type ColumnDef, type ConfirmOptions, StatusDot } from '@/shared/ui';
import type { ComposeJob } from '../model/composeJob';
import { STATE_LABEL, STATE_TONE } from '../model/stackState';
import { StackActions } from './StackActions';

/** The Stacks table's columns; the last one holds the row's actions. */
export function stackColumns({
  hostId,
  confirm,
  onCompose,
}: {
  hostId: string;
  confirm: (o: ConfirmOptions) => void;
  onCompose: (job: ComposeJob) => void;
}): ColumnDef<Stack, unknown>[] {
  return [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (s) => s.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    {
      id: 'kind',
      header: 'Kind',
      accessorFn: (s) => s.kind,
      cell: ({ row }) => (
        <Badge variant={row.original.kind === 'managed' ? 'default' : 'outline'}>
          {row.original.kind === 'managed' ? 'Managed' : 'External'}
        </Badge>
      ),
    },
    {
      id: 'state',
      header: 'State',
      accessorFn: (s) => s.state,
      cell: ({ row }) => (
        <span className="flex items-center gap-1.5 text-sm">
          <StatusDot tone={STATE_TONE[row.original.state]} />
          {STATE_LABEL[row.original.state]}
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
      header: 'Services',
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
          confirm={confirm}
          onCompose={onCompose}
        />
      ),
    },
  ];
}
