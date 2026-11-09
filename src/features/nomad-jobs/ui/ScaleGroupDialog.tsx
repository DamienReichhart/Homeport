import { useState } from 'react';
import type { NomadTaskGroupView } from '@/shared/api';
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
} from '@/shared/ui';
import { useScaleGroup, type JobRef } from '../api/mutations';
import { parseCount } from '../model/jobs';
import { t } from '@/shared/i18n';

/** Sets how many allocations a task group runs, within its scaling policy when it has one. */
export function ScaleGroupDialog({
  job,
  group,
  onClose,
}: {
  job: JobRef;
  group: NomadTaskGroupView;
  onClose: () => void;
}) {
  const current = group.counts.desired ?? group.counts.running;
  const [text, setText] = useState(String(current));
  const scaling = group.scaling;
  const count = parseCount(text, scaling);
  const scale = useScaleGroup(job, onClose);
  const name = group.counts.name;
  return (
    <Dialog open onOpenChange={(open) => !open && !scale.isPending && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('nomad.scale.title', { group: name })}</DialogTitle>
          <DialogDescription>
            {t('nomad.scale.description', { job: job.jobId, current })}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (count !== null) scale.mutate({ group: name, count });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="scale-count">{t('nomad.scale.count')}</Label>
            <Input
              id="scale-count"
              inputMode="numeric"
              value={text}
              onChange={(e) => {
                scale.reset();
                setText(e.target.value);
              }}
              aria-invalid={count === null}
              className="w-32"
              autoFocus
            />
            {scaling && (scaling.min !== null || scaling.max !== null) && (
              <p className="text-muted-foreground text-xs">
                {t('nomad.scale.bounds', {
                  min: scaling.min ?? 0,
                  max: scaling.max ?? '∞',
                })}
              </p>
            )}
            {scaling?.enabled && (
              <p className="text-muted-foreground text-xs">{t('nomad.scale.autoscaled')}</p>
            )}
          </div>
          {scale.error && <ErrorMessage error={scale.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={scale.isPending}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={scale.isPending || count === null}>
              {t('nomad.scale.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
