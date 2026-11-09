import { useQuery } from '@tanstack/react-query';
import { NetworkIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePageQuery } from '@/core/feature';
import { IncusGate } from '@/core/host';
import type { IncusNetwork } from '@/shared/api';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirmedAction,
  WithShortcut,
  type ColumnDef,
} from '@/shared/ui';
import { useNetworks } from '../api/mutations';
import { incusNetworkQueries } from '../api/queries';
import { NewBridgeDialog } from './NewBridgeDialog';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function NetworksList({ hostId, project }: { hostId: string; project: string }) {
  const networks = useQuery(incusNetworkQueries.list(hostId, project));
  const { remove } = useNetworks(hostId, project);
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = usePageQuery();
  const [creating, setCreating] = useState(false);
  usePageAction(t('incus.networks.new'), () => setCreating(true));

  if (networks.isPending) return <Spinner />;
  if (networks.error)
    return <ErrorMessage error={networks.error} onRetry={() => void networks.refetch()} />;

  const columns: ColumnDef<IncusNetwork, unknown>[] = [
    {
      id: 'name',
      header: t('incus.networks.column.name'),
      accessorFn: (n) => n.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {!row.original.managed && (
            <Badge variant="outline">{t('incus.networks.unmanaged')}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'type',
      header: t('incus.networks.column.type'),
      accessorFn: (n) => n.type,
      cell: ({ row }) => <Badge variant="secondary">{row.original.type}</Badge>,
    },
    {
      id: 'ipv4',
      header: t('incus.networks.column.ipv4'),
      accessorFn: (n) => n.ipv4 ?? '',
      cell: ({ row }) => (
        <span className="font-mono text-xs">
          {row.original.ipv4 ?? '—'}
          {row.original.nat && (
            <Badge variant="outline" className="ml-2">
              NAT
            </Badge>
          )}
        </span>
      ),
    },
    {
      id: 'ipv6',
      header: t('incus.networks.column.ipv6'),
      accessorFn: (n) => n.ipv6 ?? '',
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.ipv6 ?? '—'}</span>,
    },
    {
      id: 'usedBy',
      header: t('incus.networks.column.usedBy'),
      accessorFn: (n) => n.usedBy.map((u) => u.name).join(' '),
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs">
          {row.original.usedBy.map((u) => u.name).join(', ') || '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const network = row.original;
        if (!network.managed) return null;
        const inUse = network.usedBy.length > 0;
        return (
          <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('incus.networks.deleteOne', { name: network.name })}
              title={
                inUse
                  ? t('incus.networks.inUse', {
                      names: network.usedBy.map((u) => u.name).join(', '),
                    })
                  : undefined
              }
              disabled={inUse || remove.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.networks.delete.title', { name: network.name }),
                  description: t('incus.networks.delete.description'),
                  confirmLabel: t('incus.delete'),
                  destructive: true,
                  action: () => remove.mutateAsync(network.name),
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
        title={t('incus.networks.title')}
        description={t('incus.networks.summary', { count: networks.data.length, project })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('incus.networks.new')}>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('incus.networks.new')}
            </Button>
          </WithShortcut>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('incus.networks.search')} />
      </div>
      {networks.data.length === 0 ? (
        <EmptyState icon={NetworkIcon} title={t('incus.networks.empty')} />
      ) : (
        <DataTable
          columns={columns}
          data={networks.data}
          getRowId={(n) => n.name}
          filter={query}
          onRowClick={(n) => void navigate(`/incus/networks/${encodeURIComponent(n.name)}`)}
        />
      )}
      <NewBridgeDialog
        hostId={hostId}
        project={project}
        open={creating}
        onClose={() => setCreating(false)}
      />
      {dialog}
    </div>
  );
}

export function NetworksPage() {
  return (
    <IncusGate>
      {(hostId, project) => (
        <NetworksList key={`${hostId}/${project}`} hostId={hostId} project={project} />
      )}
    </IncusGate>
  );
}
