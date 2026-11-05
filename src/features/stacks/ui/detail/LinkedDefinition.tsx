import { useQuery } from '@tanstack/react-query';
import { type ReactNode, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { toAppError, type LinkedFiles, type Stack } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Spinner,
  Tabs,
  TabsList,
  TabsTrigger,
  useConfirm,
} from '@/shared/ui';
import { useForgetLinked, useReloadLinked, useWriteLinked } from '../../api/mutations';
import { stackQueries } from '../../api/queries';
import { useComposeCheck } from '../../api/useComposeCheck';
import { changedOutside, dirtyFiles, fileName, textOf, type Drafts } from '../../model/linkedEdit';
import { ComposeChecks } from '../ComposeChecks';
import { YamlEditor } from '../YamlEditor';
import { t, tRich } from '@/shared/i18n';

const mono = (chunks: ReactNode[]) => <span className="font-mono">{chunks}</span>;

/**
 * A Linked Stack's compose files, edited where they are in its folder, or why
 * they can't be: the folder is gone or a file can't be read.
 */
export function LinkedDefinition({
  hostId,
  stack,
  linkedId,
}: {
  hostId: string;
  stack: Stack;
  linkedId: string;
}) {
  const missing = stack.problem === 'filesMissing';
  const files = useQuery({ ...stackQueries.linked(linkedId), enabled: !missing });
  // Bumped to start editing again from what's on disk.
  const [version, setVersion] = useState(0);
  const forget = <ForgetButton hostId={hostId} stack={stack} linkedId={linkedId} />;

  if (missing)
    return (
      <Card>
        <CardHeader>
          <CardTitle>{t('stacks.linked.missingTitle')}</CardTitle>
          <CardDescription>
            {tRich('stacks.linked.missingDescription', { folder: stack.workingDir ?? '', mono })}
          </CardDescription>
        </CardHeader>
        <CardContent>{forget}</CardContent>
      </Card>
    );
  if (files.error)
    return (
      <div className="grid justify-items-start gap-3">
        <ErrorMessage error={files.error} onRetry={() => void files.refetch()} />
        {forget}
      </div>
    );
  if (!files.data) return <Spinner />;
  return (
    <LinkedEditor
      key={version}
      linkedId={linkedId}
      opened={files.data}
      current={files.data}
      onRestart={() => setVersion((v) => v + 1)}
      forget={forget}
    />
  );
}

/** Edits the files as they were when it opened, `opened`, against what's on disk now. */
function LinkedEditor({
  linkedId,
  opened,
  current,
  onRestart,
  forget,
}: {
  linkedId: string;
  opened: LinkedFiles;
  current: LinkedFiles;
  onRestart: () => void;
  forget: ReactNode;
}) {
  const [base] = useState(opened);
  const [drafts, setDrafts] = useState<Drafts>({});
  const [active, setActive] = useState(base.files[0]?.path ?? '');
  const check = useComposeCheck(
    base.files.map((f) => textOf(f, drafts)),
    'linked',
  );
  const reload = useReloadLinked(linkedId, onRestart);
  const write = useWriteLinked(linkedId, onRestart);
  const dirty = dirtyFiles(base, drafts);
  const outside = changedOutside(base, current);
  const conflict = write.error && toAppError(write.error).code === 'stack_files_changed';
  const save = (overwrite: boolean) =>
    write.mutate({
      files: dirty.map((f) => ({
        path: f.path,
        content: textOf(f, drafts),
        expected: overwrite ? null : f.fingerprint,
      })),
      deploy: false,
    });
  const file = base.files.find((f) => f.path === active) ?? base.files[0];

  return (
    <div className="grid gap-4">
      <p className="text-muted-foreground text-sm">
        {tRich('stacks.linked.description', { folder: base.workingDir, mono })}
      </p>
      {outside && (
        <Alert>
          <AlertTitle>{t('stacks.linked.changedTitle')}</AlertTitle>
          <AlertDescription>
            {t('stacks.linked.changedDescription')}
            <Button
              size="sm"
              variant="outline"
              className="mt-2"
              disabled={reload.isPending}
              onClick={() => reload.mutate()}
            >
              {t('stacks.linked.reload')}
            </Button>
          </AlertDescription>
        </Alert>
      )}
      {reload.error && <ErrorMessage error={reload.error} />}
      <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
        <div className="grid gap-1.5">
          <Tabs value={file?.path ?? ''} onValueChange={setActive}>
            <TabsList aria-label={t('stacks.linked.files')}>
              {base.files.map((f) => (
                <TabsTrigger key={f.path} value={f.path} title={f.path}>
                  {fileName(f.path)}
                  {dirty.includes(f) && ' •'}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
          {file && (
            <YamlEditor
              key={file.path}
              value={textOf(file, drafts)}
              onChange={(text) => setDrafts((d) => ({ ...d, [file.path]: text }))}
              label={fileName(file.path)}
            />
          )}
        </div>
        <div className="grid content-start gap-4">
          <p className="text-muted-foreground text-xs">{t('stacks.linked.envNote')}</p>
          <ComposeChecks check={check} />
        </div>
      </div>
      {write.error && <ErrorMessage error={write.error} />}
      <div className="flex flex-wrap gap-2">
        <Button disabled={write.isPending || dirty.length === 0} onClick={() => save(false)}>
          {t('common.save')}
        </Button>
        {conflict && (
          <Button variant="destructive" disabled={write.isPending} onClick={() => save(true)}>
            {t('stacks.linked.overwrite')}
          </Button>
        )}
        <span className="ml-auto">{forget}</span>
      </div>
    </div>
  );
}

function ForgetButton({
  hostId,
  stack,
  linkedId,
}: {
  hostId: string;
  stack: Stack;
  linkedId: string;
}) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const forget = useForgetLinked(hostId, linkedId);
  return (
    <>
      <Button
        variant="ghost"
        onClick={() =>
          confirm({
            title: t('stacks.linked.forgetTitle', { name: stack.name }),
            description: t('stacks.linked.forgetDescription'),
            confirmLabel: t('stacks.linked.forget'),
            onConfirm: async () => {
              await forget.mutateAsync();
              navigate('/stacks');
            },
          })
        }
      >
        {t('stacks.linked.forget')}
      </Button>
      {dialog}
    </>
  );
}
