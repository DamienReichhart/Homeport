import { useState } from 'react';
import type { Container } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
} from '@/shared/ui';
import { useRenameContainer } from '../../api/mutations';
import { t } from '@/shared/i18n';

export function RenameDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container | null;
  onClose: () => void;
}) {
  const [name, setName] = useState(container?.name ?? '');
  const rename = useRenameContainer(hostId, container?.id ?? '', onClose);
  return (
    <Dialog open={container !== null} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('containers.rename.title', { name: container?.name })}</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            rename.mutate(name);
          }}
        >
          <Input
            value={name}
            onChange={(e) => setName(e.target.value)}
            autoFocus
            aria-label={t('containers.rename.newName')}
          />
          {rename.error && <ErrorMessage error={rename.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={rename.isPending}>
              {t('containers.action.rename')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
