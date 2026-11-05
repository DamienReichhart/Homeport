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
import { t } from '@/shared/i18n';

/**
 * Creates a Managed Stack from a template picked first, or adopts the
 * External Stack named `adopting` from an empty editor.
 */
export function NewManagedStack({ hostId, adopting }: { hostId: string; adopting: string | null }) {
  const [template, setTemplate] = useState<StackTemplate | null>(null);
  if (adopting) return <ManagedForm key="adopt" hostId={hostId} adopting={adopting} />;
  if (!template) return <TemplateGallery templates={TEMPLATES} onPick={setTemplate} />;
  return (
    <div className="grid gap-4">
      <Button variant="link" className="justify-self-start px-0" onClick={() => setTemplate(null)}>
        {t('stacks.newPage.otherTemplate')}
      </Button>
      <ManagedForm key={template.id} hostId={hostId} adopting={null} template={template} />
    </div>
  );
}

function ManagedForm({
  hostId,
  adopting,
  template,
}: {
  hostId: string;
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

  const save = useCreateStack(hostId, (stack, deploy) => {
    if (deploy) setJob({ kind: 'managed', stackId: stack.id, name: stack.name, action: 'up' });
    else navigate(`/stacks/${encodeURIComponent(stack.name)}`);
  });
  const canDeploy =
    capabilities.data?.compose.state === 'available' && check !== null && check.errors.length === 0;

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
      />
      {save.error && <ErrorMessage error={save.error} />}
      <div className="flex gap-2">
        <Button
          disabled={save.isPending || !canDeploy || !name}
          onClick={() => save.mutate({ name, files, env, deploy: true })}
        >
          {t('stacks.newPage.saveAndDeploy')}
        </Button>
        <Button
          variant="outline"
          disabled={save.isPending || !name}
          onClick={() => save.mutate({ name, files, env, deploy: false })}
        >
          {t('common.save')}
        </Button>
        <Button variant="ghost" onClick={() => navigate(-1)}>
          {t('common.cancel')}
        </Button>
      </div>
      {capabilities.data?.compose.state === 'unavailable' && (
        <ErrorMessage error={capabilities.data.compose.problem} />
      )}
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
