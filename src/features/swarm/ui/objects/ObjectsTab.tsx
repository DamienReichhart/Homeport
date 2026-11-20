import { useQuery } from '@tanstack/react-query';
import { EyeIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import type { SwarmObject } from '@/shared/api';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  SearchInput,
  Spinner,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import { useRemoveObject, type ObjectKind } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { ConfigContentDialog } from './ConfigContentDialog';
import { CreateObjectDialog } from './CreateObjectDialog';
import { t } from '@/shared/i18n';
import { formatDateTime, formatRelative } from '@/shared/lib';

/** When, relative ("3 days ago"), with the date and time in its tooltip. */
function When({ value }: { value: string | null }) {
  if (!value) return <span className="text-muted-foreground">—</span>;
  return (
    <span className="text-muted-foreground whitespace-nowrap" title={formatDateTime(value)}>
      {formatRelative(Date.parse(value) / 1000)}
    </span>
  );
}

/** An icon button of a row, named and with its tooltip. */
function RowButton({
  label,
  onClick,
  children,
}: {
  label: string;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={label}
          onClick={(e) => {
            e.stopPropagation();
            onClick();
          }}
        >
          {children}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}

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
      cell: ({ row }) => {
        const labels = Object.entries(row.original.labels).map(([k, v]) => `${k}=${v}`);
        return (
          <span
            className="text-muted-foreground block max-w-64 truncate font-mono text-xs"
            title={labels.join('\n') || undefined}
          >
            {labels.join(', ') || '—'}
          </span>
        );
      },
    },
    {
      id: 'created',
      header: t('swarm.objects.column.created'),
      accessorFn: (o) => o.created ?? '',
      cell: ({ row }) => <When value={row.original.created} />,
    },
    {
      id: 'updated',
      header: t('swarm.objects.column.updated'),
      accessorFn: (o) => o.updated ?? '',
      cell: ({ row }) => <When value={row.original.updated} />,
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end">
          {!secret && (
            <RowButton
              label={t('swarm.objects.viewOne', { name: row.original.name })}
              onClick={() => setViewing(row.original)}
            >
              <EyeIcon />
            </RowButton>
          )}
          <RowButton
            label={t('swarm.objects.removeOne', { name: row.original.name })}
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
          </RowButton>
        </div>
      ),
    },
  ];

  const createLabel = (
    <>
      <PlusIcon />
      {secret ? t('swarm.objects.createSecret') : t('swarm.objects.createConfig')}
    </>
  );

  if (objects.isPending) return <Spinner />;
  if (objects.error)
    return <ErrorMessage error={objects.error} onRetry={() => void objects.refetch()} />;
  return (
    <div className="grid gap-3">
      {objects.data.length > 0 && (
        <div className="flex flex-wrap items-center gap-2">
          <SearchInput
            value={query}
            onChange={setQuery}
            placeholder={
              secret ? t('swarm.objects.searchSecrets') : t('swarm.objects.searchConfigs')
            }
          />
          <Button className="ms-auto" onClick={() => setCreating(true)}>
            {createLabel}
          </Button>
        </div>
      )}
      {objects.data.length === 0 ? (
        <EmptyState
          title={secret ? t('swarm.objects.noSecrets') : t('swarm.objects.noConfigs')}
          description={secret ? t('swarm.objects.secretNote') : t('swarm.objects.configNote')}
          action={<Button onClick={() => setCreating(true)}>{createLabel}</Button>}
        />
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
