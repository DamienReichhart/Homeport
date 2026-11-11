import { FileUpIcon, PaperclipIcon, XIcon } from 'lucide-react';
import { useState } from 'react';
import { toAppError, type QuadletUnit } from '@/shared/api';
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  toastError,
  UnitFileEditor,
} from '@/shared/ui';
import { openExtraFiles, openUnitFile } from '../../api/files';
import { useInstallUnit } from '../../api/mutations';
import {
  canSaveUnit,
  emptyUnitForm,
  neededSection,
  template,
  toInstall,
  UNIT_KINDS,
  unitFormErrors,
  type UnitForm,
} from '../../model/units';
import { t } from '@/shared/i18n';

/**
 * **New unit** and **Edit**: a unit file, typed or opened from this
 * computer, with files to install beside it. Editing keeps the name and
 * installs the file again in place, so the files beside it stay.
 */
export function UnitDialog({
  hostId,
  open,
  onOpenChange,
  editing,
  initialContent = '',
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  /** The installed unit to edit; a new one otherwise. */
  editing?: QuadletUnit;
  initialContent?: string;
}) {
  const fresh = (): UnitForm =>
    editing ? { name: editing.name ?? '', content: initialContent, extra: [] } : emptyUnitForm();
  const [form, setForm] = useState<UnitForm>(fresh);
  const install = useInstallUnit(hostId, () => {
    onOpenChange(false);
    setForm(fresh());
  });
  const errors = unitFormErrors(form);
  // The file changed on the Host since the editor opened it.
  const changed = install.error && toAppError(install.error).key === 'errors.quadlets.changed';
  const save = (overwrite: boolean) =>
    install.mutate({
      install: toInstall(form, editing?.app ?? null),
      replace: editing !== undefined,
      expected: editing && !overwrite ? initialContent : null,
    });
  const extension = form.name.split('.').pop() ?? '';
  const changeKind = (next: string) => {
    const stem = form.name.includes('.')
      ? form.name.slice(0, form.name.lastIndexOf('.'))
      : form.name;
    setForm({ ...form, name: `${stem || 'web'}.${next}`, content: template(next) });
  };
  const addExtra = async () => {
    try {
      const files = await openExtraFiles();
      if (files.length) setForm((f) => ({ ...f, extra: [...f.extra, ...files] }));
    } catch (e) {
      toastError(e);
    }
  };
  const openUnit = async () => {
    try {
      const file = await openUnitFile();
      if (file) setForm((f) => ({ ...f, name: file.name, content: file.content }));
    } catch (e) {
      toastError(e);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>
            {editing
              ? t('quadlets.form.editTitle', { name: editing.name ?? '' })
              : t('quadlets.form.newTitle')}
          </DialogTitle>
          <DialogDescription>
            {editing ? t('quadlets.form.editDescription') : t('quadlets.form.newDescription')}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (canSaveUnit(form)) save(false);
          }}
        >
          <div className="grid grid-cols-[1fr_auto] items-end gap-3">
            <div className="grid gap-1.5">
              <Label htmlFor="unit-name">{t('quadlets.form.name')}</Label>
              <Input
                id="unit-name"
                className="font-mono"
                value={form.name}
                disabled={editing !== undefined}
                aria-invalid={errors.name ? true : undefined}
                onChange={(e) => setForm({ ...form, name: e.target.value })}
              />
            </div>
            {!editing && (
              <div className="flex gap-2">
                <Select value={extension} onValueChange={changeKind}>
                  <SelectTrigger className="w-36" aria-label={t('quadlets.form.kind')}>
                    <SelectValue placeholder={t('quadlets.form.kind')} />
                  </SelectTrigger>
                  <SelectContent>
                    {UNIT_KINDS.map((k) => (
                      <SelectItem key={k.extension} value={k.extension}>
                        .{k.extension}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button type="button" variant="outline" onClick={() => void openUnit()}>
                  <FileUpIcon /> {t('quadlets.form.openFile')}
                </Button>
              </div>
            )}
          </div>
          {errors.name && <p className="text-destructive text-xs">{t(errors.name)}</p>}

          <div className="grid gap-1.5">
            <Label>{t('quadlets.form.content')}</Label>
            <UnitFileEditor
              label={t('quadlets.form.content')}
              value={form.content}
              onChange={(content) => setForm((f) => ({ ...f, content }))}
            />
            {errors.content && (
              <p className="text-destructive text-xs">
                {t(errors.content, { section: neededSection(form.name) })}
              </p>
            )}
          </div>

          {!editing && (
            <div className="grid gap-1.5">
              <Label>{t('quadlets.form.extra')}</Label>
              <p className="text-muted-foreground text-xs">{t('quadlets.form.extraHint')}</p>
              {form.extra.map((file, i) => (
                <div key={`${file.name}-${i}`} className="flex items-center gap-2 text-sm">
                  <PaperclipIcon className="text-muted-foreground size-4" />
                  <span className="font-mono">{file.name}</span>
                  <Button
                    type="button"
                    variant="ghost"
                    size="icon-sm"
                    aria-label={t('quadlets.form.removeExtra', { name: file.name })}
                    onClick={() =>
                      setForm((f) => ({ ...f, extra: f.extra.filter((_, j) => j !== i) }))
                    }
                  >
                    <XIcon />
                  </Button>
                </div>
              ))}
              <Button
                type="button"
                variant="outline"
                size="sm"
                className="justify-self-start"
                onClick={() => void addExtra()}
              >
                <PaperclipIcon /> {t('quadlets.form.addExtra')}
              </Button>
              {errors.extra && <p className="text-destructive text-xs">{t(errors.extra)}</p>}
            </div>
          )}

          {install.error && <ErrorMessage error={install.error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              {t('common.cancel')}
            </Button>
            {changed && (
              <Button
                type="button"
                variant="destructive"
                disabled={!canSaveUnit(form) || install.isPending}
                onClick={() => save(true)}
              >
                {t('quadlets.form.overwrite')}
              </Button>
            )}
            <Button type="submit" disabled={!canSaveUnit(form) || install.isPending}>
              {editing ? t('quadlets.form.save') : t('quadlets.form.install')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
