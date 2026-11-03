import { BoxesIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { HostGate } from '@/core/host';
import { type Stack } from '@/shared/api';
import {
  Badge,
  Button,
  type ColumnDef,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  StatusDot,
  useConfirm,
} from '@/shared/ui';
import { STATE_LABEL, STATE_TONE, useStacks } from './api';
import { ComposeRunDialog, type ComposeJob } from './ComposeRunDialog';
import { StackActions } from './StackActions';

function StacksList({ hostId }: { hostId: string }) {
  const stacks = useStacks(hostId);
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [job, setJob] = useState<ComposeJob | null>(null);

  const columns: ColumnDef<Stack, unknown>[] = [
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
          onCompose={setJob}
        />
      ),
    },
  ];

  if (stacks.isPending) return <Spinner />;
  if (stacks.error)
    return <ErrorMessage error={stacks.error} onRetry={() => void stacks.refetch()} />;
  return (
    <div>
      <PageHeader
        title="Stacks"
        description="Compose projects on this host. Managed Stacks keep their definition in Homeport; External ones were deployed elsewhere."
        actions={
          <Button asChild>
            <Link to="/stacks/new">
              <PlusIcon /> New stack
            </Link>
          </Button>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search stacks" />
      </div>
      {stacks.data.length === 0 ? (
        <EmptyState
          icon={BoxesIcon}
          title="No stacks"
          description="Write a compose definition and deploy it to this host."
          action={
            <Button asChild>
              <Link to="/stacks/new">New stack</Link>
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={columns}
          data={stacks.data}
          getRowId={(s) => s.name}
          filter={query}
          onRowClick={(s) => navigate(`/stacks/${encodeURIComponent(s.name)}`)}
        />
      )}
      <ComposeRunDialog hostId={hostId} job={job} onClose={() => setJob(null)} />
      {dialog}
    </div>
  );
}

export function StacksPage() {
  return <HostGate>{(hostId) => <StacksList key={hostId} hostId={hostId} />}</HostGate>;
}
