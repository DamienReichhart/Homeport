import { DownloadIcon, PlusIcon, UploadIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { Button, DataTable, ErrorMessage, PageHeader } from '@/shared/ui';
import { ExportHostsDialog } from './bundle/ExportHostsDialog';
import { ImportBundleDialog } from './bundle/ImportBundleDialog';
import { hostColumns } from './hostColumns';
import { t } from '@/shared/i18n';

export function HostsPage() {
  const hosts = useHosts();
  const navigate = useNavigate();
  const [dialog, setDialog] = useState<'export' | 'import' | null>(null);
  return (
    <div>
      <PageHeader
        title={t('hosts.title')}
        description={t('hosts.description')}
        actions={
          <>
            <Button variant="outline" onClick={() => setDialog('import')}>
              <UploadIcon /> {t('hosts.bundle.import.button')}
            </Button>
            <Button
              variant="outline"
              disabled={!hosts.data?.length}
              onClick={() => setDialog('export')}
            >
              <DownloadIcon /> {t('hosts.bundle.export.button')}
            </Button>
            <Button asChild>
              <Link to="/hosts/new">
                <PlusIcon /> {t('hosts.add')}
              </Link>
            </Button>
          </>
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
      {dialog === 'export' && <ExportHostsDialog onClose={() => setDialog(null)} />}
      {dialog === 'import' && <ImportBundleDialog onClose={() => setDialog(null)} />}
    </div>
  );
}
