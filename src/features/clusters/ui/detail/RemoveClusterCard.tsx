import { TrashIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { ClusterView } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  useConfirm,
} from '@/shared/ui';
import { useRemoveCluster } from '../../api/mutations';
import { t } from '@/shared/i18n';

/** Forgets a Cluster after confirmation; nothing on the Cluster changes. */
export function RemoveClusterCard({ view }: { view: ClusterView }) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const remove = useRemoveCluster(view.cluster.id);
  return (
    <Card className="border-destructive/40">
      <CardHeader>
        <CardTitle className="text-destructive">{t('clusters.remove.title')}</CardTitle>
        <CardDescription>{t('clusters.remove.description')}</CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          variant="destructive"
          onClick={() =>
            confirm({
              title: t('clusters.remove.confirmTitle', { name: view.cluster.name }),
              description: t('clusters.remove.confirmDescription'),
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
      </CardContent>
      {dialog}
    </Card>
  );
}
