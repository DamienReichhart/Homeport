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
import { ManagedDefinition } from './ManagedDefinition';

/** A Managed Stack's editable definition, or how to adopt an External one. */
export function StackDefinition({
  hostId,
  stack,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  const id = stack.managedId;
  const managed = useQuery({ ...stackQueries.managed(id ?? ''), enabled: !!id });
  const env = useQuery({ ...stackQueries.env(id ?? ''), enabled: !!id });
  if (!id) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>External Stack</CardTitle>
          <CardDescription>
            It was deployed outside Homeport
            {stack.configFiles ? (
              <>
                {' '}
                from <span className="font-mono">{stack.configFiles}</span> on that machine
              </>
            ) : null}
            . You can start, stop and remove it; adopt it with its compose file to edit and
            redeploy.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Button asChild>
            <Link to={`/stacks/new?adopt=${encodeURIComponent(stack.name)}`}>Adopt</Link>
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
