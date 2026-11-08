import { useState } from 'react';
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
import { useCreateNamespace } from '../api/mutations';
import { namespaceNameValid } from '../model/names';
import { t } from '@/shared/i18n';

/** Creates an empty containerd namespace. */
export function NewNamespaceDialog({
  hostId,
  open,
  onOpenChange,
  onCreated,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreated: (name: string) => void;
}) {
  const [name, setName] = useState('');
  const create = useCreateNamespace(hostId, (created) => {
    setName('');
    onOpenChange(false);
    onCreated(created);
  });
  const valid = namespaceNameValid(name);
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('containerd.namespaces.newDialog.title')}</DialogTitle>
          <DialogDescription>{t('containerd.namespaces.newDialog.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (valid) create.mutate(name.trim());
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="namespace-name">{t('containerd.namespaces.newDialog.name')}</Label>
            <Input
              id="namespace-name"
              value={name}
              placeholder="lab"
              autoFocus
              aria-invalid={name && !valid ? true : undefined}
              onChange={(e) => setName(e.target.value)}
            />
            {name && !valid && (
              <p className="text-destructive text-xs">
                {t('containerd.namespaces.newDialog.invalid')}
              </p>
            )}
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!valid || create.isPending}>
              {t('containerd.namespaces.newDialog.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
