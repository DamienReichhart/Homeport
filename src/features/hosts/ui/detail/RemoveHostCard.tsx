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

/** Forgets a Host after confirmation; nothing on the Host itself changes. */
export function RemoveHostCard({ view }: { view: HostView }) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const remove = useDeleteHost(view.host.id);
  return (
    <Card className="border-destructive/40">
      <CardHeader>
        <CardTitle className="text-destructive">Remove host</CardTitle>
        <CardDescription>
          Homeport forgets this host, its credentials and its Managed Stack definitions. Nothing
          running on the host is touched.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          variant="destructive"
          onClick={() =>
            confirm({
              title: `Remove ${view.host.name}?`,
              description:
                'Deletes the stored connection, SSH key or certificates, and the compose definitions of its Managed Stacks. Containers, images and volumes on the host stay as they are.',
              confirmLabel: 'Remove host',
              destructive: true,
              onConfirm: async () => {
                await remove.mutateAsync();
                navigate('/hosts');
              },
            })
          }
        >
          <TrashIcon /> Remove host
        </Button>
      </CardContent>
      {dialog}
    </Card>
  );
}
