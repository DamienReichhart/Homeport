import { ScrollTextIcon, SquareTerminalIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { age, formatCpu, measureOf, podKey, podTone, UsageCell, usageFormat } from '@/core/cluster';
import type { PodUsage, PodView } from '@/shared/api';
import {
  Button,
  DataTable,
  StatusDot,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  type ColumnDef,
} from '@/shared/ui';
import { cn } from '@/shared/lib';
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

/** A Pod's page, or one of its tabs. */
const podPath = (p: { namespace: string; name: string }, tab?: string) =>
  `/pods/${encodeURIComponent(p.namespace)}/${encodeURIComponent(p.name)}${tab ? `/${tab}` : ''}`;

/** A row's icon link to one of the Pod's tabs, which doesn't also open the row. */
function TabLink({ to, label, icon }: { to: string; label: string; icon: ReactNode }) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button variant="ghost" size="icon-sm" asChild>
          <Link to={to} aria-label={label} onClick={(e) => e.stopPropagation()}>
            {icon}
          </Link>
        </Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}

function columns(
  showNamespace: boolean,
  usage: Map<string, PodUsage> | null,
  showNode: boolean,
): ColumnDef<PodView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('pods.column.name'),
      accessorFn: (p) => p.name,
      cell: ({ row }) => (
        <span
          className="inline-block max-w-80 truncate align-bottom font-medium @max-md:max-w-36"
          title={row.original.name}
        >
          {row.original.name}
        </span>
      ),
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
      // A failing Pod's reason is in red, so it stands out of a long list.
      cell: ({ row }) => {
        const tone = podTone(row.original.health);
        return (
          <span
            className={cn(
              'flex max-w-56 items-center gap-1.5 @max-md:max-w-28',
              tone === 'danger' && 'text-destructive font-medium',
            )}
            title={row.original.status}
          >
            <StatusDot tone={tone} />
            <span className="truncate">{row.original.status}</span>
          </span>
        );
      },
      meta: { compact: true },
    },
    {
      id: 'ready',
      header: t('pods.column.ready'),
      accessorFn: (p) => `${p.ready}/${p.containers}`,
    },
    {
      id: 'restarts',
      header: t('pods.column.restarts'),
      accessorFn: (p) => p.restarts,
      cell: ({ row }) => (
        <span className={cn('tabular-nums', row.original.restarts > 0 && 'text-warning')}>
          {row.original.restarts}
        </span>
      ),
    },
    ...(usage ? usageColumns(usage) : []),
    ...(showNode
      ? [
          {
            id: 'node',
            header: t('pods.column.node'),
            accessorFn: (p: PodView) => p.node ?? '',
            cell: ({ row }: { row: { original: PodView } }) => (
              <span
                className="inline-block max-w-48 truncate align-bottom"
                title={row.original.node ?? ''}
              >
                {row.original.node}
              </span>
            ),
          },
        ]
      : []),
    {
      id: 'age',
      header: t('pods.column.age'),
      accessorFn: (p) => p.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex items-center justify-end gap-0.5">
          {row.original.health === 'running' && (
            <TabLink
              to={podPath(row.original, 'terminal')}
              label={t('pods.tab.terminal')}
              icon={<SquareTerminalIcon />}
            />
          )}
          <TabLink
            to={podPath(row.original, 'logs')}
            label={t('pods.tab.logs')}
            icon={<ScrollTextIcon />}
          />
        </div>
      ),
    },
  ];
}

/** Pods, each opening its page; long lists show a page at a time. */
export function PodsTable({
  pods,
  filter = '',
  showNamespace,
  usage = null,
  showNode = true,
  onSelectionChange,
}: {
  pods: PodView[];
  filter?: string;
  showNamespace: boolean;
  /** Each Pod's latest CPU and memory, by `namespace/name`; no columns for them without. */
  usage?: Map<string, PodUsage> | null;
  /** Off where every Pod shares a page with its owner, which has room for less. */
  showNode?: boolean;
  /** Lets rows be picked, for deleting several. */
  onSelectionChange?: (pods: PodView[]) => void;
}) {
  const navigate = useNavigate();
  return (
    // Sized as a container, so a narrow list (beside an open Pod) cuts names shorter.
    <div className="@container">
      <DataTable
        columns={columns(showNamespace, usage, showNode)}
        data={pods}
        getRowId={(p) => `${p.namespace}/${p.name}`}
        filter={filter}
        pageSize={PAGE}
        empty={t('pods.empty')}
        onSelectionChange={onSelectionChange}
        onRowClick={(p) => void navigate(podPath(p))}
      />
    </div>
  );
}
