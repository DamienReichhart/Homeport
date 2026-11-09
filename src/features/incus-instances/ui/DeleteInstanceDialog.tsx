import { useState } from 'react';
import type { IncusInstance } from '@/shared/api';
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
import { useDeleteInstance } from '../api/mutations';
import { isUp } from '../model/instances';
import { t } from '@/shared/i18n';

/**
 * Asks before deleting an Instance, naming the snapshots that go with it;
 * a running one is only deleted once the user agrees to stop it first.
 */
export function DeleteInstanceDialog({
  hostId,
  project,
  instance,
  snapshotNames,
  onClose,
  onDeleted,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance | null;
  /** Its snapshots' names, when known (the Instance's page). */
  snapshotNames?: string[];
  onClose: () => void;
  onDeleted?: () => void;
}) {
  const [stopFirst, setStopFirst] = useState(false);
  const remove = useDeleteInstance(hostId, project);
  const up = instance ? isUp(instance.status) : false;
  return (
    <Dialog
      open={!!instance}
      onOpenChange={(open) => {
        if (!open) {
          setStopFirst(false);
          onClose();
        }
      }}
    >
      <DialogContent className="sm:max-w-md">
        {instance && (
          <>
            <DialogHeader>
              <DialogTitle>
                {t('incus.instances.delete.title', { name: instance.name })}
              </DialogTitle>
              <DialogDescription>
                {t('incus.instances.delete.description', { project })}
              </DialogDescription>
            </DialogHeader>
            {instance.snapshots > 0 && (
              <p className="text-warning text-sm">
                {snapshotNames && snapshotNames.length > 0
                  ? t('incus.instances.delete.snapshotsNamed', {
                      count: snapshotNames.length,
                      names: snapshotNames.join(', '),
                    })
                  : t('incus.instances.delete.snapshots', { count: instance.snapshots })}
              </p>
            )}
            {up && (
              <div className="flex items-center gap-2">
                <Checkbox
                  id="stop-first"
                  checked={stopFirst}
                  onCheckedChange={(v) => setStopFirst(v === true)}
                />
                <Label htmlFor="stop-first">{t('incus.instances.delete.stopFirst')}</Label>
              </div>
            )}
            <DialogFooter>
              <Button variant="outline" onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button
                variant="destructive"
                disabled={remove.isPending || (up && !stopFirst)}
                onClick={() =>
                  remove
                    .mutateAsync({ name: instance.name, stopFirst: up && stopFirst })
                    .then(() => {
                      setStopFirst(false);
                      onClose();
                      onDeleted?.();
                    })
                    .catch(() => {})
                }
              >
                {up ? t('incus.instances.delete.stopAndDelete') : t('incus.delete')}
              </Button>
            </DialogFooter>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
}
