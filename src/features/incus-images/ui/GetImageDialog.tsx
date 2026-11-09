import { useQueryClient } from '@tanstack/react-query';
import { useEffect, useReducer, useRef, useState } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import type { InstanceType } from '@/shared/api';
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
  Switch,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { startImageCopy } from '../api/streams';
import { initialProgress, reduceProgress } from '../model/progress';
import { DEFAULT_SERVER } from '../model/remote';
import { OperationProgress } from './OperationProgress';
import { RemoteImagePicker } from './RemoteImagePicker';
import { t } from '@/shared/i18n';

/** Copies an image from a remote server into the project, showing the download. */
export function GetImageDialog({
  hostId,
  project,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  open: boolean;
  onClose: () => void;
}) {
  const client = useQueryClient();
  const [server, setServer] = useState(DEFAULT_SERVER);
  const [type, setType] = useState<InstanceType>('container');
  const [alias, setAlias] = useState('');
  const [autoUpdate, setAutoUpdate] = useState(true);
  const [keepAlias, setKeepAlias] = useState(true);
  const [started, setStarted] = useState(false);
  const [progress, dispatch] = useReducer(reduceProgress, initialProgress);
  const cancel = useRef<(() => void) | null>(null);

  // Closing stops following the copy; Incus finishes it anyway.
  useEffect(() => () => cancel.current?.(), []);

  const start = () => {
    setStarted(true);
    cancel.current = startImageCopy(
      hostId,
      project,
      { server, alias: alias.trim(), type, autoUpdate, addAlias: keepAlias },
      (event) => {
        dispatch(event);
        if (event.type === 'done') {
          toast.success(t('incus.images.copied', { alias: alias.trim(), project }));
          void client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'images') });
          void client.invalidateQueries({ queryKey: hostKeys.incusProjects(hostId) });
        }
      },
      (error) => dispatch({ type: 'error', error }),
    );
  };

  return (
    <Dialog open={open} onOpenChange={(next) => !next && onClose()}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>{t('incus.images.get.title')}</DialogTitle>
          <DialogDescription>{t('incus.images.get.description', { project })}</DialogDescription>
        </DialogHeader>
        {started ? (
          <OperationProgress state={progress} done={t('incus.images.get.done')} />
        ) : (
          <div className="grid gap-3">
            <div className="grid gap-1.5">
              <Label htmlFor="image-server">{t('incus.remote.server')}</Label>
              <Input id="image-server" value={server} onChange={(e) => setServer(e.target.value)} />
            </div>
            <ToggleGroup
              type="single"
              variant="outline"
              size="sm"
              value={type}
              onValueChange={(v) => v && setType(v as InstanceType)}
              className="justify-self-start"
            >
              <ToggleGroupItem value="container">{t('incus.type.container')}</ToggleGroupItem>
              <ToggleGroupItem value="virtual-machine">
                {t('incus.type.virtualMachine')}
              </ToggleGroupItem>
            </ToggleGroup>
            <RemoteImagePicker
              hostId={hostId}
              server={server}
              type={type}
              alias={alias}
              onAlias={setAlias}
            />
            <div className="flex items-center gap-2">
              <Switch id="auto-update" checked={autoUpdate} onCheckedChange={setAutoUpdate} />
              <Label htmlFor="auto-update">{t('incus.images.get.autoUpdate')}</Label>
            </div>
            <div className="flex items-center gap-2">
              <Switch id="keep-alias" checked={keepAlias} onCheckedChange={setKeepAlias} />
              <Label htmlFor="keep-alias">{t('incus.images.get.keepAlias')}</Label>
            </div>
          </div>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {started ? t('common.close') : t('common.cancel')}
          </Button>
          {!started && (
            <Button disabled={!alias.trim() || !server.trim()} onClick={start}>
              {t('incus.images.get.confirm')}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
