import { useQuery } from '@tanstack/react-query';
import { openUrl } from '@tauri-apps/plugin-opener';
import { ArrowRightLeftIcon, CircleStopIcon, ExternalLinkIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import { objectPath } from '@/core/cluster';
import type { PortForward } from '@/shared/api';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  toastError,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  type ColumnDef,
} from '@/shared/ui';
import { useStopForward } from '../api/mutations';
import { forwardQueries } from '../api/queries';
import { targetLabel } from '../model/forwards';
import { t } from '@/shared/i18n';

function columns(stop: (id: number) => void): ColumnDef<PortForward, unknown>[] {
  return [
    {
      id: 'local',
      header: t('portforwards.column.local'),
      accessorFn: (f) => f.addresses.join(' '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.addresses.join(', ')}</span>
      ),
    },
    {
      id: 'target',
      header: t('portforwards.column.target'),
      accessorFn: (f) => `${targetLabel(f.target)}:${f.remotePort}`,
      cell: ({ row }) => {
        const f = row.original;
        const path = objectPath(f.target.kind, f.target.namespace, f.target.name);
        const label = `${targetLabel(f.target)}:${f.remotePort}`;
        return (
          <span className="font-mono text-xs">
            {path ? <Link to={path}>{label}</Link> : label}
            {f.pod && f.target.kind === 'service' && (
              <span className="text-muted-foreground"> → {f.pod}</span>
            )}
          </span>
        );
      },
    },
    {
      id: 'namespace',
      header: t('portforwards.column.namespace'),
      accessorFn: (f) => f.target.namespace,
    },
    { id: 'cluster', header: t('portforwards.column.cluster'), accessorFn: (f) => f.clusterName },
    {
      id: 'connections',
      header: t('portforwards.column.connections'),
      accessorFn: (f) => f.connections,
      cell: ({ row }) => <span className="tabular-nums">{row.original.connections}</span>,
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end gap-0.5">
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('portforwards.open')}
                onClick={() =>
                  openUrl(`http://127.0.0.1:${row.original.localPort}`).catch(toastError)
                }
              >
                <ExternalLinkIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t('portforwards.open')}</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t('portforwards.stop')}
                onClick={() => stop(row.original.id)}
              >
                <CircleStopIcon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t('portforwards.stop')}</TooltipContent>
          </Tooltip>
        </div>
      ),
    },
  ];
}

/** Every active port forward, across Clusters. */
export function PortForwardsPage() {
  const list = useQuery(forwardQueries.list());
  const stop = useStopForward();
  return (
    <div>
      <PageHeader
        title={t('portforwards.title')}
        description={
          list.data?.length ? t('portforwards.summary', { count: list.data.length }) : undefined
        }
      />
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : list.data.length === 0 ? (
        <EmptyState
          icon={ArrowRightLeftIcon}
          title={t('portforwards.empty.title')}
          description={t('portforwards.empty.description')}
          action={
            <Button variant="outline" asChild>
              <Link to="/services">{t('portforwards.empty.action')}</Link>
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={columns((id) => stop.mutate(id))}
          data={list.data}
          getRowId={(f) => String(f.id)}
        />
      )}
    </div>
  );
}
