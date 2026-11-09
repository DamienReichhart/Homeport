import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { HostGate, indexFreshness, resourceQueries, useContainers } from '@/core/host';
import { LogViewer } from '@/features/containers';
import {
  Badge,
  Button,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  useConfirm,
} from '@/shared/ui';
import type { ComposeJob } from '../../model/composeJob';
import { serviceFreshness, stackFreshness } from '../../model/freshness';
import { KIND_BADGE, KIND_LABEL } from '../../model/stackKind';
import { STATE_LABEL, STATE_TONE } from '../../model/stackState';
import { ComposeRunDialog } from '../dialogs/ComposeRunDialog';
import { StackActions } from '../StackActions';
import { SyncBadge } from '../SyncBadge';
import { UpdateBadge } from '../UpdateBadge';
import { StackDefinition } from './StackDefinition';
import { StackHistory } from './StackHistory';
import { StackServices } from './StackServices';
import { hostOwner } from '../../api/owner';
import { t } from '@/shared/i18n';

function StackDetail({ hostId, name }: { hostId: string; name: string }) {
  const stacks = useQuery(resourceQueries.stacks(hostId));
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [job, setJob] = useState<ComposeJob | null>(null);
  const stack = stacks.data?.find((s) => s.name === name);
  const containers = useContainers(hostId);
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);

  if (stacks.isPending) return <Spinner />;
  if (!stack)
    return <ErrorMessage error={{ code: 'not_found', message: t('stacks.notFound', { name }) }} />;
  const byId = new Map(containers.data?.map((c) => [c.id, c]));
  const { newer } = stackFreshness(serviceFreshness(stack, byId, freshness));
  // Managed and Linked Stacks keep a history of their deploys.
  const recorded = stack.kind !== 'external' ? stack.id : null;
  const sources = stack.services.flatMap((s) =>
    s.containerIds.map((containerId) => ({ containerId, label: s.name })),
  );

  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('common.back')}
              onClick={() => navigate('/stacks')}
            >
              <ArrowLeftIcon className="rtl:-scale-x-100" />
            </Button>
            {stack.name}
            <Badge variant={KIND_BADGE[stack.kind]}>{t(KIND_LABEL[stack.kind])}</Badge>
            {newer.length > 0 && <UpdateBadge newer={newer} />}
            {stack.updateAvailable && <SyncBadge />}
          </span>
        }
        description={
          <span className="flex items-center gap-1.5">
            <StatusDot tone={STATE_TONE[stack.state]} /> {t(STATE_LABEL[stack.state])}
            {stack.total > 0 &&
              ` · ${t('stacks.detail.running', { running: stack.running, total: stack.total })}`}
            {stack.workingDir && <span className="font-mono text-xs"> · {stack.workingDir}</span>}
          </span>
        }
        actions={
          <StackActions
            hostId={hostId}
            stack={stack}
            newer={newer}
            confirm={confirm}
            onCompose={setJob}
          />
        }
      />
      <Tabs
        defaultValue={stack.state === 'notDeployed' ? 'definition' : 'services'}
        className="flex min-h-0 flex-1 flex-col"
      >
        <TabsList>
          <TabsTrigger value="services">{t('stacks.tab.services')}</TabsTrigger>
          <TabsTrigger value="logs" disabled={sources.length === 0}>
            {t('stacks.tab.logs')}
          </TabsTrigger>
          <TabsTrigger value="definition">{t('stacks.tab.definition')}</TabsTrigger>
          {recorded && <TabsTrigger value="history">{t('stacks.tab.history')}</TabsTrigger>}
        </TabsList>
        <TabsContent value="services" className="overflow-auto">
          <StackServices hostId={hostId} stack={stack} />
        </TabsContent>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogViewer hostId={hostId} sources={sources} />
        </TabsContent>
        <TabsContent value="definition" className="overflow-auto">
          <StackDefinition owner={hostOwner(hostId)} stack={stack} onDeploy={setJob} />
        </TabsContent>
        {recorded && (
          <TabsContent value="history" className="overflow-auto">
            <StackHistory
              owner={hostOwner(hostId)}
              stack={stack}
              stackId={recorded}
              onDeploy={setJob}
            />
          </TabsContent>
        )}
      </Tabs>
      <ComposeRunDialog owner={hostOwner(hostId)} job={job} onClose={() => setJob(null)} />
      {dialog}
    </div>
  );
}

export function StackDetailPage() {
  const { name = '' } = useParams();
  return (
    <HostGate>
      {(hostId) => <StackDetail key={`${hostId}/${name}`} hostId={hostId} name={name} />}
    </HostGate>
  );
}
