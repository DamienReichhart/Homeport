import { ArrowLeftIcon, ArrowRightIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useEffect, useState } from 'react';
import type { ComposeCheck, ComposeFile } from '@/shared/api';
import { Button, Input, Label, Tabs, TabsList, TabsTrigger, Textarea } from '@/shared/ui';
import { useComposeCheck } from '../api/useComposeCheck';
import { addFile, contents, move, removeFile, updateFile } from '../model/composeFiles';
import { ComposeChecks } from './ComposeChecks';
import { YamlEditor } from './YamlEditor';
import { t } from '@/shared/i18n';

/** An example environment, written as a .env file. */
const ENV_EXAMPLE = 'DB_PASSWORD=…\nTAG=1.4';

/**
 * The compose files and environment of a Managed Stack, with live checks.
 * Files merge in tab order, like `docker compose -f a.yaml -f b.yaml`; the
 * first is the base and can't be removed.
 */
export function StackEditor({
  files,
  env,
  onFilesChange,
  onEnvChange,
  keychain,
  onCheck,
  swarm = false,
}: {
  files: ComposeFile[];
  /** Checked against a Swarm's rules too. */
  swarm?: boolean;
  env: string;
  onFilesChange: (files: ComposeFile[]) => void;
  onEnvChange: (value: string) => void;
  keychain: boolean;
  onCheck?: (check: ComposeCheck | null) => void;
}) {
  const check = useComposeCheck(contents(files), 'managed', swarm);
  useEffect(() => onCheck?.(check), [check, onCheck]);
  const [active, setActive] = useState(0);
  const index = Math.min(active, files.length - 1);
  const file = files[index];

  return (
    <div className="grid gap-4 lg:grid-cols-[1fr_20rem]">
      <div className="grid gap-1.5">
        <div className="flex flex-wrap items-center gap-2">
          <Tabs value={String(index)} onValueChange={(v) => setActive(Number(v))}>
            <TabsList aria-label={t('stacks.editor.files')}>
              {files.map((f, i) => (
                <TabsTrigger key={i} value={String(i)}>
                  {f.name}
                </TabsTrigger>
              ))}
            </TabsList>
          </Tabs>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => {
              onFilesChange(addFile(files));
              setActive(files.length);
            }}
          >
            <PlusIcon /> {t('stacks.editor.addFile')}
          </Button>
        </div>
        {file && (
          <>
            {files.length > 1 && (
              <div className="flex flex-wrap items-end gap-2">
                <div className="grid gap-1">
                  <Label htmlFor="compose-file-name" className="text-xs">
                    {t('stacks.editor.fileName')}
                  </Label>
                  <Input
                    id="compose-file-name"
                    className="h-8 w-64 font-mono text-xs"
                    value={file.name}
                    onChange={(e) =>
                      onFilesChange(updateFile(files, index, { name: e.target.value }))
                    }
                  />
                </div>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('stacks.editor.moveEarlier')}
                  disabled={index === 0}
                  onClick={() => {
                    onFilesChange(move(files, index, -1));
                    setActive(index - 1);
                  }}
                >
                  <ArrowLeftIcon />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('stacks.editor.moveLater')}
                  disabled={index === files.length - 1}
                  onClick={() => {
                    onFilesChange(move(files, index, 1));
                    setActive(index + 1);
                  }}
                >
                  <ArrowRightIcon />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('stacks.editor.removeFile', { name: file.name })}
                  disabled={index === 0}
                  title={index === 0 ? t('stacks.editor.baseFile') : undefined}
                  onClick={() => {
                    onFilesChange(removeFile(files, index));
                    setActive(index - 1);
                  }}
                >
                  <Trash2Icon />
                </Button>
              </div>
            )}
            <YamlEditor
              key={index}
              value={file.content}
              onChange={(content) => onFilesChange(updateFile(files, index, { content }))}
              label={t('stacks.editor.compose')}
            />
          </>
        )}
      </div>
      <div className="grid content-start gap-4">
        <div className="grid gap-1.5">
          <Label htmlFor="stack-env">{t('stacks.editor.environment')}</Label>
          <Textarea
            id="stack-env"
            value={env}
            onChange={(e) => onEnvChange(e.target.value)}
            disabled={!keychain}
            placeholder={keychain ? ENV_EXAMPLE : t('stacks.editor.envNoKeychain')}
            className="h-40 font-mono text-xs"
            spellCheck={false}
          />
          <p className="text-muted-foreground text-xs">
            {keychain
              ? t('stacks.editor.envHint', { variable: '${VAR}' })
              : t('stacks.editor.envHintNoKeychain')}
          </p>
        </div>
        <ComposeChecks check={check} />
      </div>
    </div>
  );
}
