import { useQuery } from '@tanstack/react-query';
import type { SwarmObject } from '@/shared/api';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
} from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/** A config's content, or its size when it isn't text. */
export function ConfigContentDialog({
  swarmId,
  config,
  onClose,
}: {
  swarmId: string;
  config: SwarmObject;
  onClose: () => void;
}) {
  const content = useQuery(swarmFeatureQueries.configContent(swarmId, config.id));
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{config.name}</DialogTitle>
        </DialogHeader>
        {content.error ? (
          <ErrorMessage error={content.error} onRetry={() => void content.refetch()} />
        ) : !content.data ? (
          <Spinner />
        ) : content.data.text !== null ? (
          <pre className="bg-muted max-h-[60vh] overflow-auto rounded-md p-3 font-mono text-xs">
            {content.data.text}
          </pre>
        ) : (
          <p className="text-muted-foreground text-sm">
            {t('swarm.objects.binary', { size: content.data.size })}
          </p>
        )}
      </DialogContent>
    </Dialog>
  );
}
