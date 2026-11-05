import { PlusIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { Button, DataTable, ErrorMessage, PageHeader } from '@/shared/ui';
import { hostColumns } from './hostColumns';
import { t } from '@/shared/i18n';

export function HostsPage() {
  const hosts = useHosts();
  const navigate = useNavigate();
  return (
    <div>
      <PageHeader
        title={t('hosts.title')}
        description={t('hosts.description')}
        actions={
          <Button asChild>
            <Link to="/hosts/new">
              <PlusIcon /> {t('hosts.add')}
            </Link>
          </Button>
        }
      />
      {hosts.error ? (
        <ErrorMessage error={hosts.error} onRetry={() => void hosts.refetch()} />
      ) : (
        <DataTable
          columns={hostColumns()}
          data={hosts.data ?? []}
          getRowId={(v) => v.host.id}
          onRowClick={(v) => navigate(`/hosts/${v.host.id}`)}
          empty={t('hosts.empty')}
        />
      )}
    </div>
  );
}
