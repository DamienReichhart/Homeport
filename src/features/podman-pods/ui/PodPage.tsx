import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, PlusIcon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { HostGate } from '@/core/host';
import { RunContainerDialog } from '@/features/containers';
import type { PodmanPodDetail } from '@/shared/api';
import { formatRelative, shortId } from '@/shared/lib';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  useConfirmedAction,
} from '@/shared/ui';
import { podQueries } from '../api/queries';
import { createdSeconds, members, podStatusLabel, podTone } from '../model/pods';
import { PodActions } from './PodActions';
import { t } from '@/shared/i18n';

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="grid grid-cols-[10rem_1fr] gap-2 py-1 text-sm">
      <dt className="text-muted-foreground">{label}</dt>
      <dd className="min-w-0">{children}</dd>
    </div>
  );
}

/** The pod's containers, each linking to its container page; the infra container last. */
function Containers({ pod }: { pod: PodmanPodDetail }) {
  const infra = pod.containers.filter((c) => c.infra);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('podmanpods.detail.containers', { count: members(pod).length })}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-1">
        {members(pod).length === 0 && (
          <p className="text-muted-foreground text-sm">{t('podmanpods.detail.noContainers')}</p>
        )}
        {[...members(pod), ...infra].map((c) => (
          <Link
            key={c.id}
            to={`/containers/${c.id}/logs`}
            className="hover:bg-muted/60 flex items-center gap-2 rounded-md px-2 py-1.5 text-sm"
          >
            <StatusDot tone={c.state === 'running' ? 'success' : 'muted'} />
            <span className="font-medium">{c.name}</span>
            <span className="text-muted-foreground font-mono text-xs">{shortId(c.id)}</span>
            {c.infra && <Badge variant="secondary">{t('podmanpods.detail.infra')}</Badge>}
            <span className="text-muted-foreground ms-auto">{c.state}</span>
          </Link>
        ))}
      </CardContent>
    </Card>
  );
}

function Details({ pod }: { pod: PodmanPodDetail }) {
  const labels = Object.entries(pod.labels);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('podmanpods.detail.settings')}</CardTitle>
      </CardHeader>
      <CardContent>
        <dl>
          <Row label={t('podmanpods.detail.ports')}>
            {pod.ports.length === 0 ? (
              <span className="text-muted-foreground">{t('podmanpods.detail.noPorts')}</span>
            ) : (
              <span className="flex flex-wrap gap-1 font-mono text-xs">
                {pod.ports.map((p) => (
                  <Badge key={`${p.containerPort}/${p.protocol}:${p.hostPort}`} variant="outline">
                    {p.hostPort
                      ? `${p.hostIp ? `${p.hostIp}:` : ''}${p.hostPort} → ${p.containerPort}/${p.protocol}`
                      : `${p.containerPort}/${p.protocol}`}
                  </Badge>
                ))}
              </span>
            )}
          </Row>
          <Row label={t('podmanpods.detail.shared')}>{pod.sharedNamespaces.join(', ') || '—'}</Row>
          <Row label={t('podmanpods.detail.networks')}>{pod.networks.join(', ') || '—'}</Row>
          <Row label={t('podmanpods.detail.hostname')}>{pod.hostname || '—'}</Row>
          <Row label={t('podmanpods.detail.created')}>{formatRelative(createdSeconds(pod))}</Row>
          <Row label={t('podmanpods.detail.labels')}>
            {labels.length === 0 ? (
              '—'
            ) : (
              <span className="flex flex-wrap gap-1">
                {labels.map(([k, v]) => (
                  <Badge key={k} variant="outline" className="font-mono">
                    {k}={v}
                  </Badge>
                ))}
              </span>
            )}
          </Row>
          {pod.createCommand.length > 0 && (
            <Row label={t('podmanpods.detail.command')}>
              <code className="font-mono text-xs break-all">{pod.createCommand.join(' ')}</code>
            </Row>
          )}
        </dl>
      </CardContent>
    </Card>
  );
}

function PodDetailView({ hostId, id }: { hostId: string; id: string }) {
  const navigate = useNavigate();
  const pod = useQuery(podQueries.inspect(hostId, id));
  const [dialog, confirm] = useConfirmedAction();
  const [running, setRunning] = useState(false);
  const back = () => void navigate('/podman-pods');

  if (pod.isPending) return <Spinner />;
  if (pod.error) return <ErrorMessage error={pod.error} onRetry={() => void pod.refetch()} />;
  const p = pod.data;
  return (
    <div className="grid gap-4">
      <div>
        <Button variant="ghost" size="sm" className="-ms-2 mb-2" onClick={back}>
          <ArrowLeftIcon className="rtl:-scale-x-100" /> {t('podmanpods.title')}
        </Button>
        <PageHeader
          title={
            <span className="flex items-center gap-2">
              <StatusDot tone={podTone(p.status)} />
              {p.name}
            </span>
          }
          description={t(podStatusLabel(p.status))}
          actions={
            <>
              <Button onClick={() => setRunning(true)}>
                <PlusIcon /> {t('podmanpods.detail.runContainer')}
              </Button>
              <PodActions hostId={hostId} pod={p} confirm={confirm} onRemoved={back} />
            </>
          }
        />
      </div>
      <Containers pod={p} />
      <Details pod={p} />
      <RunContainerDialog
        hostId={hostId}
        open={running}
        onOpenChange={setRunning}
        pod={p.name}
        title={t('podmanpods.detail.runContainerIn', { name: p.name })}
      />
      {dialog}
    </div>
  );
}

export function PodPage() {
  const { id = '' } = useParams();
  return (
    <HostGate requires="podman">
      {(hostId) => <PodDetailView key={`${hostId}:${id}`} hostId={hostId} id={id} />}
    </HostGate>
  );
}
