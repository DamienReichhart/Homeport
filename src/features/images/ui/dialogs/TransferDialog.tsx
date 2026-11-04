import { useEffect, useReducer, useRef } from 'react';
import type { AppError } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Progress,
} from '@/shared/ui';
import { useLoadImages, useSaveImages } from '../../api/streams';
import { initialTransfer, reduceTransfer } from '../../model/transfer';

/** A save of `names` to `path`, or a load from `path`. */
export type Transfer =
  { kind: 'save'; names: string[]; path: string } | { kind: 'load'; path: string };

/** Runs one save or load and shows its progress, with a way to cancel. */
export function TransferDialog({
  hostId,
  transfer,
  onClose,
}: {
  hostId: string;
  transfer: Transfer;
  onClose: () => void;
}) {
  const save = useSaveImages(hostId);
  const load = useLoadImages(hostId);
  const [state, dispatch] = useReducer(reduceTransfer, initialTransfer);
  const stop = useRef<(() => void) | null>(null);

  useEffect(() => {
    const fail = (error: AppError) => dispatch({ type: 'error', error });
    const cancel =
      transfer.kind === 'save'
        ? save(transfer.names, transfer.path, dispatch, fail)
        : load(transfer.path, dispatch, fail);
    stop.current = cancel;
    return cancel;
  }, [transfer, save, load]);

  const busy = !state.done && !state.error;
  const close = () => {
    stop.current?.();
    onClose();
  };
  const title =
    transfer.kind === 'save'
      ? `Saving ${transfer.names.length === 1 ? transfer.names[0] : `${transfer.names.length} images`}`
      : 'Loading images';

  return (
    <Dialog open onOpenChange={(open) => !open && close()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription className="break-all">
            {transfer.kind === 'save' ? 'To' : 'From'} {transfer.path}
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-2">
          <div className="text-muted-foreground flex justify-between font-mono text-xs">
            <span>{transfer.kind === 'save' ? 'Written' : 'Sent'}</span>
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
          {state.done && (
            <p className="text-success text-sm">
              Done
              {transfer.kind === 'load' && state.images.length > 0
                ? `: ${state.images.join(', ')}`
                : '.'}
            </p>
          )}
        </div>
        <DialogFooter>
          <Button variant={busy ? 'ghost' : 'default'} onClick={close}>
            {busy ? 'Cancel' : 'Close'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
