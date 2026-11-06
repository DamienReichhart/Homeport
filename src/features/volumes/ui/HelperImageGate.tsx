import { useQuery } from '@tanstack/react-query';
import { DownloadIcon } from 'lucide-react';
import { useEffect, useReducer, useRef, useState, type ReactNode } from 'react';
import { formatBytes } from '@/shared/lib';
import { Button, ErrorMessage, Progress, Spinner } from '@/shared/ui';
import { usePullHelperImage } from '../api/helper';
import { volumeQueries } from '../api/queries';
import { initialPull, pullPercent, reducePull } from '../model/pull';
import { t } from '@/shared/i18n';

/**
 * Renders `children` once the Host has the helper container's image
 * (ADR 0015). Without it, says what it is and pulls it only once the user
 * agrees; `children` is told whether Homeport pulled it, so it can be
 * removed again afterwards.
 */
export function HelperImageGate({
  hostId,
  onCancel,
  children,
}: {
  hostId: string;
  onCancel: () => void;
  children: (pulledImage: boolean) => ReactNode;
}) {
  const image = useQuery(volumeQueries.helperImage(hostId));
  const pull = usePullHelperImage(hostId);
  const [pulling, setPulling] = useState(false);
  const [pulled, setPulled] = useState(false);
  const [error, setError] = useState<unknown>(null);
  const [progress, dispatch] = useReducer(reducePull, initialPull);
  const cancel = useRef<(() => void) | null>(null);
  useEffect(() => () => cancel.current?.(), []);

  const start = () => {
    setError(null);
    setPulling(true);
    cancel.current = pull(
      (event) => {
        dispatch(event);
        if (event.type === 'done') {
          setPulled(true);
          setPulling(false);
        }
        if (event.type === 'error') {
          setError(event.error);
          setPulling(false);
        }
      },
      (e) => {
        setError(e);
        setPulling(false);
      },
    );
  };

  if (pulled) return children(true);
  if (image.isPending) return <Spinner />;
  if (image.error) return <ErrorMessage error={image.error} onRetry={() => void image.refetch()} />;
  if (image.data.present) return children(false);

  const percent = pullPercent(progress);
  return (
    <div className="grid gap-4">
      <p className="text-sm">
        {t('volumes.helper.needed', {
          image: image.data.image,
          size: formatBytes(image.data.size, 0),
        })}
      </p>
      <p className="text-muted-foreground text-sm">{t('volumes.helper.removed')}</p>
      {pulling && (
        <div className="grid gap-1" role="status">
          <span className="text-sm">
            {t('volumes.helper.pulling', { image: image.data.image })}
          </span>
          <Progress value={percent ?? 0} aria-label={t('volumes.helper.progress')} />
        </div>
      )}
      {error !== null && <ErrorMessage error={error} onRetry={start} />}
      <div className="flex justify-end gap-2">
        <Button variant="outline" onClick={onCancel}>
          {t('common.cancel')}
        </Button>
        {error === null && (
          <Button disabled={pulling} onClick={start}>
            <DownloadIcon /> {t('volumes.helper.pull', { image: image.data.image })}
          </Button>
        )}
      </div>
    </div>
  );
}
