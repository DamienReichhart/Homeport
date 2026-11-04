import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { HostGate, resourceQueries } from '@/core/host';
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
import { STATE_LABEL, STATE_TONE } from '../../model/stackState';
import { ComposeRunDialog } from '../dialogs/ComposeRunDialog';
import { StackActions } from '../StackActions';
import { StackDefinition } from './StackDefinition';
import { StackServices } from './StackServices';
import { t } from '@/shared/i18n';

function StackDetail({ hostId, name }: { hostId: string; name: string }) {
  const stacks = useQuery(resourceQueries.stacks(hostId));
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [job, setJob] = useState<ComposeJob | null>(null);
  const stack = stacks.data?.find((s) => s.name === name);

  if (stacks.isPending) return <Spinner />;
  if (!stack)
    return <ErrorMessage error={{ code: 'not_found', message: t('stacks.notFound', { name }) }} />;
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
              <ArrowLeftIcon />
            </Button>
            {stack.name}
            <Badge variant={stack.kind === 'managed' ? 'default' : 'outline'}>
              {stack.kind === 'managed' ? t('stacks.kind.managed') : t('stacks.kind.external')}
            </Badge>
          </span>
        }
        description={
          <span className="flex items-center gap-1.5">
            <StatusDot tone={STATE_TONE[stack.state]} /> {t(STATE_LABEL[stack.state])}
            {stack.total > 0 &&
              ` · ${t('stacks.detail.running', { running: stack.running, total: stack.total })}`}
          </span>
        }
        actions={
          <StackActions hostId={hostId} stack={stack} confirm={confirm} onCompose={setJob} />
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
        </TabsList>
        <TabsContent value="services" className="overflow-auto">
          <StackServices hostId={hostId} stack={stack} />
        </TabsContent>
        <TabsContent value="logs" className="min-h-[24rem] flex-1">
          <LogViewer hostId={hostId} sources={sources} />
        </TabsContent>
        <TabsContent value="definition" className="overflow-auto">
          <StackDefinition hostId={hostId} stack={stack} onDeploy={setJob} />
        </TabsContent>
      </Tabs>
      <ComposeRunDialog hostId={hostId} job={job} onClose={() => setJob(null)} />
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
