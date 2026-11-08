import { useState, type ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import type { ServiceView } from '@/shared/api';
import { useConfirmedAction } from '@/shared/ui';
import { useRemoveService, useRollbackService } from '../../api/mutations';
import type { ServiceActions } from './ServiceMenu';
import { ScaleDialog } from './ScaleDialog';
import { t } from '@/shared/i18n';

/** The actions of a Swarm's services, with the dialogs they open. */
export function useServiceActions(
  swarmId: string,
  onRemoved?: () => void,
): [ReactNode, ServiceActions] {
  const navigate = useNavigate();
  const [confirmDialog, confirm] = useConfirmedAction();
  const [scaling, setScaling] = useState<ServiceView | null>(null);
  const rollback = useRollbackService(swarmId);
  const remove = useRemoveService(swarmId);
  const actions: ServiceActions = {
    onLogs: (s) => void navigate(`/swarms/${swarmId}/services/${s.id}/logs`),
    onScale: setScaling,
    onRollback: (s) =>
      confirm({
        title: t('swarm.services.rollback.title', { name: s.name }),
        description: t('swarm.services.rollback.description'),
        confirmLabel: t('swarm.services.action.rollback'),
        action: () => rollback.mutateAsync(s),
      }),
    onRemove: (s) =>
      confirm({
        title: t('swarm.services.remove.title', { name: s.name }),
        description: t('swarm.services.remove.description', { count: s.running }),
        confirmLabel: t('common.remove'),
        destructive: true,
        action: () => remove.mutateAsync(s).then(() => onRemoved?.()),
      }),
  };
  const dialogs = (
    <>
      {confirmDialog}
      {scaling && (
        <ScaleDialog swarmId={swarmId} service={scaling} onClose={() => setScaling(null)} />
      )}
    </>
  );
  return [dialogs, actions];
}
