import { TrashIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { HostView } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  useConfirm,
} from '@/shared/ui';
import { useDeleteHost } from '../../api/mutations';
import { t } from '@/shared/i18n';

/** Forgets a Host after confirmation; nothing on the Host itself changes. */
export function RemoveHostCard({ view }: { view: HostView }) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const remove = useDeleteHost(view.host.id);
  return (
    <Card className="border-destructive/40">
      <CardHeader>
        <CardTitle className="text-destructive">{t('hosts.remove.title')}</CardTitle>
        <CardDescription>{t('hosts.remove.description')}</CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          variant="destructive"
          onClick={() =>
            confirm({
              title: t('hosts.remove.confirmTitle', { name: view.host.name }),
              description: t('hosts.remove.confirmDescription'),
              confirmLabel: t('hosts.remove.title'),
              destructive: true,
              onConfirm: async () => {
                await remove.mutateAsync();
                navigate('/hosts');
              },
            })
          }
        >
          <TrashIcon /> {t('hosts.remove.title')}
        </Button>
      </CardContent>
      {dialog}
    </Card>
  );
}
