import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { HostGate, useCapabilities } from '@/features/hosts';
import { commands, keys, type ComposeCheck } from '@/shared/api';
import { Alert, AlertDescription, AlertTitle } from '@/shared/ui/alert';
import { Button } from '@/shared/ui/button';
import { Input } from '@/shared/ui/input';
import { Label } from '@/shared/ui/label';
import { ErrorMessage, PageHeader } from '@/shared/ui';
import { ComposeRunDialog, type ComposeJob } from './ComposeRunDialog';
import { STARTER, StackEditor } from './StackEditor';

function NewStack({ hostId }: { hostId: string }) {
  const [params] = useSearchParams();
  const adopting = params.get('adopt');
  const navigate = useNavigate();
  const client = useQueryClient();
  const capabilities = useCapabilities();
  const [name, setName] = useState(adopting ?? '');
  const [compose, setCompose] = useState(adopting ? '' : STARTER);
  const [env, setEnv] = useState('');
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  const [job, setJob] = useState<ComposeJob | null>(null);

  const save = useMutation({
    meta: { silent: true },
    mutationFn: () => commands.stacksCreate(hostId, name, compose, env),
    onSuccess: async (stack, deploy: boolean) => {
      await client.invalidateQueries({ queryKey: keys.stacks(hostId) });
      if (deploy) setJob({ stackId: stack.id, name: stack.name, action: 'up' });
      else navigate(`/stacks/${encodeURIComponent(stack.name)}`);
    },
  });
  const canDeploy = !!capabilities.data?.compose && check !== null && check.errors.length === 0;

  return (
    <div className="grid gap-4">
      <PageHeader
        title={adopting ? `Adopt ${adopting}` : 'New stack'}
        description={
          adopting
            ? 'Paste the compose file this Stack was deployed with. Homeport then manages it: you can edit and redeploy it from here.'
            : 'A compose definition Homeport keeps and deploys to this host.'
        }
      />
      {adopting && (
        <Alert>
          <AlertTitle>Use the same definition</AlertTitle>
          <AlertDescription>
            Redeploying with a different file recreates the containers to match it, and removes
            services that are no longer listed.
          </AlertDescription>
        </Alert>
      )}
      <div className="grid max-w-sm gap-1.5">
        <Label htmlFor="stack-name">Name</Label>
        <Input
          id="stack-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          disabled={!!adopting}
          placeholder="my-app"
        />
      </div>
      <StackEditor
        compose={compose}
        env={env}
        onComposeChange={setCompose}
        onEnvChange={setEnv}
        keychain={capabilities.data?.keychain ?? true}
        onCheck={setCheck}
      />
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex gap-2">
        <Button disabled={save.isPending || !canDeploy || !name} onClick={() => save.mutate(true)}>
          Save and deploy
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !name}
          onClick={() => save.mutate(false)}
        >
          Save
        </Button>
        <Button variant="ghost" onClick={() => navigate(-1)}>
          Cancel
        </Button>
        {!capabilities.data?.compose && (
          <span className="text-muted-foreground self-center text-sm">
            Install the docker compose CLI to deploy.
          </span>
        )}
      </div>
      <ComposeRunDialog
        hostId={hostId}
        job={job}
        onClose={() => {
          setJob(null);
          navigate(`/stacks/${encodeURIComponent(name)}`);
        }}
      />
    </div>
  );
}

export function NewStackPage() {
  return <HostGate>{(hostId) => <NewStack key={hostId} hostId={hostId} />}</HostGate>;
}
