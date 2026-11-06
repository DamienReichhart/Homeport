import { useRef, useState } from 'react';
import { toast } from 'sonner';
import type { VolumeView } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  ToggleGroup,
  ToggleGroupItem,
  TransferDialog,
} from '@/shared/ui';
import { useRestoreVolume } from '../../api/streams';
import { volumeNameFromFile } from '../../model/transferFlow';
import { VolumeActionFlow } from '../VolumeActionFlow';
import { t } from '@/shared/i18n';

type Target = { volume: string; create: boolean };

/**
 * Restores a volume from the `.tar` at `source`: into a new volume, named
 * after the file unless changed, or into an existing one, `existing` if given.
 */
export function RestoreVolumeDialog({
  hostId,
  source,
  volumes,
  existing,
  onClose,
}: {
  hostId: string;
  source: string;
  volumes: VolumeView[];
  existing?: string;
  onClose: () => void;
}) {
  const [target, setTarget] = useState<Target | null>(null);
  if (!target)
    return (
      <TargetForm
        source={source}
        volumes={volumes}
        existing={existing}
        onCancel={onClose}
        onChoose={setTarget}
      />
    );
  return <Restore hostId={hostId} source={source} target={target} onClose={onClose} />;
}

function TargetForm({
  source,
  volumes,
  existing,
  onCancel,
  onChoose,
}: {
  source: string;
  volumes: VolumeView[];
  existing?: string;
  onCancel: () => void;
  onChoose: (target: Target) => void;
}) {
  const [kind, setKind] = useState<'new' | 'existing'>(existing ? 'existing' : 'new');
  const [name, setName] = useState(volumeNameFromFile(source));
  const [into, setInto] = useState(existing ?? '');
  const taken = kind === 'new' && volumes.some((v) => v.name === name.trim());
  const ready = kind === 'new' ? name.trim() !== '' && !taken : into !== '';
  return (
    <Dialog open onOpenChange={(open) => !open && onCancel()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('volumes.restore.title')}</DialogTitle>
          <DialogDescription className="break-all">
            {t('volumes.restore.from', { path: source })}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (ready)
              onChoose(
                kind === 'new'
                  ? { volume: name.trim(), create: true }
                  : { volume: into, create: false },
              );
          }}
        >
          <ToggleGroup
            type="single"
            variant="outline"
            value={kind}
            onValueChange={(v) => v && setKind(v as 'new' | 'existing')}
            aria-label={t('volumes.restore.into')}
          >
            <ToggleGroupItem value="new">{t('volumes.restore.newVolume')}</ToggleGroupItem>
            <ToggleGroupItem value="existing" disabled={volumes.length === 0}>
              {t('volumes.restore.existingVolume')}
            </ToggleGroupItem>
          </ToggleGroup>
          {kind === 'new' ? (
            <div className="grid gap-1.5">
              <Label htmlFor="restore-name">{t('volumes.createDialog.name')}</Label>
              <Input
                id="restore-name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                autoFocus
              />
              {taken && (
                <p className="text-destructive text-sm">
                  {t('volumes.restore.nameTaken', { volume: name.trim() })}
                </p>
              )}
            </div>
          ) : (
            <div className="grid gap-1.5">
              <Label htmlFor="restore-into">{t('volumes.restore.volume')}</Label>
              <Select value={into} onValueChange={setInto}>
                <SelectTrigger id="restore-into">
                  <SelectValue placeholder={t('volumes.restore.chooseVolume')} />
                </SelectTrigger>
                <SelectContent>
                  {volumes.map((v) => (
                    <SelectItem key={v.name} value={v.name}>
                      {v.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          )}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onCancel}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!ready}>
              {t('volumes.restore.start')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

function Restore({
  hostId,
  source,
  target,
  onClose,
}: {
  hostId: string;
  source: string;
  target: Target;
  onClose: () => void;
}) {
  const restore = useRestoreVolume(hostId);
  const ended = useRef(false);
  const { volume, create } = target;
  return (
    <VolumeActionFlow
      hostId={hostId}
      volume={volume}
      mode="restore"
      checkUsers={!create}
      title={t('volumes.restore.titleInto', { volume })}
      onClose={onClose}
      transfer={({ pulledImage, replace, onNotEmpty, onFinished }) => (
        <TransferDialog
          key={String(replace)}
          title={t('volumes.restore.titleInto', { volume })}
          description={t('volumes.restore.from', { path: source })}
          counted={t('volumes.transfer.sent')}
          start={(onEvent, onError) => {
            ended.current = false;
            return restore(
              source,
              { volume, create, replace },
              pulledImage,
              (event) => {
                if (event.type === 'error' && event.error.code === 'volume_not_empty')
                  return onNotEmpty();
                if (event.type === 'done' || event.type === 'error') ended.current = true;
                onEvent(event);
              },
              (error) => {
                ended.current = true;
                onError(error);
              },
            );
          }}
          onClose={() => {
            if (!ended.current) toast.warning(t('volumes.restore.cancelled', { volume }));
            onFinished();
          }}
        />
      )}
    />
  );
}
