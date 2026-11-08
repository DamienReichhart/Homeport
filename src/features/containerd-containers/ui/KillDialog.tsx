import { useState } from 'react';
import type { ContainerdContainer } from '@/shared/api';
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
} from '@/shared/ui';
import { useKill } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Sends a signal of the user's choice to a container's task, as `ctr task kill -s`. */
export function KillDialog({
  hostId,
  namespace,
  container,
  onClose,
}: {
  hostId: string;
  namespace: string;
  container: ContainerdContainer | null;
  onClose: () => void;
}) {
  const [signal, setSignal] = useState('SIGTERM');
  const kill = useKill(hostId, namespace);
  return (
    <Dialog open={!!container} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-sm">
        {container && (
          <form
            className="grid gap-4"
            onSubmit={(e) => {
              e.preventDefault();
              kill
                .mutateAsync({ container, signal: signal.trim() })
                .then(onClose)
                .catch(() => {});
            }}
          >
            <DialogHeader>
              <DialogTitle>
                {t('containerd.containers.kill.title', { name: container.name })}
              </DialogTitle>
              <DialogDescription>{t('containerd.containers.kill.description')}</DialogDescription>
            </DialogHeader>
            <div className="grid gap-1.5">
              <Label htmlFor="kill-signal">{t('containerd.containers.kill.signal')}</Label>
              <Input
                id="kill-signal"
                className="font-mono"
                value={signal}
                onChange={(e) => setSignal(e.target.value)}
              />
            </div>
            <DialogFooter>
              <Button type="button" variant="outline" onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button type="submit" disabled={!signal.trim() || kill.isPending}>
                {t('containerd.containers.kill.send')}
              </Button>
            </DialogFooter>
          </form>
        )}
      </DialogContent>
    </Dialog>
  );
}
