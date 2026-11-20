import { useQuery } from '@tanstack/react-query';
import { PlusIcon, RefreshCwIcon, ShipWheelIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  age,
  ClusterGate,
  kubernetesConnection,
  NamespacePicker,
  useNamespace,
} from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import type { ClusterView, InstallRequest, ReleaseSummary } from '@/shared/api';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import {
  Alert,
  AlertDescription,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  SearchInput,
  Spinner,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  WithShortcut,
  type ColumnDef,
} from '@/shared/ui';
import { helmQueries } from '../api/queries';
import { useInstall } from '../api/streams';
import { releasePath } from '../model/paths';
import { HelmGate } from './HelmGate';
import { HelmPageHeader } from './HelmPageHeader';
import { InstallDialog } from './install/InstallDialog';
import { ReleaseStatus } from './ReleaseStatus';
import { HelmRunDialog } from './run/HelmRunDialog';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<ReleaseSummary, unknown>[] {
  return [
    {
      id: 'name',
      header: t('helm.column.name'),
      accessorFn: (r) => r.name,
      cell: ({ row }) => (
        <span
          className="inline-block max-w-64 truncate align-bottom font-medium @max-md:max-w-32"
          title={row.original.name}
        >
          {row.original.name}
        </span>
      ),
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('helm.column.namespace'),
            accessorFn: (r: ReleaseSummary) => r.namespace,
          },
        ]
      : []),
    {
      id: 'chart',
      header: t('helm.column.chart'),
      accessorFn: (r) => r.chart,
      cell: ({ row }) => (
        <span className="inline-block max-w-64 truncate align-bottom" title={row.original.chart}>
          {row.original.chartName}{' '}
          <span className="text-muted-foreground font-mono text-xs">
            {row.original.chartVersion}
          </span>
        </span>
      ),
    },
    {
      id: 'app',
      header: t('helm.column.appVersion'),
      accessorFn: (r) => r.appVersion || '—',
      cell: ({ getValue }) => (
        <span className="inline-block max-w-40 truncate align-bottom" title={getValue<string>()}>
          {getValue<string>()}
        </span>
      ),
    },
    {
      id: 'revision',
      header: t('helm.column.revision'),
      accessorFn: (r) => r.revision,
      cell: ({ row }) => <span className="tabular-nums">{row.original.revision}</span>,
    },
    {
      id: 'status',
      header: t('helm.column.status'),
      accessorFn: (r) => r.status,
      cell: ({ row }) => <ReleaseStatus status={row.original.status} />,
      // Beside an open release, the list still shows a failed one.
      meta: { compact: true },
    },
    {
      id: 'updated',
      header: t('helm.column.updated'),
      accessorFn: (r) => r.updated ?? '',
      cell: ({ row }) => age(row.original.updated),
    },
  ];
}

function Releases({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(helmQueries.releases(id, namespace));
  const only = list.data?.onlyNamespace ?? null;
  const install = useInstall(id);
  const [choosing, setChoosing] = useState(false);
  const [installing, setInstalling] = useState<InstallRequest | null>(null);
  usePageAction(t('helm.install.open'), () => setChoosing(true));
  return (
    <div>
      <HelmPageHeader
        current="/helm"
        description={list.data && t('helm.summary', { count: list.data.releases.length })}
        actions={
          <>
            <NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('helm.install.open')}>
              <Button onClick={() => setChoosing(true)}>
                <PlusIcon /> {t('helm.install.open')}
              </Button>
            </WithShortcut>
          </>
        }
        toolbar={
          <>
            <SearchInput value={query} onChange={setQuery} placeholder={t('helm.search')} />
            {/* A release made in a terminal shows up once the list is read again. */}
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  aria-label={t('helm.refresh')}
                  disabled={list.isFetching}
                  onClick={() => void list.refetch()}
                >
                  <RefreshCwIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>{t('helm.refresh')}</TooltipContent>
            </Tooltip>
          </>
        }
      />
      <div className="mb-3" />
      {only && (
        <Alert className="mb-3">
          <AlertDescription>{t('helm.onlyNamespace', { namespace: only })}</AlertDescription>
        </Alert>
      )}
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : list.data.releases.length === 0 ? (
        <EmptyState
          icon={ShipWheelIcon}
          title={t('helm.empty')}
          description={t('helm.emptyHint')}
          action={
            <Button variant="outline" onClick={() => setChoosing(true)}>
              <PlusIcon /> {t('helm.install.open')}
            </Button>
          }
        />
      ) : (
        // Sized as a container, so a narrow list (beside an open release) cuts names shorter.
        <div className="@container">
          <DataTable<ReleaseSummary>
            columns={columns(namespace === null && only === null)}
            data={list.data.releases}
            getRowId={(r) => `${r.namespace}/${r.name}`}
            filter={query}
            pageSize={100}
            empty={t('helm.empty')}
            onRowClick={(r) => void navigate(releasePath(r.namespace, r.name))}
          />
        </div>
      )}
      {choosing && (
        <InstallDialog
          clusterId={id}
          clusterName={cluster.cluster.name}
          namespace={
            namespace ?? only ?? kubernetesConnection(cluster.cluster)?.namespace ?? 'default'
          }
          onInstall={(request) => {
            setChoosing(false);
            setInstalling(request);
          }}
          onClose={() => setChoosing(false)}
        />
      )}
      {installing && (
        <HelmRunDialog
          title={t('helm.install.running', {
            name: installing.name,
            chart: installing.chart,
            cluster: cluster.cluster.name,
          })}
          run={install(installing)}
          doneAction={{
            label: t('helm.install.openRelease'),
            onClick: () => void navigate(releasePath(installing.namespace, installing.name)),
          }}
          onClose={() => setInstalling(null)}
        />
      )}
    </div>
  );
}

export function ReleasesPage() {
  return (
    <ClusterGate>
      {(cluster) => (
        <HelmGate>
          <Releases key={cluster.cluster.id} cluster={cluster} />
        </HelmGate>
      )}
    </ClusterGate>
  );
}
