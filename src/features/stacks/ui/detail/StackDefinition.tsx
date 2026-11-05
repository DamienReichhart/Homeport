import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import type { Stack } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Spinner,
} from '@/shared/ui';
import { stackQueries } from '../../api/queries';
import type { ComposeJob } from '../../model/composeJob';
import { LinkedDefinition } from './LinkedDefinition';
import { ManagedDefinition } from './ManagedDefinition';
import { SyncedDefinition } from './SyncedDefinition';
import { t, tRich } from '@/shared/i18n';

/** A Managed, Linked or Synced Stack's definition, or how to adopt an External one. */
export function StackDefinition({
  hostId,
  stack,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  // A Linked Stack, or one shown as External while its files are missing.
  if (stack.id && (stack.kind === 'linked' || stack.problem === 'filesMissing'))
    return (
      <LinkedDefinition hostId={hostId} stack={stack} linkedId={stack.id} onDeploy={onDeploy} />
    );
  if (stack.id && stack.kind === 'synced')
    return <SyncedDefinition hostId={hostId} stack={stack} syncedId={stack.id} />;
  return <OwnDefinition hostId={hostId} stack={stack} onDeploy={onDeploy} />;
}

function OwnDefinition({
  hostId,
  stack,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  const id = stack.kind === 'managed' ? stack.id : null;
  const managed = useQuery({ ...stackQueries.managed(id ?? ''), enabled: !!id });
  const env = useQuery({ ...stackQueries.env(id ?? ''), enabled: !!id });
  if (!id) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>{t('stacks.external.title')}</CardTitle>
          <CardDescription>
            {stack.configFiles
              ? tRich('stacks.external.descriptionFrom', {
                  files: stack.configFiles,
                  mono: (chunks) => <span className="font-mono">{chunks}</span>,
                })
              : t('stacks.external.description')}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Button asChild>
            <Link to={`/stacks/new?adopt=${encodeURIComponent(stack.name)}`}>
              {t('stacks.action.adopt')}
            </Link>
          </Button>
        </CardContent>
      </Card>
    );
  }
  if (managed.error)
    return <ErrorMessage error={managed.error} onRetry={() => void managed.refetch()} />;
  if (env.error) return <ErrorMessage error={env.error} onRetry={() => void env.refetch()} />;
  if (!managed.data || env.data === undefined) return <Spinner />;
  return (
    <ManagedDefinition
      key={managed.data.updatedAt}
      deployed={stack.state !== 'notDeployed'}
      hostId={hostId}
      managed={managed.data}
      initialEnv={env.data}
      onDeploy={onDeploy}
    />
  );
}
