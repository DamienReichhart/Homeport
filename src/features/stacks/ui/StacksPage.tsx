import { useQuery } from '@tanstack/react-query';
import { BoxesIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { HostGate, resourceQueries } from '@/core/host';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import type { ComposeJob } from '../model/composeJob';
import { ComposeRunDialog } from './dialogs/ComposeRunDialog';
import { stackColumns } from './stackColumns';

function StacksList({ hostId }: { hostId: string }) {
  const stacks = useQuery(resourceQueries.stacks(hostId));
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [job, setJob] = useState<ComposeJob | null>(null);

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
          columns={stackColumns({ hostId, confirm, onCompose: setJob })}
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
