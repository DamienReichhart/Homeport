import { PlusIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { Button, DataTable, ErrorMessage, PageHeader } from '@/shared/ui';
import { hostColumns } from './hostColumns';

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
          columns={hostColumns}
          data={hosts.data ?? []}
          getRowId={(v) => v.host.id}
          onRowClick={(v) => navigate(`/hosts/${v.host.id}`)}
          empty="No hosts yet."
        />
      )}
    </div>
  );
}
