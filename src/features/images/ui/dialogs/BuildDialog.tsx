import { FolderOpenIcon } from 'lucide-react';
import { useRef, useState } from 'react';
import { toAppError, type AppError, type BuildSpec } from '@/shared/api';
import { pickFolder } from '@/shared/lib';
import {
  Button,
  Checkbox,
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
  XTerm,
} from '@/shared/ui';
import { useBuildImage } from '../../api/streams';
import { emptyBuild, toBuildSpec, type BuildForm } from '../../model/build';
import { t } from '@/shared/i18n';

/**
 * Builds an image on the Host from a folder on this machine: the form, then
 * the build's output live.
 */
export function BuildDialog({ hostId, onClose }: { hostId: string; onClose: () => void }) {
  const [form, setForm] = useState<BuildForm>(emptyBuild);
  const [running, setRunning] = useState<BuildSpec | null>(null);
  const [result, setResult] = useState<'running' | 'ok' | 'failed'>('running');
  const [failure, setFailure] = useState<AppError | null>(null);
  const stop = useRef<(() => void) | null>(null);
  const build = useBuildImage(hostId);
  const checked = toBuildSpec(form);
  const set = <K extends keyof BuildForm>(key: K, value: BuildForm[K]) =>
    setForm((f) => ({ ...f, [key]: value }));
  const close = () => {
    stop.current?.();
    onClose();
  };
  const fail = (error: unknown) => {
    setFailure(toAppError(error));
    setResult('failed');
  };

  return (
    <Dialog open onOpenChange={(open) => !open && close()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{t('images.buildDialog.title')}</DialogTitle>
          <DialogDescription>{t('images.buildDialog.description')}</DialogDescription>
        </DialogHeader>
        {running ? (
          <>
            {failure && <ErrorMessage error={failure} />}
            <div className="h-80">
              <XTerm
                readOnly
                className="h-full"
                onReady={({ terminal }) => {
                  stop.current = build(
                    running,
                    (e) => {
                      if (e.type === 'line') terminal.writeln(e.text);
                      else if (e.type === 'error') {
                        terminal.writeln(`\x1b[31m${e.error.message}\x1b[39m`);
                        fail(e.error);
                      } else {
                        terminal.writeln(`\x1b[32m${t('images.buildDialog.built')}\x1b[39m`);
                        setResult('ok');
                      }
                    },
                    fail,
                  );
                }}
              />
            </div>
          </>
        ) : (
          <form
            id="build-form"
            className="grid gap-3"
            onSubmit={(e) => {
              e.preventDefault();
              if ('spec' in checked) setRunning(checked.spec);
            }}
          >
            <div className="grid gap-1.5">
              <Label htmlFor="build-context">{t('images.buildDialog.folder')}</Label>
              <div className="flex gap-2">
                <Input
                  id="build-context"
                  value={form.context}
                  onChange={(e) => set('context', e.target.value)}
                  placeholder="/home/me/projects/app"
                  spellCheck={false}
                />
                <Button
                  type="button"
                  variant="outline"
                  onClick={async () => {
                    const folder = await pickFolder(t('images.buildDialog.chooseFolder'));
                    if (folder) set('context', folder);
                  }}
                >
                  <FolderOpenIcon /> {t('images.buildDialog.choose')}
                </Button>
              </div>
            </div>
            <div className="grid grid-cols-2 gap-3">
              <div className="grid gap-1.5">
                <Label htmlFor="build-dockerfile">{t('images.buildDialog.dockerfile')}</Label>
                <Input
                  id="build-dockerfile"
                  value={form.dockerfile}
                  onChange={(e) => set('dockerfile', e.target.value)}
                  spellCheck={false}
                />
              </div>
              <div className="grid gap-1.5">
                <Label htmlFor="build-target">{t('images.buildDialog.target')}</Label>
                <Input
                  id="build-target"
                  value={form.target}
                  onChange={(e) => set('target', e.target.value)}
                  placeholder={t('images.buildDialog.targetPlaceholder')}
                  spellCheck={false}
                />
              </div>
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="build-tags">{t('images.buildDialog.names')}</Label>
              <Input
                id="build-tags"
                value={form.tags}
                onChange={(e) => set('tags', e.target.value)}
                // eslint-disable-next-line homeport/no-literal-ui-string -- example image names
                placeholder="app:1.0, ghcr.io/me/app:1.0"
                spellCheck={false}
              />
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="build-args">{t('images.buildDialog.buildArgs')}</Label>
              <Textarea
                id="build-args"
                value={form.buildArgs}
                onChange={(e) => set('buildArgs', e.target.value)}
                placeholder={t('images.buildDialog.buildArgsPlaceholder')}
                className="h-20 font-mono text-xs"
                spellCheck={false}
              />
            </div>
            <div className="flex flex-wrap gap-4">
              <Label className="inline-flex items-center gap-2 font-normal">
                <Checkbox
                  checked={form.noCache}
                  onCheckedChange={(v) => set('noCache', v === true)}
                />
                {t('images.buildDialog.noCache')}
              </Label>
              <Label className="inline-flex items-center gap-2 font-normal">
                <Checkbox checked={form.pull} onCheckedChange={(v) => set('pull', v === true)} />
                {t('images.buildDialog.pull')}
              </Label>
            </div>
            {'problem' in checked && form.context && (
              <p className="text-muted-foreground text-xs">{checked.problem}</p>
            )}
          </form>
        )}
        <DialogFooter>
          {running ? (
            <Button variant={result === 'running' ? 'ghost' : 'default'} onClick={close}>
              {result === 'running' ? t('common.cancel') : t('common.close')}
            </Button>
          ) : (
            <>
              <Button variant="ghost" onClick={close}>
                {t('common.cancel')}
              </Button>
              <Button type="submit" form="build-form" disabled={!('spec' in checked)}>
                {t('images.build')}
              </Button>
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
