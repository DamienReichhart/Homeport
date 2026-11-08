import { useQuery } from '@tanstack/react-query';
import { PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { repositoryLabel, repositoryQueries, RepositoryDialog } from '@/features/repositories';
import { useCapabilities, type RefKind } from '@/shared/api';
import {
  Button,
  ErrorMessage,
  HintView,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Textarea,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { useCreateSynced } from '../../api/synced';
import type { ComposeJob } from '../../model/composeJob';
import { nameFromRepository, refChoices } from '../../model/newSynced';
import { ComposeRunDialog } from '../dialogs/ComposeRunDialog';
import { stackPath } from '../../api/owner';
import { deployTool } from '../../model/deployTool';
import type { StackOwner } from '@/shared/api';
import { t } from '@/shared/i18n';

/** Creates a Synced Stack from a git repository's branch or tag. */
export function NewSyncedStack({ owner }: { owner: StackOwner }) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const repositories = useQuery(repositoryQueries.list());
  const [repositoryId, setRepositoryId] = useState<string | null>(null);
  const [adding, setAdding] = useState(false);
  const [refKind, setRefKind] = useState<RefKind>('branch');
  const [refName, setRefName] = useState('');
  const [composePath, setComposePath] = useState('compose.yaml');
  const [name, setName] = useState('');
  const [env, setEnv] = useState('');
  const [job, setJob] = useState<ComposeJob | null>(null);
  const refs = useQuery(repositoryQueries.refs(repositoryId));
  const choices = refChoices(refs.data, refKind);
  const chosenRef = refName || choices[0] || '';
  const save = useCreateSynced(owner, (stack, deploy) => {
    if (deploy) setJob({ kind: 'synced', stackId: stack.id, name: stack.name, action: 'up' });
    else navigate(stackPath(owner, stack.name));
  });
  const git = capabilities.data?.git;
  const choose = (id: string) => {
    setRepositoryId(id);
    setRefName('');
    const chosen = repositories.data?.find((r) => r.id === id);
    if (chosen && !name) setName(nameFromRepository(repositoryLabel(chosen)));
  };
  const create = (deploy: boolean) =>
    repositoryId &&
    save.mutate({
      draft: { name, repositoryId, refKind, refName: chosenRef, composePath },
      env,
      deploy,
    });
  const ready = repositoryId !== null && chosenRef !== '' && composePath.trim() !== '' && name;
  const canDeploy = deployTool(capabilities.data, owner)?.state === 'available';

  if (git?.state === 'unavailable')
    return (
      <div className="grid gap-2">
        <p className="text-sm">{git.problem.message}</p>
        {git.problem.hint && <HintView hint={git.problem.hint} />}
      </div>
    );

  return (
    <div className="grid max-w-xl gap-4">
      <p className="text-muted-foreground text-sm">{t('stacks.synced.whileOpen')}</p>
      <div className="grid gap-1.5">
        <Label htmlFor="synced-repository">{t('stacks.newSynced.repository')}</Label>
        <div className="flex gap-2">
          <Select value={repositoryId ?? ''} onValueChange={choose}>
            <SelectTrigger id="synced-repository" className="min-w-0 flex-1">
              <SelectValue placeholder={t('stacks.newSynced.pickRepository')} />
            </SelectTrigger>
            <SelectContent>
              {repositories.data?.map((r) => (
                <SelectItem key={r.id} value={r.id}>
                  {repositoryLabel(r)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Button variant="outline" onClick={() => setAdding(true)}>
            <PlusIcon /> {t('stacks.newSynced.addRepository')}
          </Button>
        </div>
      </div>
      {repositoryId && (
        <div className="grid gap-1.5">
          <Label htmlFor="synced-ref">{t('stacks.newSynced.ref')}</Label>
          <div className="flex flex-wrap gap-2">
            <ToggleGroup
              type="single"
              variant="outline"
              value={refKind}
              onValueChange={(k) => {
                if (!k) return;
                setRefKind(k as RefKind);
                setRefName('');
              }}
              aria-label={t('stacks.newSynced.refKind')}
            >
              <ToggleGroupItem value="branch">{t('stacks.newSynced.branch')}</ToggleGroupItem>
              <ToggleGroupItem value="tag">{t('stacks.newSynced.tag')}</ToggleGroupItem>
            </ToggleGroup>
            {refs.isPending ? (
              <Spinner />
            ) : (
              <Select value={chosenRef} onValueChange={setRefName} disabled={choices.length === 0}>
                <SelectTrigger id="synced-ref" className="min-w-48">
                  <SelectValue
                    placeholder={
                      refKind === 'tag'
                        ? t('stacks.newSynced.noTags')
                        : t('stacks.newSynced.noBranches')
                    }
                  />
                </SelectTrigger>
                <SelectContent>
                  {choices.map((r) => (
                    <SelectItem key={r} value={r}>
                      {r}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          </div>
          {refs.error && <ErrorMessage error={refs.error} onRetry={() => void refs.refetch()} />}
        </div>
      )}
      <div className="grid gap-1.5">
        <Label htmlFor="synced-path">{t('stacks.newSynced.path')}</Label>
        <Input
          id="synced-path"
          className="font-mono"
          value={composePath}
          onChange={(e) => setComposePath(e.target.value)}
          spellCheck={false}
        />
        <p className="text-muted-foreground text-xs">{t('stacks.newSynced.pathHelp')}</p>
      </div>
      <div className="grid max-w-sm gap-1.5">
        <Label htmlFor="synced-name">{t('stacks.newPage.name')}</Label>
        <Input id="synced-name" value={name} onChange={(e) => setName(e.target.value)} />
      </div>
      <div className="grid gap-1.5">
        <Label htmlFor="synced-env">{t('stacks.newSynced.env')}</Label>
        <Textarea
          id="synced-env"
          className="font-mono text-xs"
          rows={4}
          value={env}
          onChange={(e) => setEnv(e.target.value)}
          disabled={!(capabilities.data?.keychain ?? true)}
          spellCheck={false}
        />
        <p className="text-muted-foreground text-xs">{t('stacks.newSynced.envHelp')}</p>
      </div>
      {save.isPending && (
        <p className="text-muted-foreground text-sm">{t('stacks.newSynced.fetching')}</p>
      )}
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex gap-2">
        <Button disabled={save.isPending || !ready || !canDeploy} onClick={() => create(true)}>
          {t('stacks.newPage.saveAndDeploy')}
        </Button>
        <Button variant="outline" disabled={save.isPending || !ready} onClick={() => create(false)}>
          {t('common.save')}
        </Button>
        <Button variant="ghost" onClick={() => navigate(-1)}>
          {t('common.cancel')}
        </Button>
      </div>
      {adding && (
        <RepositoryDialog
          repositoryId={null}
          onClose={() => setAdding(false)}
          onAdded={(r) => choose(r.id)}
        />
      )}
      <ComposeRunDialog
        owner={owner}
        job={job}
        onClose={() => {
          setJob(null);
          navigate(stackPath(owner, name));
        }}
      />
    </div>
  );
}
