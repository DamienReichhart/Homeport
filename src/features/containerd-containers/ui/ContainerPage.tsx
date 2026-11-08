import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, TriangleAlertIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { ContainerdGate } from '@/core/host';
import {
  Alert,
  AlertDescription,
  Badge,
  Button,
  ErrorMessage,
  JsonView,
  KeyValue,
  PageHeader,
  Spinner,
  StatusDot,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { containerdContainerQueries } from '../api/queries';
import { managedByKubernetes, STATE_LABELS, stateTone } from '../model/containers';
import { CREATED_BY } from './containerColumns';
import { ExecTab } from './ExecTab';
import { LogsTab } from './LogsTab';
import { KillDialog } from './KillDialog';
import { RemoveContainerDialog } from './RemoveContainerDialog';
import { TaskActions } from './TaskActions';
import { t, type MessageKey } from '@/shared/i18n';

const TABS = ['overview', 'logs', 'exec', 'spec'] as const;
const TAB_LABELS: Record<(typeof TABS)[number], MessageKey> = {
  overview: 'containerd.containers.tab.overview',
  logs: 'containerd.containers.tab.logs',
  exec: 'containerd.containers.tab.exec',
  spec: 'containerd.containers.tab.spec',
};

function ContainerDetail({
  hostId,
  namespace,
  id,
}: {
  hostId: string;
  namespace: string;
  id: string;
}) {
  const { tab = 'overview' } = useParams();
  const navigate = useNavigate();
  const back = () => void navigate('/containerd/containers');
  const detail = useQuery(containerdContainerQueries.inspect(hostId, namespace, id));
  const [removing, setRemoving] = useState(false);
  const [killing, setKilling] = useState(false);

  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return (
      <div className="grid gap-3">
        <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('containerd.containers.backToList')}
        </Button>
      </div>
    );
  const { container, spec, extensions, updatedAt } = detail.data;
  const path = `/containerd/containers/${encodeURIComponent(container.id)}`;
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon />
            </Button>
            <StatusDot tone={stateTone(container.state)} />
            {container.name}
            <Badge variant="secondary">{t(CREATED_BY[container.createdBy])}</Badge>
          </span>
        }
        description={`${container.image} · ${t(STATE_LABELS[container.state])}`}
        actions={
          <TaskActions
            hostId={hostId}
            namespace={namespace}
            container={container}
            onKill={() => setKilling(true)}
            onRemove={() => setRemoving(true)}
          />
        }
      />
      {managedByKubernetes(container) && (
        <Alert className="mb-3">
          <TriangleAlertIcon />
          <AlertDescription>{t('containerd.containers.kubelet')}</AlertDescription>
        </Alert>
      )}
      <Tabs
        value={tab}
        onValueChange={(next) => void navigate(`${path}/${next}`, { replace: true })}
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          {TABS.map((value) => (
            <TabsTrigger key={value} value={value}>
              {t(TAB_LABELS[value])}
            </TabsTrigger>
          ))}
        </TabsList>
        <TabsContent value="overview" className="grid content-start gap-4 overflow-auto">
          <KeyValue
            items={[
              [t('containerd.containers.field.id'), <code key="id">{container.id}</code>],
              [t('containerd.containers.field.namespace'), namespace],
              [t('containerd.containers.field.image'), container.image || '—'],
              [t('containerd.containers.field.runtime'), container.runtime || '—'],
              [
                t('containerd.containers.field.snapshot'),
                container.snapshotKey ? `${container.snapshotter} · ${container.snapshotKey}` : '—',
              ],
              [t('containerd.containers.field.pid'), container.pid ?? '—'],
              [
                t('containerd.containers.field.exit'),
                container.exitStatus === null
                  ? '—'
                  : t('containerd.state.exited', { code: container.exitStatus }),
              ],
              [t('containerd.containers.field.pod'), container.pod ?? '—'],
              [t('containerd.containers.field.created'), container.createdAt ?? '—'],
              [t('containerd.containers.field.updated'), updatedAt ?? '—'],
              [t('containerd.containers.field.extensions'), extensions.join(', ') || '—'],
            ]}
          />
          {Object.keys(container.labels).length > 0 && (
            <div>
              <h3 className="mb-2 text-sm font-medium">{t('containerd.containers.labels')}</h3>
              <KeyValue
                items={Object.entries(container.labels).map(([k, v]) => [
                  k,
                  <code key={k} className="break-all">
                    {v}
                  </code>,
                ])}
              />
            </div>
          )}
        </TabsContent>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogsTab hostId={hostId} namespace={namespace} id={container.id} />
        </TabsContent>
        <TabsContent value="exec" className="min-h-[24rem] flex-1">
          <ExecTab
            hostId={hostId}
            namespace={namespace}
            id={container.id}
            running={container.state === 'running'}
          />
        </TabsContent>
        <TabsContent value="spec" className="overflow-auto">
          {spec ? (
            <JsonView value={spec} />
          ) : (
            <p className="text-muted-foreground text-sm">{t('containerd.containers.noSpec')}</p>
          )}
        </TabsContent>
      </Tabs>
      <RemoveContainerDialog
        hostId={hostId}
        namespace={namespace}
        container={removing ? container : null}
        onClose={() => setRemoving(false)}
        onRemoved={back}
      />
      <KillDialog
        hostId={hostId}
        namespace={namespace}
        container={killing ? container : null}
        onClose={() => setKilling(false)}
      />
    </div>
  );
}

export function ContainerPage() {
  const { id = '' } = useParams();
  return (
    <ContainerdGate>
      {(hostId, namespace) => (
        <ContainerDetail
          key={`${hostId}/${namespace}/${id}`}
          hostId={hostId}
          namespace={namespace}
          id={decodeURIComponent(id)}
        />
      )}
    </ContainerdGate>
  );
}
