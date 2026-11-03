import { useQueryClient } from '@tanstack/react-query';
import { useEffect, useReducer, useRef, useState } from 'react';
import { toast } from 'sonner';
import { commands, keys, subscribe, type PullEvent } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { Button } from '@/shared/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/shared/ui/dialog';
import { Input } from '@/shared/ui/input';
import { Progress } from '@/shared/ui/progress';
import { initialPull, reducePull, type PullState } from './pull';

export function PullDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const client = useQueryClient();
  const [reference, setReference] = useState('');
  const [pulling, setPulling] = useState<string | null>(null);
  const [state, dispatch] = useReducer(
    (s: PullState, e: PullEvent | 'reset') => (e === 'reset' ? initialPull : reducePull(s, e)),
    initialPull,
  );
  const stop = useRef<(() => void) | null>(null);

  useEffect(() => {
    if (!pulling) return;
    dispatch('reset');
    const cancel = subscribe<PullEvent>(
      (ch) => commands.imagesPullSubscribe(hostId, pulling, ch),
      (e) => {
        dispatch(e);
        if (e.type === 'done') {
          toast.success(`Pulled ${pulling}`);
          void client.invalidateQueries({ queryKey: keys.images(hostId) });
        }
      },
      (error) => dispatch({ type: 'error', error }),
    );
    stop.current = cancel;
    return cancel;
  }, [hostId, pulling, client]);

  const busy = pulling !== null && !state.done && !state.error;
  const close = (next: boolean) => {
    if (!next) {
      stop.current?.();
      setPulling(null);
      dispatch('reset');
    }
    onOpenChange(next);
  };

  return (
    <Dialog open={open} onOpenChange={close}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>Pull an image</DialogTitle>
          <DialogDescription>
            From Docker Hub or any registry the host can reach, e.g. nginx:1.27 or
            ghcr.io/org/app:v2.
          </DialogDescription>
        </DialogHeader>
        <form
          className="flex gap-2"
          onSubmit={(e) => {
            e.preventDefault();
            if (reference.trim()) setPulling(reference.trim());
          }}
        >
          <Input
            value={reference}
            onChange={(e) => setReference(e.target.value)}
            placeholder="nginx:latest"
            autoFocus
            disabled={busy}
            aria-label="Image reference"
          />
          <Button type="submit" disabled={busy || !reference.trim()}>
            Pull
          </Button>
        </form>
        {pulling && (
          <div className="grid max-h-80 gap-2 overflow-y-auto">
            {state.messages.map((m, i) => (
              <p key={i} className="text-muted-foreground font-mono text-xs">
                {m}
              </p>
            ))}
            {state.layers.map((l) => (
              <div key={l.id} className="grid gap-1">
                <div className="flex justify-between font-mono text-xs">
                  <span>{l.id}</span>
                  <span className="text-muted-foreground">
                    {l.status}
                    {l.total ? ` ${formatBytes(l.current ?? 0)} / ${formatBytes(l.total)}` : ''}
                  </span>
                </div>
                <Progress
                  value={
                    l.total
                      ? ((l.current ?? 0) / l.total) * 100
                      : /complete|exists/i.test(l.status)
                        ? 100
                        : 0
                  }
                />
              </div>
            ))}
            {state.error && (
              <p role="alert" className="text-destructive text-sm">
                {state.error.message}
              </p>
            )}
            {state.done && <p className="text-success text-sm">Done.</p>}
          </div>
        )}
        <DialogFooter>
          <Button variant="ghost" onClick={() => close(false)}>
            {busy ? 'Cancel' : 'Close'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
