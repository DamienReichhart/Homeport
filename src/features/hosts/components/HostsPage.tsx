import { PlusIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import type { HostView } from '@/shared/api';
import { Badge, Button, type ColumnDef, DataTable, ErrorMessage, PageHeader } from '@/shared/ui';
import { useHosts } from '../api';
import { connectionLabel, describeAddress } from '../status';
import { HostStatusBadge } from './HostStatusBadge';

const columns: ColumnDef<HostView, unknown>[] = [
  {
    id: 'name',
    header: 'Name',
    accessorFn: (v) => v.host.name,
    cell: ({ row }) => <span className="font-medium">{row.original.host.name}</span>,
  },
  {
    id: 'kind',
    header: 'Type',
    accessorFn: (v) => v.host.connection.kind,
    cell: ({ row }) => (
      <Badge variant="outline">{connectionLabel(row.original.host.connection.kind)}</Badge>
    ),
  },
  {
    id: 'address',
    header: 'Address',
    accessorFn: (v) => describeAddress(v.host.connection),
    cell: ({ getValue }) => <span className="font-mono text-xs">{String(getValue())}</span>,
  },
  {
    id: 'status',
    header: 'Status',
    accessorFn: (v) => v.status.state,
    cell: ({ row }) => <HostStatusBadge status={row.original.status} />,
  },
];

export function HostsPage() {
  const hosts = useHosts();
  const navigate = useNavigate();
  return (
    <div>
      <PageHeader
        title="Hosts"
        description="The Docker Engines Homeport connects to."
        actions={
          <Button asChild>
            <Link to="/hosts/new">
              <PlusIcon /> Add host
            </Link>
          </Button>
        }
      />
      {hosts.error ? (
        <ErrorMessage error={hosts.error} onRetry={() => void hosts.refetch()} />
      ) : (
        <DataTable
          columns={columns}
          data={hosts.data ?? []}
          getRowId={(v) => v.host.id}
          onRowClick={(v) => navigate(`/hosts/${v.host.id}`)}
          empty="No hosts yet."
        />
      )}
    </div>
  );
}
