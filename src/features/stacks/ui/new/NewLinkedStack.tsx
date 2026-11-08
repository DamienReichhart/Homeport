import { useQuery } from '@tanstack/react-query';
import { FolderOpenIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCapabilities, type FolderProject } from '@/shared/api';
import { pickFolder } from '@/shared/lib';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { useCreateLinked, useInspectFolder } from '../../api/mutations';
import { useComposeCheck } from '../../api/useComposeCheck';
import type { ComposeJob } from '../../model/composeJob';
import { fileName } from '../../model/linkedEdit';
import { takenOver } from '../../model/newLinked';
import { ComposeChecks } from '../ComposeChecks';
import { ComposeRunDialog } from '../dialogs/ComposeRunDialog';
import { stackPath } from '../../api/owner';
import { stackQueries } from '../../api/queries';
import { deployTool } from '../../model/deployTool';
import type { StackOwner } from '@/shared/api';
import { t } from '@/shared/i18n';

/** Creates a Linked Stack from a folder on this computer that holds a compose project. */
export function NewLinkedStack({ owner }: { owner: StackOwner }) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const stacks = useQuery(stackQueries.of(owner));
  const [project, setProject] = useState<FolderProject | null>(null);
  const [name, setName] = useState('');
  const [job, setJob] = useState<ComposeJob | null>(null);
  const inspect = useInspectFolder((found) => {
    setProject(found);
    setName(found.name);
  });
  const check = useComposeCheck(
    project?.files.map((f) => f.content) ?? [],
    'linked',
    owner.kind === 'swarm',
  );
  const save = useCreateLinked(owner, (stack, deploy) => {
    if (deploy) setJob({ kind: 'linked', stackId: stack.id, name: stack.name, action: 'up' });
    else navigate(stackPath(owner, stack.name));
  });
  const choose = async () => {
    const folder = await pickFolder(t('stacks.newLinked.pickTitle'));
    if (folder) inspect.mutate(folder);
  };
  const create = (deploy: boolean) =>
    project &&
    save.mutate({
      name,
      workingDir: project.workingDir,
      configFiles: project.files.map((f) => f.path),
      deploy,
    });
  const canDeploy =
    deployTool(capabilities.data, owner)?.state === 'available' &&
    check !== null &&
    check.errors.length === 0;
  const external = takenOver(stacks.data, name);

  return (
    <div className="grid gap-4">
      <div className="flex flex-wrap items-center gap-3">
        <Button variant="outline" onClick={() => void choose()} disabled={inspect.isPending}>
          <FolderOpenIcon /> {t('stacks.newLinked.choose')}
        </Button>
        {project && <span className="font-mono text-sm">{project.workingDir}</span>}
      </div>
      {inspect.error && <ErrorMessage error={inspect.error} />}
      {project && (
        <>
          <div className="grid gap-1.5">
            <Label>{t('stacks.newLinked.files')}</Label>
            <ol className="grid gap-1 text-sm">
              {project.files.map((f) => (
                <li key={f.path} className="font-mono" title={f.path}>
                  {fileName(f.path)}
                </li>
              ))}
            </ol>
            <p className="text-muted-foreground text-xs">{t('stacks.linked.envNote')}</p>
          </div>
          <div className="grid max-w-sm gap-1.5">
            <Label htmlFor="linked-name">{t('stacks.newPage.name')}</Label>
            <Input id="linked-name" value={name} onChange={(e) => setName(e.target.value)} />
          </div>
          {external && (
            <Alert>
              <AlertTitle>{t('stacks.newLinked.takeOver.title', { name })}</AlertTitle>
              <AlertDescription>{t('stacks.newPage.sameDefinition.description')}</AlertDescription>
            </Alert>
          )}
          <ComposeChecks check={check} />
          {save.error && <ErrorMessage error={save.error} />}
          <div className="flex gap-2">
            <Button disabled={save.isPending || !canDeploy || !name} onClick={() => create(true)}>
              {t('stacks.newPage.saveAndDeploy')}
            </Button>
            <Button
              variant="outline"
              disabled={save.isPending || !name}
              onClick={() => create(false)}
            >
              {t('common.save')}
            </Button>
            <Button variant="ghost" onClick={() => navigate(-1)}>
              {t('common.cancel')}
            </Button>
          </div>
        </>
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
