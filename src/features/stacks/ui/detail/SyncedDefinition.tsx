import { useQuery } from '@tanstack/react-query';
import { RefreshCwIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCapabilities, type Stack, type SyncedStack } from '@/shared/api';
import {
  Button,
  DangerZone,
  EnvTextarea,
  ErrorMessage,
  Label,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import { syncedQueries, useForgetSynced, useSaveSyncedEnv } from '../../api/synced';
import type { ComposeJob } from '../../model/composeJob';
import { repositoryQueries } from '@/features/repositories';
import { SyncDialog } from '../dialogs/SyncDialog';
import { DeployButton } from './DeployButton';
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
  const [syncing, setSyncing] = useState(false);
  const canDeploy = deployTool(capabilities.data, owner)?.state === 'available';
  const dirty = env !== initialEnv;
  return (
    <div className="grid gap-4">
      <SyncedSource owner={owner} synced={synced} />
      <SyncedFilesView synced={synced} url={url} canDeploy={canDeploy} onDeploy={onDeploy} />
      <div className="grid gap-1.5">
        <Label htmlFor="synced-env-edit">{t('stacks.newSynced.env')}</Label>
        <EnvTextarea
          id="synced-env-edit"
          className="min-h-28"
          value={env}
          onChange={setEnv}
          disabled={!(capabilities.data?.keychain ?? true)}
        />
        <p className="text-muted-foreground text-xs">{t('stacks.newSynced.envHelp')}</p>
      </div>
      {save.error && <ErrorMessage error={save.error} />}
      <DeployButton
        label={t('stacks.action.sync')}
        icon={RefreshCwIcon}
        canDeploy={canDeploy}
        dirty={dirty}
        pending={save.isPending}
        onDeploy={() =>
          dirty ? save.mutate(env, { onSuccess: () => setSyncing(true) }) : setSyncing(true)
        }
        onSave={() => save.mutate(env)}
      />
      <DangerZone
        title={t('stacks.forget.sectionTitle')}
        description={t('stacks.synced.forgetDescription')}
        action={
          <Button
            variant="destructive"
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
        }
      />
      {syncing && (
        <SyncDialog
          stackId={synced.id}
          name={stack.name}
          onDeploy={onDeploy}
          onClose={() => setSyncing(false)}
        />
      )}
      {dialog}
    </div>
  );
}
