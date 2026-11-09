import { useQuery } from '@tanstack/react-query';
import { EyeIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import type { SwarmObject } from '@/shared/api';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  SearchInput,
  Spinner,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import { useRemoveObject, type ObjectKind } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { ConfigContentDialog } from './ConfigContentDialog';
import { CreateObjectDialog } from './CreateObjectDialog';
import { t } from '@/shared/i18n';
import { formatDateTime } from '@/shared/lib';

const date = (value: string | null) => (value ? formatDateTime(value) : '—');

/** A Swarm's secrets or configs: their metadata and the services using them. */
export function ObjectsTab({ swarmId, kind }: { swarmId: string; kind: ObjectKind }) {
  const objects = useQuery(swarmFeatureQueries.objects(swarmId, kind));
  const remove = useRemoveObject(swarmId, kind);
  const [dialog, confirm] = useConfirmedAction();
  const [creating, setCreating] = useState(false);
  const [viewing, setViewing] = useState<SwarmObject | null>(null);
  const [query, setQuery] = useState('');
  const secret = kind === 'secret';

  const columns: ColumnDef<SwarmObject, unknown>[] = [
    {
      id: 'name',
      header: t('swarm.objects.column.name'),
      accessorFn: (o) => o.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    {
      id: 'services',
      header: t('swarm.objects.column.services'),
      accessorFn: (o) => o.services.join(' '),
      cell: ({ row }) =>
        row.original.services.length ? (
          <span className="flex flex-wrap gap-1">
            {row.original.services.map((s) => (
              <Badge key={s} variant="outline">
                {s}
              </Badge>
            ))}
          </span>
        ) : (
          <span className="text-muted-foreground">{t('swarm.objects.unused')}</span>
        ),
    },
    {
      id: 'labels',
      header: t('swarm.objects.column.labels'),
      accessorFn: (o) => Object.keys(o.labels).join(' '),
      cell: ({ row }) => (
        <span className="flex flex-wrap gap-1">
          {Object.entries(row.original.labels).map(([k, v]) => (
            <Badge key={k} variant="secondary" className="font-mono text-xs">
              {k}={v}
            </Badge>
          ))}
        </span>
      ),
    },
    {
      id: 'created',
      header: t('swarm.objects.column.created'),
      accessorFn: (o) => date(o.created),
    },
    {
      id: 'updated',
      header: t('swarm.objects.column.updated'),
      accessorFn: (o) => date(o.updated),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end">
          {!secret && (
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('swarm.objects.viewOne', { name: row.original.name })}
              onClick={() => setViewing(row.original)}
            >
              <EyeIcon />
            </Button>
          )}
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('swarm.objects.removeOne', { name: row.original.name })}
            onClick={() =>
              confirm({
                title: t('swarm.objects.remove.title', { name: row.original.name }),
                description: row.original.services.length
                  ? t('swarm.objects.remove.used', { services: row.original.services.join(', ') })
                  : t('swarm.objects.remove.unused'),
                confirmLabel: t('common.remove'),
                destructive: true,
                action: () => remove.mutateAsync(row.original),
              })
            }
          >
            <Trash2Icon />
          </Button>
        </div>
      ),
    },
  ];

  if (objects.isPending) return <Spinner />;
  if (objects.error)
    return <ErrorMessage error={objects.error} onRetry={() => void objects.refetch()} />;
  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center gap-2">
        <SearchInput
          value={query}
          onChange={setQuery}
          placeholder={secret ? t('swarm.objects.searchSecrets') : t('swarm.objects.searchConfigs')}
        />
        <Button className="ms-auto" onClick={() => setCreating(true)}>
          <PlusIcon />
          {secret ? t('swarm.objects.createSecret') : t('swarm.objects.createConfig')}
        </Button>
      </div>
      {secret && <p className="text-muted-foreground text-sm">{t('swarm.objects.secretNote')}</p>}
      {objects.data.length === 0 ? (
        <EmptyState title={secret ? t('swarm.objects.noSecrets') : t('swarm.objects.noConfigs')} />
      ) : (
        <DataTable
          columns={columns}
          data={objects.data}
          getRowId={(o) => o.id}
          filter={query}
          onRowClick={secret ? undefined : setViewing}
        />
      )}
      {creating && (
        <CreateObjectDialog swarmId={swarmId} kind={kind} onClose={() => setCreating(false)} />
      )}
      {viewing && (
        <ConfigContentDialog swarmId={swarmId} config={viewing} onClose={() => setViewing(null)} />
      )}
      {dialog}
    </div>
  );
}
