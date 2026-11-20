import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  type Candidate,
  type ComposeCheck,
  type ManagedStack,
  useCapabilities,
} from '@/shared/api';
import { Button, DangerZone, ErrorMessage, useConfirm } from '@/shared/ui';
import { useForgetDefinition, useSaveDefinition } from '../../api/mutations';
import { sameFiles } from '../../model/composeFiles';
import type { ComposeJob } from '../../model/composeJob';
import { ReviewDialog } from '../dialogs/ReviewDialog';
import { StackEditor } from '../StackEditor';
import { DeployButton } from './DeployButton';
import { ProfilesControl } from './ProfilesControl';
import type { StackOwner } from '@/shared/api';
import { deployTool } from '../../model/deployTool';
import { t } from '@/shared/i18n';

/**
 * Edits and deploys the stored definition of a Managed Stack. Deploy saves,
 * shows what changes when it already runs, then deploys; saving without
 * deploying is one step away under it, and forgetting is set apart below.
 */
export function ManagedDefinition({
  owner,
  managed,
  initialEnv,
  deployed,
  onDeploy,
}: {
  owner: StackOwner;
  managed: ManagedStack;
  initialEnv: string;
  /** Whether it runs, so a redeploy shows what changes first. */
  deployed: boolean;
  onDeploy: (job: ComposeJob) => void;
}) {
  const [review, setReview] = useState<Candidate | null>(null);
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [dialog, confirm] = useConfirm();
  const [files, setFiles] = useState(managed.files);
  const [env, setEnv] = useState(initialEnv);
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  const dirty = !sameFiles(files, managed.files) || env !== initialEnv;
  const save = useSaveDefinition(managed.id, (saved, deploy) => {
    if (deploy) onDeploy({ kind: 'managed', stackId: saved.id, name: saved.name, action: 'up' });
  });
  const forget = useForgetDefinition(owner, managed.id);
  const canDeploy =
    deployTool(capabilities.data, owner)?.state === 'available' &&
    check !== null &&
    check.errors.length === 0;
  return (
    <div className="grid gap-4">
      <StackEditor
        files={files}
        env={env}
        onFilesChange={setFiles}
        onEnvChange={setEnv}
        keychain={capabilities.data?.keychain ?? true}
        onCheck={setCheck}
        swarm={owner.kind === 'swarm'}
      />
      <ProfilesControl
        key={managed.profiles.join()}
        kind="managed"
        stackId={managed.id}
        declared={check?.profiles ?? null}
        active={managed.profiles}
        canDeploy={canDeploy && !dirty}
        onDeploy={() =>
          onDeploy({ kind: 'managed', stackId: managed.id, name: managed.name, action: 'up' })
        }
      />
      {save.error && <ErrorMessage error={save.error} />}
      <DeployButton
        canDeploy={canDeploy}
        dirty={dirty}
        pending={save.isPending}
        onDeploy={() =>
          deployed ? setReview({ files, env }) : save.mutate({ files, env, deploy: true })
        }
        onSave={() => save.mutate({ files, env, deploy: false })}
      />
      <DangerZone
        title={t('stacks.forget.sectionTitle')}
        description={t('stacks.forget.description')}
        action={
          <Button
            variant="destructive"
            onClick={() =>
              confirm({
                title: t('stacks.forget.title', { name: managed.name }),
                description: t('stacks.forget.description'),
                confirmLabel: t('stacks.forget.confirm'),
                destructive: true,
                onConfirm: async () => {
                  await forget.mutateAsync();
                  navigate('/stacks');
                },
              })
            }
          >
            {t('stacks.forget.confirm')}
          </Button>
        }
      />
      <ReviewDialog
        kind="managed"
        stackId={managed.id}
        name={managed.name}
        candidate={review}
        onDeploy={() => save.mutate({ files, env, deploy: true })}
        onClose={() => setReview(null)}
      />
      {dialog}
    </div>
  );
}
