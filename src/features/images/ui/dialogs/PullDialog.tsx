import { useEffect, useReducer, useRef, useState } from 'react';
import { RegistryLogin } from '@/features/registries';
import type { AppError, PullEvent } from '@/shared/api';
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
  Progress,
} from '@/shared/ui';
import { usePullImage } from '../../api/streams';
import { searchTerm } from '../../model/hub';
import { initialPull, reducePull, type PullState } from '../../model/pull';
import { HubSearch } from './HubSearch';
import { HubTags } from './HubTags';
import { t } from '@/shared/i18n';

/** Starts a pull of `reference`; the returned function cancels it. */
export type ImagePuller = (
  reference: string,
  onEvent: (event: PullEvent) => void,
  onError: (error: AppError) => void,
) => () => void;

/**
 * Pulls an image by reference and shows each layer's progress, through the
 * Host's Docker Engine API unless `puller` pulls another way (containerd's).
 * `search` offers Docker Hub's search, which only a Docker Engine answers.
 */
export function PullDialog({
  hostId,
  open,
  onOpenChange,
  puller,
  search = true,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  puller?: ImagePuller;
  search?: boolean;
}) {
  const dockerPull = usePullImage(hostId);
  const pull = puller ?? dockerPull;
  const [reference, setReference] = useState('');
  const [pulling, setPulling] = useState<string | null>(null);
  /** The Docker Hub repository whose tags are listed, once one is chosen. */
  const [repository, setRepository] = useState<string | null>(null);
  const [state, dispatch] = useReducer(
    (s: PullState, e: PullEvent | 'reset') => (e === 'reset' ? initialPull : reducePull(s, e)),
    initialPull,
  );
  const stop = useRef<(() => void) | null>(null);

  useEffect(() => {
    if (!pulling) return;
    dispatch('reset');
    const cancel = pull(pulling, dispatch, (error) => dispatch({ type: 'error', error }));
    stop.current = cancel;
    return cancel;
  }, [pulling, pull]);

  const busy = pulling !== null && !state.done && !state.error;
  const close = (next: boolean) => {
    if (!next) {
      stop.current?.();
      setPulling(null);
      setRepository(null);
      dispatch('reset');
    }
    onOpenChange(next);
  };

  return (
    <Dialog open={open} onOpenChange={close}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('images.pullDialog.title')}</DialogTitle>
          <DialogDescription>{t('images.pullDialog.description')}</DialogDescription>
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
            onChange={(e) => {
              setReference(e.target.value);
              setRepository(null);
              if (pulling) {
                setPulling(null);
                dispatch('reset');
              }
            }}
            placeholder={t('images.pullDialog.placeholder')}
            autoFocus
            disabled={busy}
            aria-label={t('images.pullDialog.reference')}
          />
          <Button type="submit" disabled={busy || !reference.trim()}>
            {t('images.pullDialog.pull')}
          </Button>
        </form>
        <RegistryLogin reference={reference} verb="pull" />
        {!pulling && repository && (
          <HubTags
            repository={repository}
            onChoose={(tag) => setReference(`${repository}:${tag}`)}
            onBack={() => {
              setReference(repository);
              setRepository(null);
            }}
          />
        )}
        {search && !pulling && !repository && searchTerm(reference) && (
          <HubSearch
            hostId={hostId}
            term={searchTerm(reference)!}
            onChoose={(repo) => {
              setReference(repo);
              setRepository(repo);
            }}
          />
        )}
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
            {state.error && <ErrorMessage error={state.error} />}
            {state.done && <p className="text-success text-sm">{t('images.done')}</p>}
          </div>
        )}
        <DialogFooter>
          <Button variant="ghost" onClick={() => close(false)}>
            {busy ? t('common.cancel') : t('common.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
