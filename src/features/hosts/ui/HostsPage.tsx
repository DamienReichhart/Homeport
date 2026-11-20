import { DownloadIcon, EllipsisVerticalIcon, PlusIcon, ServerIcon, UploadIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import {
  Button,
  DataTable,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  WithShortcut,
} from '@/shared/ui';
import { ExportHostsDialog } from './bundle/ExportHostsDialog';
import { ImportBundleDialog } from './bundle/ImportBundleDialog';
import { hostColumns } from './hostColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

/** Every Host, opened beside the list; adding one is the page's action. */
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
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('hosts.add')}>
              <Button asChild>
                <Link to="/hosts/new">
                  <PlusIcon /> {t('hosts.add')}
                </Link>
              </Button>
            </WithShortcut>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
                  <EllipsisVerticalIcon />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end" className="w-auto">
                <DropdownMenuItem onSelect={() => setDialog('import')}>
                  <UploadIcon /> {t('hosts.bundle.import.button')}
                </DropdownMenuItem>
                <DropdownMenuItem
                  disabled={!hosts.data?.length}
                  onSelect={() => setDialog('export')}
                >
                  <DownloadIcon /> {t('hosts.bundle.export.button')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </>
        }
      />
      {hosts.error ? (
        <ErrorMessage error={hosts.error} onRetry={() => void hosts.refetch()} />
      ) : hosts.isPending ? (
        <Spinner />
      ) : hosts.data.length === 0 ? (
        <EmptyState
          icon={ServerIcon}
          title={t('hosts.empty')}
          description={t('hosts.emptyDescription')}
          action={
            <Button variant="outline" asChild>
              <Link to="/hosts/new">
                <PlusIcon /> {t('hosts.add')}
              </Link>
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={hostColumns()}
          data={hosts.data}
          getRowId={(v) => v.host.id}
          onRowClick={(v) => navigate(`/hosts/${v.host.id}`)}
        />
      )}
      {dialog === 'export' && <ExportHostsDialog onClose={() => setDialog(null)} />}
      {dialog === 'import' && <ImportBundleDialog onClose={() => setDialog(null)} />}
    </div>
  );
}
