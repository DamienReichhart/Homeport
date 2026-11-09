import { useState } from 'react';
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
import { useStopJob, type JobRef } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Asks before stopping a job; purging it at once is an option, off by default. */
export function StopJobDialog({
  job,
  clusterName,
  onClose,
}: {
  job: JobRef;
  clusterName: string;
  onClose: () => void;
}) {
  const [purge, setPurge] = useState(false);
  const stop = useStopJob(job, onClose);
  return (
    <Dialog open onOpenChange={(open) => !open && !stop.isPending && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle className="text-destructive">
            {t('nomad.stop.title', { job: job.jobId })}
          </DialogTitle>
          <DialogDescription>
            {t('nomad.stop.description', {
              job: job.jobId,
              namespace: job.namespace,
              cluster: clusterName,
            })}
          </DialogDescription>
        </DialogHeader>
        <div className="flex items-start gap-2">
          <Checkbox id="stop-purge" checked={purge} onCheckedChange={(v) => setPurge(v === true)} />
          <div className="grid gap-1">
            <Label htmlFor="stop-purge">{t('nomad.stop.purge')}</Label>
            <p className="text-muted-foreground text-xs">{t('nomad.stop.purgeHelp')}</p>
          </div>
        </div>
        <DialogFooter>
          <Button variant="ghost" onClick={onClose} disabled={stop.isPending}>
            {t('common.cancel')}
          </Button>
          <Button
            variant="destructive"
            className="bg-destructive hover:bg-destructive/90 text-white"
            disabled={stop.isPending}
            onClick={() => stop.mutate(purge)}
          >
            {purge ? t('nomad.stop.confirmPurge') : t('nomad.stop.confirm')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
