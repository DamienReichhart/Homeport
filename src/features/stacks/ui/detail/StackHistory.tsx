import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useCapabilities, type Stack } from '@/shared/api';
import { formatDateTime } from '@/shared/lib';
import {
  Badge,
  Button,
  ErrorMessage,
  Spinner,
  StatusDot,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  useConfirm,
  type Tone,
} from '@/shared/ui';
import { stackQueries } from '../../api/queries';
import type { ComposeJob } from '../../model/composeJob';
import { fileName } from '../../model/linkedEdit';
import { ACTION_LABEL, currentDeploy, outcomeOf, type Outcome } from '../../model/history';
import { t, type MessageKey } from '@/shared/i18n';

const OUTCOME_LABEL: Record<Outcome, MessageKey> = {
  succeeded: 'stacks.history.succeeded',
  failed: 'stacks.history.failed',
  unfinished: 'stacks.history.unfinished',
};

const OUTCOME_TONE: Record<Outcome, Tone> = {
  succeeded: 'success',
  failed: 'danger',
  unfinished: 'muted',
};

/**
 * The deploys of a Managed, Linked or Synced Stack, newest first, with the
 * definition each used; a Managed Stack can be rolled back to one.
 */
export function StackHistory({
  hostId,
  stack,
  stackId,
  onDeploy,
}: {
  hostId: string;
  stack: Stack;
  stackId: string;
  onDeploy: (job: ComposeJob) => void;
}) {
  const deploys = useQuery(stackQueries.history(hostId, stackId));
  const [selected, setSelected] = useState<string | null>(null);
  if (deploys.isPending) return <Spinner />;
  if (deploys.error)
    return <ErrorMessage error={deploys.error} onRetry={() => void deploys.refetch()} />;
  if (deploys.data.length === 0)
    return <p className="text-muted-foreground text-sm">{t('stacks.history.empty')}</p>;
  const current = currentDeploy(deploys.data);

  return (
    <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(0,1fr)]">
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t('stacks.history.column.started')}</TableHead>
              <TableHead>{t('stacks.history.column.action')}</TableHead>
              <TableHead>{t('stacks.history.column.outcome')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {deploys.data.map((d) => {
              const outcome = outcomeOf(d);
              return (
                <TableRow
                  key={d.id}
                  data-state={selected === d.id ? 'selected' : undefined}
                  className="cursor-pointer"
                  onClick={() => setSelected(d.id)}
                >
                  <TableCell>
                    <button
                      type="button"
                      className="text-left hover:underline"
                      onClick={() => setSelected(d.id)}
                    >
                      {formatDateTime(Date.parse(d.startedAt) / 1000)}
                    </button>
                    {d.id === current && (
                      <Badge variant="secondary" className="ml-2">
                        {t('stacks.history.current')}
                      </Badge>
                    )}
                  </TableCell>
                  <TableCell>
                    {t(ACTION_LABEL[d.action])}
                    {d.commit && (
                      <span className="text-muted-foreground ml-1.5 font-mono text-xs">
                        {d.commit.slice(0, 7)}
                      </span>
                    )}
                  </TableCell>
                  <TableCell>
                    <span className="flex items-center gap-1.5">
                      <StatusDot tone={OUTCOME_TONE[outcome]} />
                      {outcome === 'failed' && d.exitCode !== null
                        ? t('stacks.history.failedCode', { code: d.exitCode })
                        : t(OUTCOME_LABEL[outcome])}
                    </span>
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </div>
      {selected ? (
        <DeployView key={selected} deployId={selected} stack={stack} onDeploy={onDeploy} />
      ) : (
        <p className="text-muted-foreground text-sm">{t('stacks.history.select')}</p>
      )}
    </div>
  );
}

/** One recorded deploy, read-only: its files, the names of its variables, its profiles. */
function DeployView({
  deployId,
  stack,
  onDeploy,
}: {
  deployId: string;
  stack: Stack;
  onDeploy: (job: ComposeJob) => void;
}) {
  const detail = useQuery(stackQueries.deploy(deployId));
  const capabilities = useCapabilities();
  const [dialog, confirm] = useConfirm();
  if (detail.isPending) return <Spinner />;
  if (detail.error) return <ErrorMessage error={detail.error} />;
  const { deploy, envKeys } = detail.data;
  const managed = deploy.summary.kind === 'managed';
  const canRollBack =
    managed && envKeys !== null && capabilities.data?.compose.state === 'available';

  return (
    <div className="grid content-start gap-3">
      {deploy.files.map((f) => (
        <div key={f.name} className="grid gap-1">
          <span className="font-mono text-xs" title={f.name}>
            {fileName(f.name)}
          </span>
          <pre className="bg-muted max-h-72 overflow-auto rounded-md p-3 font-mono text-xs">
            {f.content}
          </pre>
        </div>
      ))}
      <p className="text-muted-foreground text-xs">
        {envKeys === null
          ? t('stacks.history.envLocked')
          : envKeys.length > 0
            ? t('stacks.history.envKeys', { keys: envKeys.join(', ') })
            : t('stacks.history.noEnv')}
      </p>
      {deploy.profiles.length > 0 && (
        <p className="text-muted-foreground text-xs">
          {t('stacks.history.profiles', { profiles: deploy.profiles.join(', ') })}
        </p>
      )}
      {managed ? (
        <Button
          className="justify-self-start"
          disabled={!canRollBack}
          onClick={() =>
            confirm({
              title: t('stacks.history.rollBackTitle', { name: stack.name }),
              description: t('stacks.history.rollBackDescription'),
              confirmLabel: t('stacks.history.rollBack'),
              onConfirm: () =>
                onDeploy({
                  kind: 'managed',
                  stackId: deploy.summary.stackId,
                  name: stack.name,
                  action: 'up',
                  rollbackTo: deploy.summary.id,
                }),
            })
          }
        >
          {t('stacks.history.rollBack')}
        </Button>
      ) : (
        <p className="text-muted-foreground text-xs">{t('stacks.history.linkedNote')}</p>
      )}
      {dialog}
    </div>
  );
}
