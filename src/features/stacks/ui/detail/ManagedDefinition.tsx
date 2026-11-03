import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { type ComposeCheck, type ManagedStack, useCapabilities } from '@/shared/api';
import { Button, ErrorMessage, useConfirm } from '@/shared/ui';
import { useForgetDefinition, useSaveDefinition } from '../../api/mutations';
import type { ComposeJob } from '../../model/composeJob';
import { StackEditor } from '../StackEditor';

/** Edits, saves, redeploys or forgets the stored definition of a Managed Stack. */
export function ManagedDefinition({
  hostId,
  managed,
  initialEnv,
  onDeploy,
}: {
  hostId: string;
  managed: ManagedStack;
  initialEnv: string;
  onDeploy: (job: ComposeJob) => void;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [dialog, confirm] = useConfirm();
  const [compose, setCompose] = useState(managed.compose);
  const [env, setEnv] = useState(initialEnv);
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  const dirty = compose !== managed.compose || env !== initialEnv;
  const save = useSaveDefinition(managed.id, (saved, deploy) => {
    if (deploy) onDeploy({ stackId: saved.id, name: saved.name, action: 'up' });
  });
  const forget = useForgetDefinition(hostId, managed.id);
  const canDeploy =
    capabilities.data?.compose.state === 'available' && check !== null && check.errors.length === 0;
  return (
    <div className="grid gap-4">
      <StackEditor
        compose={compose}
        env={env}
        onComposeChange={setCompose}
        onEnvChange={setEnv}
        keychain={capabilities.data?.keychain ?? true}
        onCheck={setCheck}
      />
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex flex-wrap gap-2">
        <Button
          disabled={save.isPending || !canDeploy}
          onClick={() => save.mutate({ compose, env, deploy: true })}
        >
          Save and redeploy
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !dirty}
          onClick={() => save.mutate({ compose, env, deploy: false })}
        >
          Save
        </Button>
        <Button
          variant="ghost"
          className="ml-auto"
          onClick={() =>
            confirm({
              title: `Forget the definition of ${managed.name}?`,
              description:
                'Homeport deletes its stored compose file and environment. Running containers are left alone and the Stack becomes External.',
              confirmLabel: 'Forget definition',
              destructive: true,
              onConfirm: async () => {
                await forget.mutateAsync();
                navigate('/stacks');
              },
            })
          }
        >
          Forget definition
        </Button>
      </div>
      {dialog}
    </div>
  );
}
