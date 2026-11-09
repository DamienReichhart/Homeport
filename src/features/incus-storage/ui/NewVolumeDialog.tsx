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
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { useVolumes } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Makes a custom volume in a pool: a filesystem or a block device, of a size or the pool's default. */
export function NewVolumeDialog({
  hostId,
  project,
  pool,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  pool: string;
  open: boolean;
  onClose: () => void;
}) {
  const { create } = useVolumes(hostId, project, pool);
  const [name, setName] = useState('');
  const [size, setSize] = useState('');
  const [contentType, setContentType] = useState('filesystem');
  const close = () => {
    setName('');
    setSize('');
    setContentType('filesystem');
    create.reset();
    onClose();
  };
  return (
    <Dialog open={open} onOpenChange={(next) => !next && close()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('incus.storage.new.title', { pool })}</DialogTitle>
          <DialogDescription>{t('incus.storage.new.description', { project })}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          <div className="grid gap-1.5">
            <Label htmlFor="volume-name">{t('incus.storage.new.name')}</Label>
            <Input id="volume-name" value={name} onChange={(e) => setName(e.target.value)} />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="volume-size">{t('incus.storage.new.size')}</Label>
            <Input
              id="volume-size"
              value={size}
              onChange={(e) => setSize(e.target.value)}
              placeholder="10GiB"
            />
          </div>
          <ToggleGroup
            type="single"
            variant="outline"
            size="sm"
            value={contentType}
            onValueChange={(v) => v && setContentType(v)}
            className="justify-self-start"
            aria-label={t('incus.storage.new.content')}
          >
            <ToggleGroupItem value="filesystem">
              {t('incus.storage.content.filesystem')}
            </ToggleGroupItem>
            <ToggleGroupItem value="block">{t('incus.storage.content.block')}</ToggleGroupItem>
          </ToggleGroup>
          {create.error && <ErrorMessage error={create.error} />}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={close}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={!name.trim() || create.isPending}
            onClick={() =>
              create
                .mutateAsync({ name: name.trim(), size: size.trim() || null, contentType })
                .then(close)
                .catch(() => {})
            }
          >
            {t('common.create')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
