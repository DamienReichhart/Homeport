import { useNavigate } from 'react-router-dom';
import {
  age,
  formatCpu,
  HealthBadge,
  measureOf,
  podKey,
  UsageCell,
  usageFormat,
} from '@/core/cluster';
import type { PodUsage, PodView } from '@/shared/api';
import { DataTable, type ColumnDef } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Rows past this show a page at a time. */
const PAGE = 100;

/** CPU and memory against each Pod's limits (else requests), sorted by use. */
function usageColumns(usage: Map<string, PodUsage>): ColumnDef<PodView, unknown>[] {
  const of = (p: PodView) => usage.get(podKey(p))?.usage;
  const cpuOf = (u: { cpu: number | null } | undefined) => (u ? (u.cpu ?? 0) : undefined);
  return [
    {
      id: 'cpu',
      header: t('pods.column.cpu'),
      accessorFn: (p) => of(p)?.cpu ?? -1,
      cell: ({ row }) => (
        <UsageCell
          used={cpuOf(of(row.original))}
          measure={measureOf(row.original.cpu)}
          format={formatCpu}
        />
      ),
    },
    {
      id: 'memory',
      header: t('pods.column.memory'),
      accessorFn: (p) => of(p)?.memory ?? -1,
      cell: ({ row }) => (
        <UsageCell
          used={of(row.original)?.memory}
          measure={measureOf(row.original.memory)}
          format={usageFormat('memory')}
        />
      ),
    },
  ];
}

function columns(
  showNamespace: boolean,
  usage: Map<string, PodUsage> | null,
): ColumnDef<PodView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('pods.column.name'),
      accessorFn: (p) => p.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'namespace',
            header: t('pods.column.namespace'),
            accessorFn: (p: PodView) => p.namespace,
          },
        ]
      : []),
    {
      id: 'status',
      header: t('pods.column.status'),
      accessorFn: (p) => p.status,
      cell: ({ row }) => <HealthBadge pod={row.original.health} status={row.original.status} />,
    },
    {
      id: 'ready',
      header: t('pods.column.ready'),
      accessorFn: (p) => `${p.ready}/${p.containers}`,
    },
    { id: 'restarts', header: t('pods.column.restarts'), accessorFn: (p) => p.restarts },
    ...(usage ? usageColumns(usage) : []),
    { id: 'node', header: t('pods.column.node'), accessorFn: (p) => p.node ?? '' },
    {
      id: 'age',
      header: t('pods.column.age'),
      accessorFn: (p) => p.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

/** Pods, each opening its page; long lists show a page at a time. */
export function PodsTable({
  pods,
  filter = '',
  showNamespace,
  usage = null,
  onSelectionChange,
}: {
  pods: PodView[];
  filter?: string;
  showNamespace: boolean;
  /** Each Pod's latest CPU and memory, by `namespace/name`; no columns for them without. */
  usage?: Map<string, PodUsage> | null;
  /** Lets rows be picked, for deleting several. */
  onSelectionChange?: (pods: PodView[]) => void;
}) {
  const navigate = useNavigate();
  return (
    <DataTable
      columns={columns(showNamespace, usage)}
      data={pods}
      getRowId={(p) => `${p.namespace}/${p.name}`}
      filter={filter}
      pageSize={PAGE}
      empty={t('pods.empty')}
      onSelectionChange={onSelectionChange}
      onRowClick={(p) =>
        void navigate(`/pods/${encodeURIComponent(p.namespace)}/${encodeURIComponent(p.name)}`)
      }
    />
  );
}
