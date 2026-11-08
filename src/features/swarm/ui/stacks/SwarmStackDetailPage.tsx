import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, Trash2Icon } from 'lucide-react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { SwarmGate } from '@/core/swarm';
import { KIND_BADGE, KIND_LABEL, STATE_LABEL, STATE_TONE } from '@/features/stacks';
import type { SwarmView } from '@/shared/api';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  StatusDot,
  useConfirmedAction,
} from '@/shared/ui';
import { useRemoveSwarmStack } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

function SwarmStackDetail({ swarm, name }: { swarm: SwarmView; name: string }) {
  const navigate = useNavigate();
  const back = () => void navigate(`/swarms/${swarm.id}/stacks`);
  const stacks = useQuery(swarmFeatureQueries.stacks(swarm.id));
  const parts = useQuery(swarmFeatureQueries.stackParts(swarm.id, name));
  const remove = useRemoveSwarmStack(swarm.id);
  const [dialog, confirm] = useConfirmedAction();
  const stack = stacks.data?.find((s) => s.name === name);

  if (stacks.isPending) return <Spinner />;
  if (!stack)
    return (
      <div className="grid gap-3">
        <ErrorMessage error={{ code: 'not_found', message: t('stacks.notFound', { name }) }} />
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('swarm.stacks.backToList')}
        </Button>
      </div>
    );
  const names = (list: string[] | undefined) => (list?.length ? list.join(', ') : '—');
  return (
    <div className="grid gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon />
            </Button>
            {stack.name}
            <Badge variant={KIND_BADGE[stack.kind]}>{t(KIND_LABEL[stack.kind])}</Badge>
            <Badge variant="outline">{t('swarm.badge')}</Badge>
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
          stack.state !== 'notDeployed' && (
            <Button
              variant="outline"
              onClick={() =>
                confirm({
                  title: t('swarm.stacks.remove.title', { name: stack.name }),
                  description: t('swarm.stacks.remove.description'),
                  confirmLabel: t('common.remove'),
                  destructive: true,
                  action: () => remove.mutateAsync(stack.name),
                })
              }
            >
              <Trash2Icon /> {t('common.remove')}
            </Button>
          )
        }
      />
      <p className="text-muted-foreground text-sm">{t('swarm.stacks.noStartStop')}</p>
      <Card>
        <CardHeader>
          <CardTitle>{t('swarm.stacks.services')}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-1.5">
          {stack.services.length === 0 && (
            <p className="text-muted-foreground text-sm">{t('swarm.stacks.noServices')}</p>
          )}
          {stack.services.map((s) => (
            <div key={s.serviceId ?? s.name} className="flex items-center gap-2 text-sm">
              <StatusDot
                tone={
                  s.desired !== null && s.running < s.desired
                    ? 'warning'
                    : s.desired === 0
                      ? 'muted'
                      : 'success'
                }
              />
              {s.serviceId ? (
                <Link
                  to={`/swarms/${swarm.id}/services/${s.serviceId}`}
                  className="font-medium hover:underline"
                >
                  {s.name}
                </Link>
              ) : (
                <span className="font-medium">{s.name}</span>
              )}
              <span className="text-muted-foreground">
                {s.running}/{s.desired ?? '?'}
              </span>
            </div>
          ))}
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('swarm.stacks.parts')}</CardTitle>
        </CardHeader>
        <CardContent>
          {parts.error ? (
            <ErrorMessage error={parts.error} onRetry={() => void parts.refetch()} />
          ) : (
            <KeyValue
              items={[
                [t('swarm.stacks.networks'), names(parts.data?.networks)],
                [t('swarm.tab.secrets'), names(parts.data?.secrets)],
                [t('swarm.tab.configs'), names(parts.data?.configs)],
              ]}
            />
          )}
        </CardContent>
      </Card>
      {dialog}
    </div>
  );
}

export function SwarmStackDetailPage() {
  const { swarmId = '', name = '' } = useParams();
  return (
    <SwarmGate key={swarmId} swarmId={swarmId}>
      {(swarm) => <SwarmStackDetail key={name} swarm={swarm} name={name} />}
    </SwarmGate>
  );
}
