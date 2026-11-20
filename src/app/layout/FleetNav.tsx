import { useQuery } from '@tanstack/react-query';
import { PlusIcon, Settings2Icon, TriangleAlertIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link, NavLink } from 'react-router-dom';
import { clusterStatusError, describeClusterStatus, useClusters } from '@/core/cluster';
import {
  ContainerdNamespacePicker,
  describeStatus,
  IncusProjectPicker,
  resourceQueries,
  statusError,
  supports,
  useHosts,
} from '@/core/host';
import { useSelection } from '@/core/selection';
import { swarmLabel, useSwarms } from '@/core/swarm';
import { toAppError, type AppErrorPayload, type HostView } from '@/shared/api';
import { cn } from '@/shared/lib';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  showErrorDetails,
  StatusDot,
  type Tone,
} from '@/shared/ui';
import { navItem } from './navItem';
import { useSwitchTarget } from './useSwitchTarget';
import { t } from '@/shared/i18n';

/**
 * Every Host, Swarm and Cluster, each with its status, always in view: one
 * click switches, and the selected one opens onto its pages beneath it.
 */
export function FleetNav({ pages }: { pages: ReactNode }) {
  const hosts = useHosts();
  const clusters = useClusters();
  const swarms = useSwarms();
  const { selection } = useSelection();
  const switchTo = useSwitchTarget();
  const isSelected = (kind: 'host' | 'cluster', id: string) =>
    selection?.kind === kind && selection.id === id;
  const noTargets = !hosts.data?.length && !clusters.data?.length;
  return (
    <>
      <Group
        title={t('app.nav.hosts')}
        manage={hosts.data?.length ? '/hosts' : undefined}
        manageLabel={t('hosts.switcher.manage')}
      >
        {hosts.data?.map((view) => (
          <FleetItem
            key={view.host.id}
            name={view.host.name}
            tone={describeStatus(view.status).tone}
            status={describeStatus(view.status).label}
            meta={<HostCount view={view} />}
            error={statusError(view.status)}
            selected={isSelected('host', view.host.id)}
            onSelect={() => switchTo({ kind: 'host', view })}
          >
            <ContainerdNamespacePicker />
            <IncusProjectPicker />
            {pages}
          </FleetItem>
        ))}
        {noTargets && <div className="grid gap-0.5">{pages}</div>}
      </Group>
      {swarms.data?.length ? (
        <Group title={t('swarm.nav')}>
          {swarms.data.map((swarm) => (
            <NavLink
              key={swarm.id}
              to={`/swarms/${swarm.id}`}
              className={({ isActive }) => navItem(isActive)}
            >
              <StatusDot tone={swarm.reachable ? 'success' : 'warning'} />
              <span className="truncate">{swarmLabel(swarm, hosts.data)}</span>
              <span aria-hidden className="text-muted-foreground ms-auto text-xs tabular-nums">
                {swarm.managers.length + swarm.workers.length}
              </span>
            </NavLink>
          ))}
        </Group>
      ) : null}
      {clusters.data?.length ? (
        <Group
          title={t('app.nav.clusters')}
          manage="/clusters"
          manageLabel={t('hosts.switcher.manageClusters')}
        >
          {clusters.data.map((view) => {
            const summary = describeClusterStatus(view.status, view.cluster.kind);
            return (
              <FleetItem
                key={view.cluster.id}
                name={view.cluster.name}
                tone={summary.tone}
                status={summary.label}
                meta={
                  <span className="text-muted-foreground text-xs">
                    {view.cluster.kind === 'nomad' ? 'Nomad' : 'K8s'}
                  </span>
                }
                error={clusterStatusError(view.status)}
                selected={isSelected('cluster', view.cluster.id)}
                onSelect={() => switchTo({ kind: 'cluster', view })}
              >
                {pages}
              </FleetItem>
            );
          })}
        </Group>
      ) : null}
      <AddMenu />
    </>
  );
}

function Group({
  title,
  manage,
  manageLabel,
  children,
}: {
  title: string;
  manage?: string;
  manageLabel?: string;
  children: ReactNode;
}) {
  return (
    <section className="mt-4 grid gap-0.5" aria-label={title}>
      <div className="flex h-6 items-center justify-between ps-2.5 pe-1">
        <h2 className="text-muted-foreground text-xs font-medium">{title}</h2>
        {manage && (
          <Link
            to={manage}
            aria-label={manageLabel}
            title={manageLabel}
            className="text-muted-foreground hover:text-foreground focus-visible:ring-sidebar-ring/50 rounded-sm p-0.5 outline-none focus-visible:ring-2"
          >
            <Settings2Icon className="size-3.5" />
          </Link>
        )}
      </div>
      {children}
    </section>
  );
}

/**
 * A Host or a Cluster: its status shape, its name and a short count. A button
 * that selects it; once selected, its pages are listed beneath it.
 */
function FleetItem({
  name,
  tone,
  status,
  meta,
  error,
  selected,
  onSelect,
  children,
}: {
  name: string;
  tone: Tone;
  status: string;
  meta?: ReactNode;
  /** Why it can't be used, when it can't. */
  error?: AppErrorPayload | null;
  selected: boolean;
  onSelect: () => void;
  children: ReactNode;
}) {
  return (
    <div className={cn('grid gap-0.5', selected && 'mb-1')}>
      <div className="flex items-center">
        <button
          type="button"
          aria-expanded={selected}
          aria-label={t('app.nav.target', { name, status })}
          title={status}
          onClick={onSelect}
          className={cn(
            navItem(false),
            'w-full text-start',
            selected && 'text-sidebar-foreground font-semibold',
          )}
        >
          <StatusDot tone={tone} pulse={tone === 'info'} />
          <span className="min-w-0 flex-1 truncate">{name}</span>
          {meta}
        </button>
        {error && (
          <button
            type="button"
            aria-label={t('hosts.switcher.whyUnusable', { name })}
            title={t('hosts.statusTitle', { label: status, detail: error.message })}
            onClick={() => showErrorDetails(toAppError(error))}
            className="text-destructive hover:bg-sidebar-accent/60 focus-visible:ring-sidebar-ring/50 grid size-7 shrink-0 place-items-center rounded-md outline-none focus-visible:ring-2"
          >
            <TriangleAlertIcon className="size-4" />
          </button>
        )}
      </div>
      {selected && (
        <div className="border-sidebar-border ms-[0.8rem] grid gap-0.5 border-s ps-1.5">
          {children}
        </div>
      )}
    </div>
  );
}

/** A Docker Host's running containers out of all of them, once it is connected. */
function HostCount({ view }: { view: HostView }) {
  const connected = view.status.state === 'connected';
  const info = useQuery({
    ...resourceQueries.systemInfo(view.host.id),
    enabled: connected && supports(view, 'docker'),
  });
  if (!connected || !info.data) return null;
  return (
    <span
      className="text-muted-foreground text-xs tabular-nums"
      title={t('app.nav.running', { running: info.data.running, total: info.data.containers })}
    >
      {info.data.running}/{info.data.containers}
    </span>
  );
}

/** Adds a Host or a Cluster. */
function AddMenu() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        className={cn(navItem(false), 'text-muted-foreground mt-2 w-full')}
        aria-label={t('app.nav.add')}
      >
        <PlusIcon className="size-4" />
        {t('app.nav.add')}
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="w-56">
        <DropdownMenuItem asChild>
          <Link to="/hosts/new">{t('hosts.switcher.add')}</Link>
        </DropdownMenuItem>
        <DropdownMenuItem asChild>
          <Link to="/clusters/new">{t('hosts.switcher.addCluster')}</Link>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
