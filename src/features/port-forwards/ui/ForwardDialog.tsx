import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { ForwardTarget } from '@/shared/api';
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
  Spinner,
} from '@/shared/ui';
import { useStartForward } from '../api/mutations';
import { forwardQueries } from '../api/queries';
import { parseLocalPort, targetLabel } from '../model/forwards';
import { t } from '@/shared/i18n';

/** Forwards a local port to a Pod's or Service's port, the same number when it's free. */
export function ForwardDialog({
  clusterId,
  target,
  port,
  onClose,
}: {
  clusterId: string;
  target: ForwardTarget;
  port: number;
  onClose: () => void;
}) {
  const free = useQuery(forwardQueries.free(port));
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>
            {t('portforwards.dialog.title', { target: targetLabel(target), port })}
          </DialogTitle>
          <DialogDescription>
            {t('portforwards.dialog.description', { namespace: target.namespace })}
          </DialogDescription>
        </DialogHeader>
        {free.data === undefined ? (
          <Spinner />
        ) : (
          <ForwardForm
            clusterId={clusterId}
            target={target}
            port={port}
            // The same number when it's free, else any free port.
            initial={free.data ? String(port) : ''}
            onClose={onClose}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function ForwardForm({
  clusterId,
  target,
  port,
  initial,
  onClose,
}: {
  clusterId: string;
  target: ForwardTarget;
  port: number;
  initial: string;
  onClose: () => void;
}) {
  const [text, setText] = useState(initial);
  const local = parseLocalPort(text);
  const start = useStartForward(clusterId, onClose);
  return (
    <form
      className="grid gap-3"
      onSubmit={(e) => {
        e.preventDefault();
        if (local !== undefined) start.mutate({ target, remotePort: port, localPort: local });
      }}
    >
      <div className="grid gap-1.5">
        <Label htmlFor="forward-local">{t('portforwards.dialog.localPort')}</Label>
        <Input
          id="forward-local"
          inputMode="numeric"
          className="w-32 font-mono"
          placeholder={t('portforwards.dialog.anyPort')}
          value={text}
          onChange={(e) => setText(e.target.value)}
          aria-invalid={local === undefined}
          autoFocus
        />
        <p className="text-muted-foreground text-xs">{t('portforwards.dialog.loopback')}</p>
      </div>
      {start.error && <ErrorMessage error={start.error} />}
      <DialogFooter>
        <Button type="button" variant="ghost" onClick={onClose} disabled={start.isPending}>
          {t('common.cancel')}
        </Button>
        <Button type="submit" disabled={start.isPending || local === undefined}>
          {t('portforwards.dialog.submit')}
        </Button>
      </DialogFooter>
    </form>
  );
}
