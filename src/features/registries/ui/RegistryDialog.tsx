import { CircleCheckIcon } from 'lucide-react';
import { useState } from 'react';
import type { Registry } from '@/shared/api';
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
} from '@/shared/ui';
import { useCheckRegistry, useSaveRegistry } from '../api/mutations';
import { emptyLogin, loginFormOf, missingField, toDraft, type LoginForm } from '../model/login';
import { t } from '@/shared/i18n';

/** Adds a registry login, or edits one (`registry` set), with a login check. */
export function RegistryDialog({
  registry,
  onClose,
}: {
  registry: Registry | null;
  onClose: () => void;
}) {
  const editing = registry !== null;
  const [form, setForm] = useState<LoginForm>(registry ? loginFormOf(registry) : emptyLogin);
  const save = useSaveRegistry(registry?.id ?? null, onClose);
  const check = useCheckRegistry(registry?.id ?? null);
  const missing = missingField(form, editing);
  const set = (patch: Partial<LoginForm>) => {
    setForm((f) => ({ ...f, ...patch }));
    check.reset();
  };

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {editing
              ? t('registries.dialog.editTitle', { registry: registry.address })
              : t('registries.dialog.addTitle')}
          </DialogTitle>
          <DialogDescription>{t('registries.dialog.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (!missing) save.mutate(toDraft(form));
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="registry-address">{t('registries.dialog.address')}</Label>
            <Input
              id="registry-address"
              value={form.address}
              onChange={(e) => set({ address: e.target.value })}
              // eslint-disable-next-line homeport/no-literal-ui-string -- example registry addresses
              placeholder="docker.io, ghcr.io, registry.example.com:5000"
              autoFocus
              spellCheck={false}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="registry-username">{t('registries.dialog.username')}</Label>
            <Input
              id="registry-username"
              value={form.username}
              onChange={(e) => set({ username: e.target.value })}
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="registry-secret">{t('registries.dialog.secret')}</Label>
            <Input
              id="registry-secret"
              type="password"
              value={form.secret}
              onChange={(e) => set({ secret: e.target.value })}
              placeholder={editing ? t('registries.dialog.keepSecret') : undefined}
              autoComplete="new-password"
            />
          </div>
          {check.isSuccess && (
            <p className="text-success inline-flex items-center gap-1.5 text-sm">
              <CircleCheckIcon className="size-4" />{' '}
              {t('registries.dialog.accepted', { registry: form.address.trim() })}
            </p>
          )}
          {check.error && <ErrorMessage error={check.error} />}
          {save.error && <ErrorMessage error={save.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button
              type="button"
              variant="outline"
              disabled={missing !== null || check.isPending}
              onClick={() => check.mutate(toDraft(form))}
            >
              {check.isPending ? t('registries.dialog.checking') : t('registries.dialog.check')}
            </Button>
            <Button type="submit" disabled={missing !== null || save.isPending}>
              {t('common.save')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
