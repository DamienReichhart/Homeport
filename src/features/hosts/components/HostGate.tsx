import { useMutation, useQueryClient } from '@tanstack/react-query';
import { RefreshCwIcon, ServerIcon, ServerOffIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { commands, keys, type HostInput } from '@/shared/api';
import { Button } from '@/shared/ui/button';
import { EmptyState, Spinner } from '@/shared/ui';
import { useHosts } from '../api';
import { useHostSelection } from '../context';
import { describeStatus } from '../status';
import { LocalEngines } from './LocalEngines';

/**
 * Renders `children` with the selected Host's id once it is connected, and
 * explains what to do otherwise.
 */
export function HostGate({ children }: { children: (hostId: string) => ReactNode }) {
  const hosts = useHosts();
  const { host, select } = useHostSelection();
  const client = useQueryClient();
  const reconnect = useMutation({ mutationFn: (id: string) => commands.hostsReconnect(id) });
  const add = useMutation({
    mutationFn: (input: HostInput) => commands.hostsCreate(input),
    onSuccess: async (view) => {
      await client.invalidateQueries({ queryKey: keys.hosts });
      select(view.host.id);
    },
  });

  if (hosts.isPending) return <Spinner />;
  if (!host) {
    return (
      <EmptyState
        icon={ServerIcon}
        title="No host yet"
        description="Add a Docker Engine — this machine's, or a remote one over SSH or TLS."
        action={
          <div className="grid max-w-xl justify-items-center gap-4">
            <LocalEngines onAdd={(input) => add.mutate(input)} pending={add.isPending} />
            <Button asChild>
              <Link to="/hosts/new">Add another host</Link>
            </Button>
          </div>
        }
      />
    );
  }
  if (host.status.state === 'connected') return <>{children(host.host.id)}</>;
  if (host.status.state === 'connecting')
    return <Spinner label={`Connecting to ${host.host.name}…`} />;

  const { label, detail } = describeStatus(host.status);
  return (
    <EmptyState
      icon={ServerOffIcon}
      title={`${host.host.name}: ${label}`}
      description={detail}
      action={
        <div className="flex gap-2">
          {host.status.state !== 'disabled' && (
            <Button
              variant="outline"
              disabled={reconnect.isPending}
              onClick={() => reconnect.mutate(host.host.id)}
            >
              <RefreshCwIcon /> Retry now
            </Button>
          )}
          <Button asChild>
            <Link to={`/hosts/${host.host.id}`}>Open host settings</Link>
          </Button>
        </div>
      }
    />
  );
}
