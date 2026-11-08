import { useState } from 'react';
import type { QuadletUnit } from '@/shared/api';
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  Button,
  Label,
  Switch,
} from '@/shared/ui';
import { useRemoveUnit } from '../../api/mutations';
import { isActive } from '../../model/units';
import { t } from '@/shared/i18n';

/** Removing a unit, with the choice of stopping its service as it goes. */
export function RemoveUnitDialog({
  hostId,
  unit,
  onClose,
  onRemoved,
}: {
  hostId: string;
  unit: QuadletUnit | null;
  onClose: () => void;
  onRemoved?: () => void;
}) {
  const [stop, setStop] = useState(true);
  const remove = useRemoveUnit(hostId);
  const running = unit ? isActive(unit.status) : false;
  return (
    <AlertDialog open={unit !== null} onOpenChange={(open) => !open && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>
            {t('quadlets.remove.title', { name: unit?.name ?? '' })}
          </AlertDialogTitle>
          <AlertDialogDescription>{t('quadlets.remove.description')}</AlertDialogDescription>
        </AlertDialogHeader>
        <div className="flex items-center gap-2">
          <Switch id="unit-stop" checked={stop} onCheckedChange={setStop} />
          <Label htmlFor="unit-stop">
            {running ? t('quadlets.remove.stopRunning') : t('quadlets.remove.stop')}
          </Label>
        </div>
        <AlertDialogFooter>
          <AlertDialogCancel>{t('common.cancel')}</AlertDialogCancel>
          <Button
            variant="destructive"
            disabled={remove.isPending}
            onClick={() => {
              if (!unit?.name) return;
              remove.mutate(
                { name: unit.name, force: stop },
                {
                  onSuccess: () => {
                    onClose();
                    onRemoved?.();
                  },
                },
              );
            }}
          >
            {t('common.remove')}
          </Button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
