import { useState } from 'react';
import type { IncusImage } from '@/shared/api';
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
import { imageName, useAliases } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Gives an image another alias, so Instances can be made from it by name. */
export function AliasDialog({
  hostId,
  project,
  image,
  onClose,
}: {
  hostId: string;
  project: string;
  image: IncusImage | null;
  onClose: () => void;
}) {
  const [name, setName] = useState('');
  const { add } = useAliases(hostId, project);
  const close = () => {
    setName('');
    add.reset();
    onClose();
  };
  return (
    <Dialog open={!!image} onOpenChange={(open) => !open && close()}>
      <DialogContent className="sm:max-w-md">
        {image && (
          <>
            <DialogHeader>
              <DialogTitle>{t('incus.images.alias.title', { name: imageName(image) })}</DialogTitle>
            </DialogHeader>
            <Input
              aria-label={t('incus.images.alias.name')}
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="debian-web"
              autoFocus
            />
            {add.error && <ErrorMessage error={add.error} />}
            <DialogFooter>
              <Button variant="outline" onClick={close}>
                {t('common.cancel')}
              </Button>
              <Button
                disabled={!name.trim() || add.isPending}
                onClick={() =>
                  add
                    .mutateAsync({ image, name })
                    .then(close)
                    .catch(() => {})
                }
              >
                {t('incus.images.alias.add')}
              </Button>
            </DialogFooter>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}
