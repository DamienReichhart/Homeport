import { useState } from 'react';
import type { PruneSelection } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Label,
} from '@/shared/ui';
import { usePruneHost } from '../../api/mutations';
import { defaultPruneSelection, PRUNE_OPTIONS } from '../../model/prune';
import { t } from '@/shared/i18n';

/** Global prune for one Host, with an explicit choice of what goes. */
export function PruneDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const [selection, setSelection] = useState<PruneSelection>(defaultPruneSelection);
  const prune = usePruneHost(hostId, () => onOpenChange(false));

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('overview.prune.title')}</DialogTitle>
          <DialogDescription>{t('overview.prune.description')}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          {PRUNE_OPTIONS.map((o) => (
            <div key={o.key} className="flex items-start gap-3">
              <Checkbox
                id={o.key}
                checked={selection[o.key]}
                onCheckedChange={(v) => setSelection((s) => ({ ...s, [o.key]: v === true }))}
              />
              <Label htmlFor={o.key} className="grid gap-0.5 font-normal">
                <span className={o.destructive ? 'text-destructive font-medium' : 'font-medium'}>
                  {t(o.label)}
                </span>
                <span
                  className={
                    o.destructive ? 'text-destructive text-xs' : 'text-muted-foreground text-xs'
                  }
                >
                  {t(o.detail)}
                </span>
              </Label>
            </div>
          ))}
        </div>
        {prune.error && <ErrorMessage error={prune.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            {t('common.cancel')}
          </Button>
          <Button
            variant="destructive"
            className={
              selection.volumes ? 'bg-destructive text-white hover:bg-destructive/90' : undefined
            }
            disabled={prune.isPending || !Object.values(selection).some(Boolean)}
            onClick={() => prune.mutate(selection)}
          >
            {selection.volumes
              ? t('overview.prune.confirmWithVolumes')
              : t('overview.prune.confirm')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
