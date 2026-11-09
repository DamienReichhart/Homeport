import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { IncusGate } from '@/core/host';
import { formatBytes } from '@/shared/lib';
import {
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
import { incusInstanceQueries } from '../api/queries';
import { cpuSeconds, STATUS_LABELS, statusTone, TYPE_LABELS } from '../model/instances';
import { ConfigTable, DevicesList } from './ConfigTable';
import { DeleteInstanceDialog } from './DeleteInstanceDialog';
import { InstanceActions } from './InstanceActions';
import { SnapshotsTab } from './SnapshotsTab';
import { ConsoleTab, ExecTab } from './TerminalTabs';
import { t, type MessageKey } from '@/shared/i18n';

const TABS = ['overview', 'console', 'exec', 'snapshots', 'config', 'devices', 'raw'] as const;
const TAB_LABELS: Record<(typeof TABS)[number], MessageKey> = {
  overview: 'incus.instances.tab.overview',
  console: 'incus.instances.tab.console',
  exec: 'incus.instances.tab.exec',
  snapshots: 'incus.instances.tab.snapshots',
  config: 'incus.instances.tab.config',
  devices: 'incus.instances.tab.devices',
  raw: 'incus.instances.tab.raw',
};

function InstanceDetail({
  hostId,
  project,
  name,
}: {
  hostId: string;
  project: string;
  name: string;
}) {
  const { tab = 'overview' } = useParams();
  const navigate = useNavigate();
  const back = () => void navigate('/incus/instances');
  const detail = useQuery(incusInstanceQueries.detail(hostId, project, name));
  const [deleting, setDeleting] = useState(false);

  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return (
      <div className="grid gap-3">
        <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('incus.instances.backToList')}
        </Button>
      </div>
    );
  const { instance } = detail.data;
  const path = `/incus/instances/${encodeURIComponent(instance.name)}`;
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon />
            </Button>
            <StatusDot tone={statusTone(instance.status)} />
            {instance.name}
            <Badge variant="secondary">{t(TYPE_LABELS[instance.type])}</Badge>
          </span>
        }
        description={[instance.image, t(STATUS_LABELS[instance.status])]
          .filter(Boolean)
          .join(' · ')}
        actions={
          <InstanceActions
            hostId={hostId}
            project={project}
            instance={instance}
            onDelete={() => setDeleting(true)}
          />
        }
      />
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
              [t('incus.instances.field.project'), project],
              [t('incus.instances.field.architecture'), instance.architecture || '—'],
              [t('incus.instances.field.addresses'), instance.addresses.join(', ') || '—'],
              [
                t('incus.instances.field.cpu'),
                instance.cpuUsageNs === null
                  ? '—'
                  : t('incus.instances.cpuSeconds', { seconds: cpuSeconds(instance.cpuUsageNs) }),
              ],
              [
                t('incus.instances.field.memory'),
                instance.memoryBytes === null ? '—' : formatBytes(instance.memoryBytes),
              ],
              [t('incus.instances.field.processes'), instance.processes ?? '—'],
              [t('incus.instances.field.created'), instance.createdAt],
              [t('incus.instances.field.lastUsed'), instance.lastUsedAt ?? '—'],
              [t('incus.instances.field.snapshots'), detail.data.snapshotNames.join(', ') || '—'],
              [t('incus.instances.field.location'), instance.location || '—'],
              [t('incus.instances.field.description'), instance.description || '—'],
            ]}
          />
          <div>
            <h3 className="mb-2 text-sm font-medium">{t('incus.instances.profiles')}</h3>
            <div className="flex flex-wrap gap-1.5">
              {instance.profiles.length === 0 && (
                <span className="text-muted-foreground text-sm">—</span>
              )}
              {instance.profiles.map((profile) => (
                <Badge key={profile} variant="outline" asChild>
                  <Link to={`/incus/profiles/${encodeURIComponent(profile)}`}>{profile}</Link>
                </Badge>
              ))}
            </div>
          </div>
        </TabsContent>
        <TabsContent value="console" className="min-h-[24rem] flex-1">
          <ConsoleTab
            hostId={hostId}
            project={project}
            name={instance.name}
            running={instance.status === 'running'}
          />
        </TabsContent>
        <TabsContent value="exec" className="min-h-[24rem] flex-1">
          <ExecTab
            hostId={hostId}
            project={project}
            name={instance.name}
            running={instance.status === 'running'}
          />
        </TabsContent>
        <TabsContent value="snapshots" className="overflow-auto">
          <SnapshotsTab
            hostId={hostId}
            project={project}
            instance={instance}
            statefulAllowed={detail.data.statefulSnapshots}
          />
        </TabsContent>
        <TabsContent value="config" className="overflow-auto">
          <ConfigTable own={detail.data.config} expanded={detail.data.expandedConfig} />
        </TabsContent>
        <TabsContent value="devices" className="overflow-auto">
          <DevicesList own={detail.data.devices} expanded={detail.data.expandedDevices} />
        </TabsContent>
        <TabsContent value="raw" className="overflow-auto">
          <JsonView value={detail.data.raw} />
        </TabsContent>
      </Tabs>
      <DeleteInstanceDialog
        hostId={hostId}
        project={project}
        instance={deleting ? instance : null}
        snapshotNames={detail.data.snapshotNames}
        onClose={() => setDeleting(false)}
        onDeleted={back}
      />
    </div>
  );
}

export function InstancePage() {
  const { name = '' } = useParams();
  return (
    <IncusGate>
      {(hostId, project) => (
        <InstanceDetail
          key={`${hostId}/${project}/${name}`}
          hostId={hostId}
          project={project}
          name={decodeURIComponent(name)}
        />
      )}
    </IncusGate>
  );
}
