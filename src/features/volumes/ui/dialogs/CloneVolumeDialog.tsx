import { useEffect, useRef, useState } from 'react';
import type { VolumeView } from '@/shared/api';
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
  Spinner,
} from '@/shared/ui';
import { useCloneVolume } from '../../api/mutations';
import { copyName } from '../../model/transferFlow';
import { VolumeActionFlow } from '../VolumeActionFlow';
import { t } from '@/shared/i18n';

/** Copies `volume` into a new volume; `onCloned` gets the new volume's name. */
export function CloneVolumeDialog({
  hostId,
  volume,
  volumes,
  onCloned,
  onClose,
}: {
  hostId: string;
  volume: string;
  volumes: VolumeView[];
  onCloned: (name: string) => void;
  onClose: () => void;
}) {
  const [target, setTarget] = useState<string | null>(null);
  if (target === null)
    return <NameForm volume={volume} volumes={volumes} onCancel={onClose} onChoose={setTarget} />;
  return (
    <VolumeActionFlow
      hostId={hostId}
      volume={volume}
      mode="clone"
      title={t('volumes.clone.title', { volume })}
      onClose={onClose}
      transfer={({ pulledImage, onFinished }) => (
        <Copying
          hostId={hostId}
          source={volume}
          target={target}
          pulledImage={pulledImage}
          onDone={(name) => {
            onCloned(name);
            onFinished();
          }}
          onClose={onFinished}
        />
      )}
    />
  );
}

function NameForm({
  volume,
  volumes,
  onCancel,
  onChoose,
}: {
  volume: string;
  volumes: VolumeView[];
  onCancel: () => void;
  onChoose: (name: string) => void;
}) {
  const taken = new Set(volumes.map((v) => v.name));
  const [name, setName] = useState(() => copyName(volume, taken));
  const clash = taken.has(name.trim());
  return (
    <Dialog open onOpenChange={(open) => !open && onCancel()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('volumes.clone.title', { volume })}</DialogTitle>
          <DialogDescription>{t('volumes.clone.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (name.trim() && !clash) onChoose(name.trim());
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="clone-name">{t('volumes.clone.name')}</Label>
            <Input
              id="clone-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              autoFocus
            />
            {clash && (
              <p className="text-destructive text-sm">
                {t('volumes.restore.nameTaken', { volume: name.trim() })}
              </p>
            )}
          </div>
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onCancel}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!name.trim() || clash}>
              {t('volumes.clone.start')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

/** Runs the copy once, and shows it until it's done or failed. */
function Copying({
  hostId,
  source,
  target,
  pulledImage,
  onDone,
  onClose,
}: {
  hostId: string;
  source: string;
  target: string;
  pulledImage: boolean;
  onDone: (name: string) => void;
  onClose: () => void;
}) {
  const clone = useCloneVolume(hostId);
  const started = useRef(false);
  useEffect(() => {
    if (started.current) return;
    started.current = true;
    clone.mutate({ source, target, pulledImage }, { onSuccess: onDone });
  }, [clone, source, target, pulledImage, onDone]);
  return (
    <Dialog open onOpenChange={(open) => !open && !clone.isPending && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('volumes.clone.title', { volume: source })}</DialogTitle>
          <DialogDescription>{t('volumes.clone.into', { target })}</DialogDescription>
        </DialogHeader>
        {clone.error ? (
          <ErrorMessage error={clone.error} />
        ) : (
          <div className="flex items-center gap-2 text-sm" role="status">
            <Spinner /> {t('volumes.clone.copying')}
          </div>
        )}
        <DialogFooter>
          <Button variant="outline" disabled={clone.isPending} onClick={onClose}>
            {t('common.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
