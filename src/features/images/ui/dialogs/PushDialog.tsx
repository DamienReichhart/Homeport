import { useEffect, useReducer, useRef, useState } from 'react';
import { MissingLogin, RegistryLogin } from '@/features/registries';
import type { Image, PullEvent } from '@/shared/api';
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
  Input,
  Label,
  Progress,
} from '@/shared/ui';
import { useTagImage } from '../../api/mutations';
import { usePushImage } from '../../api/streams';
import { imageName } from '../../model/imageName';
import { describePush, initialPush, reducePush, type PushState } from '../../model/push';

/**
 * Pushes an image to a registry under one of its names, or under a new one
 * it is given first, and shows the upload's progress.
 */
export function PushDialog({
  hostId,
  image,
  onClose,
}: {
  hostId: string;
  image: Image;
  onClose: () => void;
}) {
  const push = usePushImage(hostId);
  const [target, setTarget] = useState(image.tags[0] ?? '');
  // A new object for every attempt, so pushing the same name again restarts it.
  const [pushing, setPushing] = useState<{ name: string } | null>(null);
  const [state, dispatch] = useReducer(
    (s: PushState, e: PullEvent | 'reset') => (e === 'reset' ? initialPush : reducePush(s, e)),
    initialPush,
  );
  const stop = useRef<(() => void) | null>(null);
  const name = target.trim();
  const isNew = name !== '' && !image.tags.includes(name);
  const tag = useTagImage(hostId, image.id, () => setPushing({ name }));

  useEffect(() => {
    if (!pushing) return;
    dispatch('reset');
    const cancel = push(pushing.name, dispatch, (error) => dispatch({ type: 'error', error }));
    stop.current = cancel;
    return cancel;
  }, [pushing, push]);

  const busy = tag.isPending || (pushing !== null && !state.done && !state.error);
  const close = () => {
    stop.current?.();
    onClose();
  };
  const start = () => {
    stop.current?.();
    if (isNew) tag.mutate(name);
    else setPushing({ name });
  };

  return (
    <Dialog open onOpenChange={(open) => !open && close()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>Push {imageName(image)}</DialogTitle>
          <DialogDescription>
            To the registry its name starts with, e.g. ghcr.io/org/app:v2, or Docker Hub for names
            like org/app:v2.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (name) start();
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="push-target">Name to push</Label>
            <div className="flex gap-2">
              <Input
                id="push-target"
                list="push-names"
                value={target}
                onChange={(e) => setTarget(e.target.value)}
                placeholder="registry.example.com/team/app:v2"
                autoFocus
                disabled={busy}
                spellCheck={false}
              />
              <datalist id="push-names">
                {image.tags.map((t) => (
                  <option key={t} value={t} />
                ))}
              </datalist>
              <Button type="submit" disabled={busy || !name}>
                Push
              </Button>
            </div>
            {isNew && (
              <p className="text-muted-foreground text-xs">
                The image is given this name first, as Tag does.
              </p>
            )}
            <RegistryLogin reference={name} verb="Pushed" />
          </div>
          {!pushing && <MissingLogin reference={name} />}
        </form>
        {tag.error && <ErrorMessage error={tag.error} />}
        {pushing && (
          <div className="grid max-h-80 gap-2 overflow-y-auto">
            {state.messages.map((m, i) => (
              <p key={i} className="text-muted-foreground font-mono text-xs">
                {m}
              </p>
            ))}
            <p className="text-sm">{describePush(state)}</p>
            {state.total ? (
              <div className="grid gap-1">
                <span className="text-muted-foreground font-mono text-xs">
                  Uploading {formatBytes(state.current ?? 0)} / {formatBytes(state.total)}
                </span>
                <Progress value={((state.current ?? 0) / state.total) * 100} />
              </div>
            ) : null}
            {state.error && <ErrorMessage error={state.error} />}
            {state.done && <p className="text-success text-sm">Done.</p>}
          </div>
        )}
        <DialogFooter>
          <Button variant="ghost" onClick={close}>
            {busy ? 'Cancel' : 'Close'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
