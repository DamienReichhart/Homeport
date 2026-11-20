import { TrashIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { ClusterView } from '@/shared/api';
import { Button, DangerZone, useConfirm } from '@/shared/ui';
import { useRemoveCluster } from '../../api/mutations';
import { t } from '@/shared/i18n';

/**
 * Forgets a Cluster, at the foot of its page. Nothing on the Cluster changes
 * and it can be added again, so one confirmation is enough.
 */
export function RemoveClusterCard({ view }: { view: ClusterView }) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const remove = useRemoveCluster(view.cluster.id);
  const nomad = view.cluster.kind === 'nomad';
  return (
    <>
      <DangerZone
        title={t('clusters.remove.title')}
        description={t(nomad ? 'clusters.remove.descriptionNomad' : 'clusters.remove.description')}
        action={
          <Button
            variant="destructive"
            onClick={() =>
              confirm({
                title: t('clusters.remove.confirmTitle', { name: view.cluster.name }),
                description: t(
                  nomad
                    ? 'clusters.remove.confirmDescriptionNomad'
                    : 'clusters.remove.confirmDescription',
                ),
                confirmLabel: t('clusters.remove.title'),
                destructive: true,
                onConfirm: async () => {
                  await remove.mutateAsync();
                  navigate('/clusters');
                },
              })
            }
          >
            <TrashIcon /> {t('clusters.remove.title')}
          </Button>
        }
      />
      {dialog}
    </>
  );
}
