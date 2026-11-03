import { RefreshCwIcon, ServerOffIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { Button, EmptyState, ErrorMessage, LogoMark, Spinner } from '@/shared/ui';
import { useCreateHost, useReconnectHost } from '../api/mutations';
import { useHosts } from '../api/queries';
import { describeStatus, statusError } from '../model/status';
import { useHostSelection } from '../selection/context';
import { LocalEngines } from './LocalEngines';

/**
 * Renders `children` with the selected Host's id once it is connected, and
 * explains what to do otherwise.
 */
export function HostGate({ children }: { children: (hostId: string) => ReactNode }) {
  const hosts = useHosts();
  const { host, select } = useHostSelection();
  const reconnect = useReconnectHost();
  const add = useCreateHost((view) => select(view.host.id));

  if (hosts.isPending) return <Spinner />;
  if (!host) {
    return (
      <EmptyState
        media={<LogoMark className="mb-2 size-14" />}
        title="Welcome to Homeport"
        description="Bring in a Docker Engine to get started — this machine's, or a remote one over SSH or TLS."
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
  const error = statusError(host.status);
  const retry = () => reconnect.mutate(host.host.id);
  return (
    <EmptyState
      icon={ServerOffIcon}
      title={`${host.host.name}: ${label}`}
      description={
        error ? (
          <div className="grid gap-2">
            <ErrorMessage error={error} onRetry={retry} />
            {host.status.state === 'unreachable' && (
              <p>Homeport tries again on its own in {host.status.retryInSecs}s.</p>
            )}
          </div>
        ) : (
          detail
        )
      }
      action={
        <div className="flex gap-2">
          {!error && host.status.state !== 'disabled' && (
            <Button variant="outline" disabled={reconnect.isPending} onClick={retry}>
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
