import { useQuery } from '@tanstack/react-query';
import { useEffect, useReducer, useRef, useState, type ReactNode } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
} from '@/shared/ui';
import { usePauseContainers } from '../api/mutations';
import { volumeQueries } from '../api/queries';
import { reduceFlow, startFlow } from '../model/transferFlow';
import { HelperImageGate } from './HelperImageGate';
import { t } from '@/shared/i18n';

const TEXT = {
  backup: {
    running: 'volumes.flow.running.backup',
    anyway: 'volumes.flow.anyway.backup',
    stop: 'volumes.flow.stopDuring.backup',
  },
  restore: {
    running: 'volumes.flow.running.restore',
    anyway: null,
    stop: 'volumes.flow.stopDuring.restore',
  },
  clone: {
    running: 'volumes.flow.running.clone',
    anyway: 'volumes.flow.anyway.clone',
    stop: 'volumes.flow.stopDuring.clone',
  },
} as const;

/** What the transfer step is given. */
export interface TransferStep {
  pulledImage: boolean;
  /** Replace what a restored volume holds; asked for after `onNotEmpty`. */
  replace: boolean;
  /** A restore found files in the volume: ask before replacing them. */
  onNotEmpty: () => void;
  /** The transfer's dialog closed: start what was stopped, then close. */
  onFinished: () => void;
}

/**
 * Takes a backup, restore or clone of `volume` to the point where it can
 * run: asks what to do about the running containers using it (a restore
 * must stop them), gets the helper's image, then renders `transfer`, which
 * shows the progress. Containers stopped for it are started again after.
 */
export function VolumeActionFlow({
  hostId,
  volume,
  mode,
  checkUsers = true,
  title,
  onClose,
  transfer,
}: {
  hostId: string;
  volume: string;
  mode: 'backup' | 'restore' | 'clone';
  /** A new volume has no users to ask about. */
  checkUsers?: boolean;
  title: string;
  onClose: () => void;
  transfer: (step: TransferStep) => ReactNode;
}) {
  const [step, dispatch] = useReducer(reduceFlow, startFlow);
  const users = useQuery({ ...volumeQueries.runningUsers(hostId, volume), enabled: checkUsers });
  const pause = usePauseContainers(hostId);
  const [stopped, setStopped] = useState<string[]>([]);

  useEffect(() => {
    if (!checkUsers) dispatch({ type: 'checked', users: [] });
    else if (users.data) dispatch({ type: 'checked', users: users.data });
  }, [checkUsers, users.data]);

  const finish = () => {
    void pause.restart(stopped);
    onClose();
  };
  const proceed = async (stop: string[]) => {
    try {
      await pause.stop.mutateAsync(stop);
    } catch {
      // Shown in place; those it did stop are started again on closing.
      setStopped(stop);
      return;
    }
    setStopped(stop);
    dispatch({ type: 'proceed', stop });
  };

  if (step.kind === 'transfer')
    return transfer({
      pulledImage: step.pulledImage,
      replace: step.replace,
      onNotEmpty: () => dispatch({ type: 'notEmpty' }),
      onFinished: finish,
    });

  return (
    <Dialog open onOpenChange={(open) => !open && finish()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          {step.kind === 'running' && (
            <DialogDescription>
              {t(TEXT[mode].running, {
                volume,
                containers: step.users.join(', '),
                count: step.users.length,
              })}
            </DialogDescription>
          )}
          {step.kind === 'replace' && (
            <DialogDescription>{t('volumes.flow.replace', { volume })}</DialogDescription>
          )}
        </DialogHeader>
        {step.kind === 'checking' &&
          (users.error ? (
            <ErrorMessage error={users.error} onRetry={() => void users.refetch()} />
          ) : (
            <Spinner />
          ))}
        {step.kind === 'image' && (
          <HelperImageGate hostId={hostId} onCancel={finish}>
            {(pulledImage) => (
              <Ready onReady={() => dispatch({ type: 'imageReady', pulledImage })} />
            )}
          </HelperImageGate>
        )}
        {step.kind === 'running' && (
          <>
            {pause.stop.error && <ErrorMessage error={pause.stop.error} />}
            <DialogFooter>
              <Button variant="outline" onClick={finish}>
                {t('common.cancel')}
              </Button>
              {TEXT[mode].anyway && (
                <Button variant="outline" onClick={() => dispatch({ type: 'proceed', stop: [] })}>
                  {t(TEXT[mode].anyway)}
                </Button>
              )}
              <Button disabled={pause.stop.isPending} onClick={() => void proceed(step.users)}>
                {t(TEXT[mode].stop, { count: step.users.length })}
              </Button>
            </DialogFooter>
          </>
        )}
        {step.kind === 'replace' && (
          <DialogFooter>
            <Button variant="outline" onClick={finish}>
              {t('common.cancel')}
            </Button>
            <Button variant="destructive" onClick={() => dispatch({ type: 'replace' })}>
              {t('volumes.flow.replaceConfirm')}
            </Button>
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  );
}

/** Tells the flow the helper's image is there, once rendered. */
function Ready({ onReady }: { onReady: () => void }) {
  const ready = useRef(onReady);
  useEffect(() => ready.current(), []);
  return <Spinner />;
}
