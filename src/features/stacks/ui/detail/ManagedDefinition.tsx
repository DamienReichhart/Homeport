import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  type Candidate,
  type ComposeCheck,
  type ManagedStack,
  useCapabilities,
} from '@/shared/api';
import { Button, ErrorMessage, useConfirm } from '@/shared/ui';
import { useForgetDefinition, useSaveDefinition } from '../../api/mutations';
import { sameFiles } from '../../model/composeFiles';
import type { ComposeJob } from '../../model/composeJob';
import { ReviewDialog } from '../dialogs/ReviewDialog';
import { StackEditor } from '../StackEditor';
import { ProfilesControl } from './ProfilesControl';
import type { StackOwner } from '@/shared/api';
import { deployTool } from '../../model/deployTool';
import { t } from '@/shared/i18n';

/** Edits, saves, redeploys or forgets the stored definition of a Managed Stack. */
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
      <div className="flex flex-wrap gap-2">
        <Button
          disabled={save.isPending || !canDeploy}
          onClick={() =>
            deployed ? setReview({ files, env }) : save.mutate({ files, env, deploy: true })
          }
        >
          {t('stacks.managed.saveAndRedeploy')}
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !dirty}
          onClick={() => save.mutate({ files, env, deploy: false })}
        >
          {t('common.save')}
        </Button>
        <Button
          variant="ghost"
          className="ml-auto"
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
      </div>
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
