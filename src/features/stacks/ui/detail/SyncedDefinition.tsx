import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCapabilities, type Stack, type SyncedStack } from '@/shared/api';
import { Button, ErrorMessage, Label, Spinner, Textarea, useConfirm } from '@/shared/ui';
import { syncedQueries, useForgetSynced, useSaveSyncedEnv } from '../../api/synced';
import type { ComposeJob } from '../../model/composeJob';
import { repositoryQueries } from '@/features/repositories';
import { SyncedFilesView } from './SyncedFilesView';
import { SyncedSource } from './SyncedSource';
import type { StackOwner } from '@/shared/api';
import { deployTool } from '../../model/deployTool';
import { t } from '@/shared/i18n';

/**
 * A Synced Stack's definition. Its compose file changes only through git;
 * its environment is Homeport's, and stays editable here.
 */
export function SyncedDefinition({
  owner,
  stack,
  syncedId,
  onDeploy,
}: {
  owner: StackOwner;
  stack: Stack;
  syncedId: string;
  onDeploy: (job: ComposeJob) => void;
}) {
  const synced = useQuery(syncedQueries.stack(syncedId));
  const env = useQuery(syncedQueries.env(syncedId));
  if (synced.error)
    return <ErrorMessage error={synced.error} onRetry={() => void synced.refetch()} />;
  if (env.error) return <ErrorMessage error={env.error} onRetry={() => void env.refetch()} />;
  if (!synced.data || env.data === undefined) return <Spinner />;
  return (
    <Loaded
      key={synced.data.updatedAt}
      onDeploy={onDeploy}
      owner={owner}
      stack={stack}
      synced={synced.data}
      initialEnv={env.data}
    />
  );
}

function Loaded({
  owner,
  stack,
  synced,
  initialEnv,
  onDeploy,
}: {
  onDeploy: (job: ComposeJob) => void;
  owner: StackOwner;
  stack: Stack;
  synced: SyncedStack;
  initialEnv: string;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [dialog, confirm] = useConfirm();
  const [env, setEnv] = useState(initialEnv);
  const save = useSaveSyncedEnv(synced.id);
  const forget = useForgetSynced(owner, synced.id);
  const repositories = useQuery(repositoryQueries.list());
  const url = repositories.data?.find((r) => r.id === synced.repositoryId)?.url ?? null;
  return (
    <div className="grid gap-4">
      <SyncedSource owner={owner} synced={synced} />
      <SyncedFilesView
        synced={synced}
        url={url}
        canDeploy={deployTool(capabilities.data, owner)?.state === 'available'}
        onDeploy={onDeploy}
      />
      <div className="grid gap-1.5">
        <Label htmlFor="synced-env-edit">{t('stacks.newSynced.env')}</Label>
        <Textarea
          id="synced-env-edit"
          className="font-mono text-xs"
          rows={5}
          value={env}
          onChange={(e) => setEnv(e.target.value)}
          disabled={!(capabilities.data?.keychain ?? true)}
          spellCheck={false}
        />
        <p className="text-muted-foreground text-xs">{t('stacks.newSynced.envHelp')}</p>
      </div>
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex flex-wrap gap-2">
        <Button
          variant="outline"
          disabled={save.isPending || env === initialEnv}
          onClick={() => save.mutate(env)}
        >
          {t('common.save')}
        </Button>
        <Button
          variant="ghost"
          className="ms-auto"
          onClick={() =>
            confirm({
              title: t('stacks.synced.forgetTitle', { name: stack.name }),
              description: t('stacks.synced.forgetDescription'),
              confirmLabel: t('stacks.synced.forget'),
              destructive: true,
              onConfirm: async () => {
                await forget.mutateAsync();
                navigate('/stacks');
              },
            })
          }
        >
          {t('stacks.synced.forget')}
        </Button>
      </div>
      {dialog}
    </div>
  );
}
