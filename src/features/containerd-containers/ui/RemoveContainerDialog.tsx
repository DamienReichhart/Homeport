import { useState } from 'react';
import type { ContainerdContainer } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Label,
} from '@/shared/ui';
import { useRemoveContainer } from '../api/mutations';
import { isUp, managedByKubernetes } from '../model/containers';
import { t } from '@/shared/i18n';

/**
 * Asks before removing a container, its task and its snapshot, which the
 * user can keep; a running task is killed first.
 */
export function RemoveContainerDialog({
  hostId,
  namespace,
  container,
  onClose,
  onRemoved,
}: {
  hostId: string;
  namespace: string;
  container: ContainerdContainer | null;
  onClose: () => void;
  onRemoved?: () => void;
}) {
  const [removeSnapshot, setRemoveSnapshot] = useState(true);
  const remove = useRemoveContainer(hostId, namespace);
  const up = container ? isUp(container.state) : false;
  return (
    <Dialog open={!!container} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-md">
        {container && (
          <>
            <DialogHeader>
              <DialogTitle>
                {t('containerd.containers.remove.title', { name: container.name })}
              </DialogTitle>
              <DialogDescription>
                {up
                  ? t('containerd.containers.remove.running')
                  : t('containerd.containers.remove.stopped')}
              </DialogDescription>
            </DialogHeader>
            {managedByKubernetes(container) && (
              <p className="text-warning text-sm">{t('containerd.containers.kubelet')}</p>
            )}
            {container.snapshotKey && (
              <div className="flex items-center gap-2">
                <Checkbox
                  id="remove-snapshot"
                  checked={removeSnapshot}
                  onCheckedChange={(v) => setRemoveSnapshot(v === true)}
                />
                <Label htmlFor="remove-snapshot">
                  {t('containerd.containers.remove.snapshot', { key: container.snapshotKey })}
                </Label>
              </div>
            )}
            <DialogFooter>
              <Button variant="outline" onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button
                variant="destructive"
                disabled={remove.isPending}
                onClick={() =>
                  remove
                    .mutateAsync({ container, force: up, keepSnapshot: !removeSnapshot })
                    .then(() => {
                      onClose();
                      onRemoved?.();
                    })
                    .catch(() => {})
                }
              >
                {up ? t('containerd.containers.remove.killAndRemove') : t('common.remove')}
              </Button>
            </DialogFooter>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}
