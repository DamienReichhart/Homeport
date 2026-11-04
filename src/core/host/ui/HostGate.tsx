import { RefreshCwIcon, ServerOffIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { Button, EmptyState, ErrorMessage, LogoMark, Spinner } from '@/shared/ui';
import { useCreateHost, useReconnectHost } from '../api/mutations';
import { useHosts } from '../api/queries';
import { describeStatus, statusError } from '../model/status';
import { useHostSelection } from '../selection/context';
import { LocalEngines } from './LocalEngines';
import { t } from '@/shared/i18n';

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
        title={t('core.gate.welcome.title')}
        description={t('core.gate.welcome.description')}
        action={
          <div className="grid max-w-xl justify-items-center gap-4">
            <LocalEngines onAdd={(input) => add.mutate(input)} pending={add.isPending} />
            <Button asChild>
              <Link to="/hosts/new">{t('core.gate.addAnother')}</Link>
            </Button>
          </div>
        }
      />
    );
  }
  if (host.status.state === 'connected') return <>{children(host.host.id)}</>;
  if (host.status.state === 'connecting')
    return <Spinner label={t('core.gate.connecting', { name: host.host.name })} />;

  const { label, detail } = describeStatus(host.status);
  const error = statusError(host.status);
  const retry = () => reconnect.mutate(host.host.id);
  return (
    <EmptyState
      icon={ServerOffIcon}
      title={t('core.gate.statusTitle', { name: host.host.name, status: label })}
      description={
        error ? (
          <div className="grid gap-2">
            <ErrorMessage error={error} onRetry={retry} />
            {host.status.state === 'unreachable' && (
              <p>{t('core.gate.retryingOnItsOwn', { secs: host.status.retryInSecs })}</p>
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
              <RefreshCwIcon /> {t('core.gate.retryNow')}
            </Button>
          )}
          <Button asChild>
            <Link to={`/hosts/${host.host.id}`}>{t('core.gate.openSettings')}</Link>
          </Button>
        </div>
      }
    />
  );
}
