import { useEffect, useReducer, useRef, type ReactNode } from 'react';
import {
  initialTransfer,
  reduceTransfer,
  type AppError,
  type TransferEvent,
  type TransferState,
} from '@/shared/api';
import { formatBytes } from '@/shared/lib/format';
import { Button } from '../primitives/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../primitives/dialog';
import { Progress } from '../primitives/progress';
import { ErrorMessage } from './ErrorMessage';
import { t } from '@/shared/i18n';

/**
 * Runs one file transfer between this machine and a Host — a save, load,
 * import or export — and shows its progress, with a way to cancel.
 * `start` begins it and returns the canceller; it runs once.
 */
export function TransferDialog({
  title,
  description,
  counted,
  start,
  done = () => t('ui.transfer.done'),
  onClose,
}: {
  title: string;
  description: ReactNode;
  /** What the byte count measures: `Written` to a file or `Sent` to the Engine. */
  counted: string;
  start: (
    onEvent: (event: TransferEvent) => void,
    onError: (error: AppError) => void,
  ) => () => void;
  /** The line shown once it's finished. */
  done?: (state: TransferState) => string;
  onClose: () => void;
}) {
  const [state, dispatch] = useReducer(reduceTransfer, initialTransfer);
  const stop = useRef<(() => void) | null>(null);
  const begin = useRef(start);

  useEffect(() => {
    const cancel = begin.current(dispatch, (error) => dispatch({ type: 'error', error }));
    let stopped = false;
    // Closing and unmounting both stop it; the backend hears once.
    stop.current = () => {
      if (stopped) return;
      stopped = true;
      cancel();
    };
    return stop.current;
  }, []);

  const busy = !state.done && !state.error;
  const close = () => {
    stop.current?.();
    onClose();
  };

  return (
    <Dialog open onOpenChange={(open) => !open && close()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription className="break-all">{description}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-2">
          <div className="text-muted-foreground flex justify-between font-mono text-xs">
            <span>{counted}</span>
            <span>
              {formatBytes(state.bytes)}
              {state.total ? ` / ${formatBytes(state.total)}` : ''}
            </span>
          </div>
          <Progress
            value={state.done ? 100 : state.total ? (state.bytes / state.total) * 100 : 0}
          />
          {state.lines.map((line, i) => (
            <p key={i} className="text-muted-foreground font-mono text-xs">
              {line}
            </p>
          ))}
          {state.error && <ErrorMessage error={state.error} />}
          {state.done && <p className="text-success text-sm">{done(state)}</p>}
        </div>
        <DialogFooter>
          <Button variant={busy ? 'ghost' : 'default'} onClick={close}>
            {busy ? t('common.cancel') : t('common.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
