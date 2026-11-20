import { TrashIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { HostView } from '@/shared/api';
import { Button, DangerZone, useConfirm } from '@/shared/ui';
import { useDeleteHost } from '../../api/mutations';
import { t } from '@/shared/i18n';

/**
 * Forgets a Host, at the foot of its page. Its Managed Stacks' definitions go
 * with it for good, so the name is typed first; nothing on the Host changes.
 */
export function RemoveHostCard({ view }: { view: HostView }) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const remove = useDeleteHost(view.host.id);
  return (
    <>
      <DangerZone
        title={t('hosts.remove.title')}
        description={t('hosts.remove.description')}
        action={
          <Button
            variant="destructive"
            onClick={() =>
              confirm({
                title: t('hosts.remove.confirmTitle', { name: view.host.name }),
                description: t('hosts.remove.confirmDescription'),
                confirmLabel: t('hosts.remove.title'),
                destructive: true,
                typeToConfirm: view.host.name,
                onConfirm: async () => {
                  await remove.mutateAsync();
                  navigate('/hosts');
                },
              })
            }
          >
            <TrashIcon /> {t('hosts.remove.title')}
          </Button>
        }
      />
      {dialog}
    </>
  );
}
