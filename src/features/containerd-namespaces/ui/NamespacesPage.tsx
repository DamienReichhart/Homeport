import { useQuery } from '@tanstack/react-query';
import { CheckIcon, FolderTreeIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { useActionRequest } from '@/core/feature';
import { containerdQueries, HostGate, useContainerdNamespace } from '@/core/host';
import type { ContainerdNamespace } from '@/shared/api';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  useConfirmedAction,
  WithShortcut,
  type ColumnDef,
} from '@/shared/ui';
import { useRemoveNamespace } from '../api/mutations';
import { NewNamespaceDialog } from './NewNamespaceDialog';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function NamespacesList({ hostId }: { hostId: string }) {
  const namespaces = useQuery(containerdQueries.namespaces(hostId));
  const [current, choose] = useContainerdNamespace(hostId);
  const remove = useRemoveNamespace(hostId);
  const [dialog, confirm] = useConfirmedAction();
  const [creating, setCreating] = useState(false);
  usePageAction(t('containerd.namespaces.new'), () => setCreating(true));
  useActionRequest('new', () => setCreating(true));

  if (namespaces.isPending) return <Spinner />;
  if (namespaces.error)
    return <ErrorMessage error={namespaces.error} onRetry={() => void namespaces.refetch()} />;

  const columns: ColumnDef<ContainerdNamespace, unknown>[] = [
    {
      id: 'name',
      header: t('containerd.namespaces.column.name'),
      accessorFn: (n) => n.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.name === current && (
            <Badge variant="secondary">{t('containerd.namespaces.shown')}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'containers',
      header: t('containerd.namespaces.column.containers'),
      accessorFn: (n) => n.containers,
    },
    {
      id: 'images',
      header: t('containerd.namespaces.column.images'),
      accessorFn: (n) => n.images,
    },
    {
      id: 'labels',
      header: t('containerd.namespaces.column.labels'),
      accessorFn: (n) => Object.keys(n.labels).join(' '),
      cell: ({ row }) => (
        <span className="flex flex-wrap gap-1">
          {Object.entries(row.original.labels).map(([k, v]) => (
            <Badge key={k} variant="outline" className="font-mono">
              {k}={v}
            </Badge>
          ))}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const ns = row.original;
        const empty = ns.containers === 0 && ns.images === 0;
        return (
          <div className="flex justify-end gap-1" onClick={(e) => e.stopPropagation()}>
            {ns.name !== current && (
              <Button variant="outline" size="sm" onClick={() => choose(ns.name)}>
                <CheckIcon /> {t('containerd.namespaces.show')}
              </Button>
            )}
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('containerd.namespaces.removeOne', { name: ns.name })}
              title={empty ? undefined : t('containerd.namespaces.notEmpty')}
              disabled={!empty || remove.isPending}
              onClick={() =>
                confirm({
                  title: t('containerd.namespaces.remove.title', { name: ns.name }),
                  description: t('containerd.namespaces.remove.description'),
                  confirmLabel: t('common.remove'),
                  destructive: true,
                  action: () => remove.mutateAsync(ns.name),
                })
              }
            >
              <Trash2Icon />
            </Button>
          </div>
        );
      },
    },
  ];

  return (
    <div>
      <PageHeader
        title={t('containerd.namespaces.title')}
        description={t('containerd.namespaces.summary', { count: namespaces.data.length })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('containerd.namespaces.new')}>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('containerd.namespaces.new')}
            </Button>
          </WithShortcut>
        }
      />
      {namespaces.data.length === 0 ? (
        <EmptyState
          icon={FolderTreeIcon}
          title={t('containerd.namespaces.empty')}
          description={t('containerd.namespaces.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={columns}
          data={namespaces.data}
          getRowId={(n) => n.name}
          onRowClick={(n) => choose(n.name)}
        />
      )}
      <NewNamespaceDialog
        hostId={hostId}
        open={creating}
        onOpenChange={setCreating}
        onCreated={choose}
      />
      {dialog}
    </div>
  );
}

export function NamespacesPage() {
  return (
    <HostGate requires="containerd">
      {(hostId) => <NamespacesList key={hostId} hostId={hostId} />}
    </HostGate>
  );
}
