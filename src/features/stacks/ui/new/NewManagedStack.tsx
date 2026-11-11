import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { type ComposeCheck, type ComposeFile, useCapabilities } from '@/shared/api';
import { Button, ErrorMessage, Input, Label } from '@/shared/ui';
import { useCreateStack } from '../../api/mutations';
import type { ComposeJob } from '../../model/composeJob';
import { TEMPLATES, type StackTemplate } from '../../model/templates';
import { ComposeRunDialog } from '../dialogs/ComposeRunDialog';
import { StackEditor } from '../StackEditor';
import { TemplateGallery } from './TemplateGallery';
import { stackPath } from '../../api/owner';
import { deployTool } from '../../model/deployTool';
import type { StackOwner } from '@/shared/api';
import { t } from '@/shared/i18n';

/**
 * Creates a Managed Stack from a template picked first, or adopts the
 * External Stack named `adopting` from an empty editor.
 */
export function NewManagedStack({
  owner,
  adopting,
}: {
  owner: StackOwner;
  adopting: string | null;
}) {
  const [template, setTemplate] = useState<StackTemplate | null>(null);
  if (adopting) return <ManagedForm key="adopt" owner={owner} adopting={adopting} />;
  if (!template) return <TemplateGallery templates={TEMPLATES} onPick={setTemplate} />;
  return (
    <div className="grid gap-4">
      <Button variant="link" className="justify-self-start px-0" onClick={() => setTemplate(null)}>
        {t('stacks.newPage.otherTemplate')}
      </Button>
      <ManagedForm key={template.id} owner={owner} adopting={null} template={template} />
    </div>
  );
}

function ManagedForm({
  owner,
  adopting,
  template,
}: {
  owner: StackOwner;
  adopting: string | null;
  template?: StackTemplate;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [name, setName] = useState(adopting ?? template?.name ?? '');
  const [files, setFiles] = useState<ComposeFile[]>([
    { name: 'compose.yaml', content: template?.compose ?? '' },
  ]);
  const [env, setEnv] = useState(template?.env ?? '');
  const [check, setCheck] = useState<ComposeCheck | null>(null);
  const [job, setJob] = useState<ComposeJob | null>(null);

  const save = useCreateStack(owner, (stack, deploy) => {
    if (deploy) setJob({ kind: 'managed', stackId: stack.id, name: stack.name, action: 'up' });
    else navigate(stackPath(owner, stack.name));
  });
  const tool = deployTool(capabilities.data, owner);
  const canDeploy = tool?.state === 'available' && check !== null && check.errors.length === 0;

  return (
    <div className="grid gap-4">
      <div className="grid max-w-sm gap-1.5">
        <Label htmlFor="stack-name">{t('stacks.newPage.name')}</Label>
        <Input
          id="stack-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          disabled={!!adopting}
          placeholder="my-app"
        />
      </div>
      <StackEditor
        files={files}
        env={env}
        onFilesChange={setFiles}
        onEnvChange={setEnv}
        keychain={capabilities.data?.keychain ?? true}
        onCheck={setCheck}
        swarm={owner.kind === 'swarm'}
      />
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex gap-2">
        <Button
          disabled={save.isPending || !canDeploy || !name}
          onClick={() => save.mutate({ name, files, env, deploy: true, adopt: !!adopting })}
        >
          {t('stacks.newPage.saveAndDeploy')}
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !name}
          onClick={() => save.mutate({ name, files, env, deploy: false, adopt: !!adopting })}
        >
          {t('common.save')}
        </Button>
        <Button variant="ghost" onClick={() => navigate(-1)}>
          {t('common.cancel')}
        </Button>
      </div>
      {tool?.state === 'unavailable' && <ErrorMessage error={tool.problem} />}
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
