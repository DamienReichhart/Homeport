import { DownloadIcon, PlusIcon, UploadIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { Button, DataTable, ErrorMessage, PageHeader, WithShortcut } from '@/shared/ui';
import { ExportHostsDialog } from './bundle/ExportHostsDialog';
import { ImportBundleDialog } from './bundle/ImportBundleDialog';
import { hostColumns } from './hostColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

export function HostsPage() {
  const hosts = useHosts();
  const navigate = useNavigate();
  const [dialog, setDialog] = useState<'export' | 'import' | null>(null);
  usePageAction(t('hosts.add'), () => void navigate('/hosts/new'));
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
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('hosts.add')}>
              <Button asChild>
                <Link to="/hosts/new">
                  <PlusIcon /> {t('hosts.add')}
                </Link>
              </Button>
            </WithShortcut>
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
