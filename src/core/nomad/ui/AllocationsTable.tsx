import { age } from '@/core/cluster';
import type { NomadAllocationView } from '@/shared/api';
import { DataTable, type ColumnDef } from '@/shared/ui';
import { allocationTone, shortId } from '../model/status';
import { NomadStatus } from './NomadStatus';
import { t } from '@/shared/i18n';

const columns: ColumnDef<NomadAllocationView, unknown>[] = [
  {
    id: 'id',
    header: t('nomad.alloc.column.id'),
    accessorFn: (a) => a.id,
    cell: ({ row }) => <span className="font-mono text-xs">{shortId(row.original.id)}</span>,
  },
  { id: 'name', header: t('nomad.alloc.column.name'), accessorFn: (a) => a.name },
  {
    id: 'status',
    header: t('nomad.alloc.column.status'),
    accessorFn: (a) => a.clientStatus,
    cell: ({ row }) => (
      <NomadStatus
        status={row.original.clientStatus}
        tone={allocationTone(row.original.clientStatus)}
      />
    ),
  },
  {
    id: 'node',
    header: t('nomad.alloc.column.node'),
    accessorFn: (a) => a.nodeName || shortId(a.nodeId),
  },
  {
    id: 'version',
    header: t('nomad.alloc.column.version'),
    accessorFn: (a) => a.jobVersion,
  },
  {
    id: 'restarts',
    header: t('nomad.alloc.column.restarts'),
    accessorFn: (a) => a.tasks.reduce((sum, task) => sum + task.restarts, 0),
  },
  {
    id: 'created',
    header: t('nomad.alloc.column.created'),
    accessorFn: (a) => a.created ?? '',
    cell: ({ row }) => age(row.original.created),
  },
];

/** Allocations, newest first, opening one on click when `onOpen` is given. */
export function AllocationsTable({
  allocations,
  onOpen,
}: {
  allocations: NomadAllocationView[];
  onOpen?: (allocation: NomadAllocationView) => void;
}) {
  if (allocations.length === 0)
    return <p className="text-muted-foreground text-sm">{t('nomad.alloc.none')}</p>;
  return (
    <DataTable columns={columns} data={allocations} getRowId={(a) => a.id} onRowClick={onOpen} />
  );
}
