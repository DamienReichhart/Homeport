import { useParams } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { ErrorMessage, Spinner } from '@/shared/ui';
import { HostDetail } from './HostDetail';

export function HostDetailPage() {
  const { hostId = '' } = useParams();
  const hosts = useHosts();
  const view = hosts.data?.find((h) => h.host.id === hostId);
  if (hosts.isPending) return <Spinner />;
  if (!view)
    return <ErrorMessage error={{ code: 'not_found', message: 'This host no longer exists.' }} />;
  // Keyed so the form starts from this host's saved settings.
  return <HostDetail key={hostId} view={view} />;
}
