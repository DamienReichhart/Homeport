import { ArrowDownIcon, ArrowUpIcon, PlusIcon, XIcon } from 'lucide-react';
import type { LinkedFiles } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { Button, ErrorMessage, Label } from '@/shared/ui';
import { useSetLinkedFiles } from '../../api/mutations';
import { move } from '../../model/composeFiles';
import { fileName } from '../../model/linkedEdit';
import { t } from '@/shared/i18n';

const YAML = [{ name: 'Compose', extensions: ['yaml', 'yml'] }];

/**
 * Which compose files of its folder a Linked Stack uses, in the order
 * compose merges them: add one from the folder, remove or reorder them.
 * `locked` while edits are unsaved, since changing files reloads the editor.
 */
export function LinkedFileList({
  linkedId,
  files,
  locked,
  onChanged,
}: {
  linkedId: string;
  files: LinkedFiles;
  locked: boolean;
  onChanged: () => void;
}) {
  const set = useSetLinkedFiles(linkedId, onChanged);
  const paths = files.files.map((f) => f.path);
  const disabled = locked || set.isPending;
  const add = async () => {
    const picked = await pickFile(t('stacks.linked.pickFile'), YAML, files.workingDir);
    if (picked && !paths.includes(picked)) set.mutate([...paths, picked]);
  };

  return (
    <div className="grid gap-1.5">
      <Label>{t('stacks.linked.order')}</Label>
      <ol className="grid gap-1">
        {paths.map((path, i) => (
          <li key={path} className="flex items-center gap-1 text-sm">
            <span className="flex-1 truncate font-mono" title={path}>
              {fileName(path)}
            </span>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('stacks.editor.moveEarlier')}
              disabled={disabled || i === 0}
              onClick={() => set.mutate(move(paths, i, -1))}
            >
              <ArrowUpIcon />
            </Button>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('stacks.editor.moveLater')}
              disabled={disabled || i === paths.length - 1}
              onClick={() => set.mutate(move(paths, i, 1))}
            >
              <ArrowDownIcon />
            </Button>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('stacks.editor.removeFile', { name: fileName(path) })}
              disabled={disabled || paths.length === 1}
              onClick={() => set.mutate(paths.filter((p) => p !== path))}
            >
              <XIcon />
            </Button>
          </li>
        ))}
      </ol>
      <Button
        variant="outline"
        size="sm"
        className="justify-self-start"
        disabled={disabled}
        onClick={() => void add()}
      >
        <PlusIcon /> {t('stacks.linked.addFile')}
      </Button>
      {locked && <p className="text-muted-foreground text-xs">{t('stacks.linked.saveFirst')}</p>}
      {set.error && <ErrorMessage error={set.error} />}
    </div>
  );
}
