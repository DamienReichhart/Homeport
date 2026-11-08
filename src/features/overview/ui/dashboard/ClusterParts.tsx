import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { age, HealthBadge, kindPath, objectPath } from '@/core/cluster';
import type { EventView, NodeCounts, Part, PodCounts, WorkloadView } from '@/shared/api';
import { Badge, Card, CardContent, CardHeader, CardTitle, StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A dashboard card whose part may not have been allowed. */
function PartCard<T>({
  title,
  part,
  children,
}: {
  title: string;
  part: Part<T>;
  children: (value: T) => ReactNode;
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-2 text-sm">
        {part.state === 'read' ? (
          children(part.value)
        ) : (
          <p className="text-muted-foreground" role="note">
            {t('overview.cluster.notAllowed')} {part.error.message}
          </p>
        )}
      </CardContent>
    </Card>
  );
}

export function NodesCard({ part }: { part: Part<NodeCounts> }) {
  return (
    <PartCard title={t('overview.cluster.nodesTitle')} part={part}>
      {(n) => (
        <>
          <p className="text-2xl font-semibold">
            {n.ready}
            <span className="text-muted-foreground text-base font-normal"> / {n.total}</span>
          </p>
          <p className="text-muted-foreground">{t('overview.cluster.nodesReady')}</p>
          {n.notReady.map((name) => (
            <Link
              key={name}
              to={objectPath('node', null, name) ?? '/nodes'}
              className="flex items-center gap-1.5"
            >
              <StatusDot tone="danger" /> {name}
            </Link>
          ))}
          <Link to="/nodes" className="text-primary text-sm hover:underline">
            {t('overview.cluster.allNodes')}
          </Link>
        </>
      )}
    </PartCard>
  );
}

export function PodsCard({ part }: { part: Part<PodCounts> }) {
  return (
    <PartCard title={t('overview.cluster.podsTitle')} part={part}>
      {(p) => (
        <>
          <p className="text-2xl font-semibold">{p.total}</p>
          <ul className="grid grid-cols-2 gap-1">
            <li className="flex items-center gap-1.5">
              <StatusDot tone="success" />
              {t('overview.cluster.running', { count: p.running })}
            </li>
            <li className="flex items-center gap-1.5">
              <StatusDot tone="info" />
              {t('overview.cluster.pending', { count: p.pending })}
            </li>
            <li className="flex items-center gap-1.5">
              <StatusDot tone="danger" />
              {t('overview.cluster.crashing', { count: p.crashing + p.failed })}
            </li>
            <li className="flex items-center gap-1.5">
              <StatusDot tone="muted" />
              {t('overview.cluster.done', { count: p.succeeded + p.other })}
            </li>
          </ul>
          <Link to="/pods" className="text-primary text-sm hover:underline">
            {t('overview.cluster.allPods')}
          </Link>
        </>
      )}
    </PartCard>
  );
}

export function UnreadyCard({ part }: { part: Part<WorkloadView[]> }) {
  return (
    <PartCard title={t('overview.cluster.unreadyTitle')} part={part}>
      {(list) =>
        list.length === 0 ? (
          <p className="text-muted-foreground">{t('overview.cluster.allReady')}</p>
        ) : (
          list.map((w) => (
            <Link
              key={`${w.kind}/${w.namespace}/${w.name}`}
              to={objectPath(w.kind, w.namespace, w.name) ?? '/workloads'}
              className="flex items-center justify-between gap-2"
            >
              <span className="truncate">
                {w.namespace}/{w.name}
              </span>
              <span className="flex items-center gap-2">
                <span className="text-muted-foreground">
                  {w.ready}/{w.desired}
                </span>
                <HealthBadge workload={w.health} />
              </span>
            </Link>
          ))
        )
      }
    </PartCard>
  );
}

export function WarningsCard({ part }: { part: Part<EventView[]> }) {
  return (
    <PartCard title={t('overview.cluster.warningsTitle')} part={part}>
      {(list) =>
        list.length === 0 ? (
          <p className="text-muted-foreground">{t('overview.cluster.noWarnings')}</p>
        ) : (
          <>
            {list.map((e) => {
              const to = kindPath(e.object.kind, e.object.namespace, e.object.name);
              return (
                <div key={`${e.namespace}/${e.name}`} className="grid gap-0.5">
                  <span className="flex items-center gap-2">
                    <Badge variant="destructive">{e.reason}</Badge>
                    {to ? (
                      <Link to={to}>{`${e.object.kind}/${e.object.name}`}</Link>
                    ) : (
                      `${e.object.kind}/${e.object.name}`
                    )}
                    <span className="text-muted-foreground ml-auto text-xs">{age(e.lastSeen)}</span>
                  </span>
                  <span className="text-muted-foreground break-words">{e.message}</span>
                </div>
              );
            })}
            <Link to="/cluster-events" className="text-primary text-sm hover:underline">
              {t('overview.cluster.allEvents')}
            </Link>
          </>
        )
      }
    </PartCard>
  );
}
