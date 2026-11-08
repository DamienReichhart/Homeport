import { EyeIcon, EyeOffIcon, FileUpIcon, XIcon } from 'lucide-react';
import { useState } from 'react';
import type { ObjectValue } from '@/shared/api';
import { cn, pickFile } from '@/shared/lib';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Textarea,
} from '@/shared/ui';
import { useCreateObject, type ObjectKind } from '../../api/mutations';
import { t } from '@/shared/i18n';

/**
 * Creates a secret (its value masked unless shown) or a config, typed or
 * loaded from a file read by the backend. Everything typed lives in this
 * component only: closing it drops the state and resets the mutation.
 */
export function CreateObjectDialog({
  swarmId,
  kind,
  onClose,
}: {
  swarmId: string;
  kind: ObjectKind;
  onClose: () => void;
}) {
  const [name, setName] = useState('');
  const [text, setText] = useState('');
  const [file, setFile] = useState<string | null>(null);
  const [shown, setShown] = useState(kind === 'config');
  const create = useCreateObject(swarmId, kind, () => close());
  const secret = kind === 'secret';

  function close() {
    setText('');
    create.reset();
    onClose();
  }

  const value: ObjectValue = file ? { kind: 'file', path: file } : { kind: 'text', text };
  const loadFile = async () => {
    const picked = await pickFile(t('swarm.objects.create.pickFile'));
    if (picked) {
      setFile(picked);
      setText('');
    }
  };

  return (
    <Dialog open onOpenChange={(open) => !open && !create.isPending && close()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {secret ? t('swarm.objects.create.secretTitle') : t('swarm.objects.create.configTitle')}
          </DialogTitle>
          <DialogDescription>
            {secret
              ? t('swarm.objects.create.secretDescription')
              : t('swarm.objects.create.configDescription')}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            create.mutate({ name, value, labels: {} });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="object-name">{t('swarm.objects.create.name')}</Label>
            <Input
              id="object-name"
              value={name}
              autoFocus
              placeholder={
                secret
                  ? t('swarm.objects.create.secretPlaceholder')
                  : t('swarm.objects.create.configPlaceholder')
              }
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div className="grid gap-1.5">
            <div className="flex items-center justify-between">
              <Label htmlFor="object-value">
                {secret ? t('swarm.objects.create.value') : t('swarm.objects.create.content')}
              </Label>
              <div className="flex gap-1">
                {secret && !file && (
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    onClick={() => setShown((v) => !v)}
                  >
                    {shown ? <EyeOffIcon /> : <EyeIcon />}
                    {shown ? t('swarm.objects.create.hide') : t('swarm.objects.create.show')}
                  </Button>
                )}
                <Button type="button" variant="ghost" size="sm" onClick={() => void loadFile()}>
                  <FileUpIcon /> {t('swarm.objects.create.loadFile')}
                </Button>
              </div>
            </div>
            {file ? (
              <div className="flex items-center gap-2 rounded-md border px-3 py-2 text-sm">
                <span className="min-w-0 flex-1 truncate font-mono text-xs">{file}</span>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('swarm.objects.create.clearFile')}
                  onClick={() => setFile(null)}
                >
                  <XIcon />
                </Button>
              </div>
            ) : (
              <Textarea
                id="object-value"
                value={text}
                rows={secret ? 4 : 10}
                spellCheck={false}
                autoComplete="off"
                className={cn('font-mono text-xs', !shown && '[-webkit-text-security:disc]')}
                onChange={(e) => setText(e.target.value)}
              />
            )}
            {file && (
              <p className="text-muted-foreground text-xs">{t('swarm.objects.create.fileNote')}</p>
            )}
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={close}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={create.isPending || !name.trim()}>
              {t('common.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
