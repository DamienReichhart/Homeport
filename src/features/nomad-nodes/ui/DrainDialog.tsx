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
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { useDrainNode } from '../api/mutations';
import { parseDuration } from '../model/nodes';
import { t } from '@/shared/i18n';

/**
 * Asks before draining a node: a deadline (an hour by default, as Nomad's
 * CLI), or none, and whether system jobs stay.
 */
export function DrainDialog({
  clusterId,
  node,
  onClose,
}: {
  clusterId: string;
  node: { id: string; name: string };
  onClose: () => void;
}) {
  const [deadline, setDeadline] = useState('1h');
  const [noDeadline, setNoDeadline] = useState(false);
  const [ignoreSystemJobs, setIgnoreSystemJobs] = useState(false);
  const drain = useDrainNode(clusterId, node.id, onClose);
  const secs = noDeadline ? null : parseDuration(deadline);
  const valid = noDeadline || secs !== null;
  return (
    <Dialog open onOpenChange={(open) => !open && !drain.isPending && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('nomad.drain.title', { node: node.name })}</DialogTitle>
          <DialogDescription>{t('nomad.drain.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (valid) drain.mutate({ deadlineSecs: secs, ignoreSystemJobs });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="drain-deadline">{t('nomad.drain.deadline')}</Label>
            <Input
              id="drain-deadline"
              value={deadline}
              disabled={noDeadline}
              aria-invalid={!valid}
              onChange={(e) => setDeadline(e.target.value)}
              className="w-32 font-mono"
            />
            <p className="text-muted-foreground text-xs">{t('nomad.drain.deadlineHelp')}</p>
          </div>
          <div className="flex items-center gap-2">
            <Checkbox
              id="drain-no-deadline"
              checked={noDeadline}
              onCheckedChange={(v) => setNoDeadline(v === true)}
            />
            <Label htmlFor="drain-no-deadline">{t('nomad.drain.noDeadline')}</Label>
          </div>
          <div className="flex items-center gap-2">
            <Checkbox
              id="drain-ignore-system"
              checked={ignoreSystemJobs}
              onCheckedChange={(v) => setIgnoreSystemJobs(v === true)}
            />
            <Label htmlFor="drain-ignore-system">{t('nomad.drain.ignoreSystemJobs')}</Label>
          </div>
          {drain.error && <ErrorMessage error={drain.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={drain.isPending}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!valid || drain.isPending}>
              {t('nomad.drain.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
