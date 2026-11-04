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
            {editing ? `Edit the login for ${registry.address}` : 'Add a registry login'}
          </DialogTitle>
          <DialogDescription>
            Used to pull, push and build on every Host. The password or token is encrypted on this
            computer and never shown again.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (!missing) save.mutate(toDraft(form));
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="registry-address">Registry</Label>
            <Input
              id="registry-address"
              value={form.address}
              onChange={(e) => set({ address: e.target.value })}
              placeholder="docker.io, ghcr.io, registry.example.com:5000"
              autoFocus
              spellCheck={false}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="registry-username">User name</Label>
            <Input
              id="registry-username"
              value={form.username}
              onChange={(e) => set({ username: e.target.value })}
              autoComplete="off"
              spellCheck={false}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="registry-secret">Password or access token</Label>
            <Input
              id="registry-secret"
              type="password"
              value={form.secret}
              onChange={(e) => set({ secret: e.target.value })}
              placeholder={editing ? 'Leave empty to keep the stored one' : undefined}
              autoComplete="new-password"
            />
          </div>
          {check.isSuccess && (
            <p className="text-success inline-flex items-center gap-1.5 text-sm">
              <CircleCheckIcon className="size-4" /> {form.address.trim()} accepted this login.
            </p>
          )}
          {check.error && <ErrorMessage error={check.error} />}
          {save.error && <ErrorMessage error={save.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              Cancel
            </Button>
            <Button
              type="button"
              variant="outline"
              disabled={missing !== null || check.isPending}
              onClick={() => check.mutate(toDraft(form))}
            >
              {check.isPending ? 'Checking…' : 'Check login'}
            </Button>
            <Button type="submit" disabled={missing !== null || save.isPending}>
              Save
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
