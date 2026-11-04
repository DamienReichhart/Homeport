import { useQuery } from '@tanstack/react-query';
import type { Container } from '@/shared/api';
import { ConfirmDialog, ErrorMessage, Spinner } from '@/shared/ui';
import { containerQueries } from '../../api/queries';
import { lossText } from '../../model/losses';

/** Says what recreating `container` loses before Edit applies. */
export function EditConfirmation({
  hostId,
  container,
  onConfirm,
  onClose,
}: {
  hostId: string;
  container: Container;
  onConfirm: () => Promise<unknown>;
  onClose: () => void;
}) {
  const losses = useQuery(containerQueries.editLosses(hostId, container.id));
  return (
    <ConfirmDialog
      options={{
        title: `Recreate ${container.name}?`,
        description: losses.error ? (
          <ErrorMessage error={losses.error} onRetry={() => void losses.refetch()} />
        ) : !losses.data ? (
          <Spinner />
        ) : losses.data.length === 0 ? (
          'Nothing else is lost: its name, networks and named volumes are kept.'
        ) : (
          <div className="grid gap-2">
            <p>Its name, networks and named volumes are kept, but not:</p>
            <ul className="list-disc space-y-1 pl-5">
              {losses.data.map((loss) => (
                <li key={JSON.stringify(loss)}>{lossText(loss)}</li>
              ))}
            </ul>
          </div>
        ),
        confirmLabel: 'Recreate',
        destructive: losses.data?.some((l) => l.kind !== 'downtime') ?? false,
        onConfirm,
      }}
      onClose={onClose}
    />
  );
}
