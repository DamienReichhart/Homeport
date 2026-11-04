import { useContainers } from '@/core/host';
import type { Container } from '@/shared/api';
import { Alert, AlertDescription } from '@/shared/ui';
import { containerQueries } from '../../api/queries';
import { copyName, sharedHostPorts } from '../../model/duplicate';
import { fromRunSpec } from '../../model/fromRunSpec';
import { LoadingDialog } from './LoadingDialog';
import { RunContainerDialog } from './RunContainerDialog';
import { t } from '@/shared/i18n';

/** The Run dialog filled in from a container, to run a copy of it. */
export function DuplicateDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container;
  onClose: () => void;
}) {
  const taken = (useContainers(hostId).data ?? []).map((c) => c.name);
  const title = t('containers.duplicate.title', { name: container.name });
  return (
    <LoadingDialog
      title={title}
      query={containerQueries.runSpec(hostId, container.id)}
      onClose={onClose}
    >
      {(spec) => {
        const original = fromRunSpec(spec);
        return (
          <RunContainerDialog
            hostId={hostId}
            open
            onOpenChange={(open) => !open && onClose()}
            title={title}
            initial={{ ...original, name: copyName(container.name, taken) }}
            notice={(form) => {
              const shared = sharedHostPorts(form, original);
              if (shared.length === 0) return null;
              return (
                <Alert>
                  <AlertDescription>
                    {t('containers.duplicate.sharedPorts', {
                      name: container.name,
                      count: shared.length,
                      ports: shared.join(', '),
                    })}{' '}
                    {t('containers.duplicate.sharedPortsFix', { count: shared.length })}
                  </AlertDescription>
                </Alert>
              );
            }}
          />
        );
      }}
    </LoadingDialog>
  );
}
