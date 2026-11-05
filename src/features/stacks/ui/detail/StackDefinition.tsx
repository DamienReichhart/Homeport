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
import { t, tRich } from '@/shared/i18n';

/** A Managed or Linked Stack's definition, or how to adopt an External one. */
export function StackDefinition({
  hostId,
  stack,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  if (stack.kind === 'linked' && stack.id)
    return <LinkedDefinition hostId={hostId} stack={stack} linkedId={stack.id} />;
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
      hostId={hostId}
      managed={managed.data}
      initialEnv={env.data}
      onDeploy={onDeploy}
    />
  );
}
