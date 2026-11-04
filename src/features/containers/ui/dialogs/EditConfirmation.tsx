import { useQuery } from '@tanstack/react-query';
import type { Container } from '@/shared/api';
import { Button, ConfirmDialog, ErrorMessage, Spinner } from '@/shared/ui';
import { containerQueries } from '../../api/queries';
import { lossText } from '../../model/losses';
import { t } from '@/shared/i18n';

/** Says what recreating `container` loses before Edit applies. */
export function EditConfirmation({
  hostId,
  container,
  onConfirm,
  onClose,
  onCommitFirst,
}: {
  hostId: string;
  container: Container;
  onConfirm: () => Promise<unknown>;
  onClose: () => void;
  onCommitFirst: () => void;
}) {
  const losses = useQuery(containerQueries.editLosses(hostId, container.id));
  return (
    <ConfirmDialog
      options={{
        title: t('containers.edit.confirmTitle', { name: container.name }),
        description: losses.error ? (
          <ErrorMessage error={losses.error} onRetry={() => void losses.refetch()} />
        ) : !losses.data ? (
          <Spinner />
        ) : losses.data.length === 0 ? (
          t('containers.edit.nothingLost')
        ) : (
          <div className="grid gap-2">
            <p>{t('containers.edit.keptBut')}</p>
            <ul className="list-disc space-y-1 pl-5">
              {losses.data.map((loss) => (
                <li key={JSON.stringify(loss)}>{lossText(loss)}</li>
              ))}
            </ul>
            {losses.data.some((l) => l.kind === 'filesystemChanges') && (
              <Button
                variant="outline"
                size="sm"
                className="justify-self-start"
                onClick={onCommitFirst}
              >
                {t('containers.edit.commitFirst')}
              </Button>
            )}
          </div>
        ),
        confirmLabel: t('containers.edit.recreate'),
        destructive: losses.data?.some((l) => l.kind !== 'downtime') ?? false,
        onConfirm,
      }}
      onClose={onClose}
    />
  );
}
