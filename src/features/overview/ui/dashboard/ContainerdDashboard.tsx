import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { containerdQueries, useContainerdNamespace } from '@/core/host';
import type { HostView } from '@/shared/api';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  Spinner,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A containerd Host's version, platform and namespaces, with what each holds. */
export function ContainerdDashboard({ host }: { host: HostView }) {
  const hostId = host.host.id;
  const namespaces = useQuery(containerdQueries.namespaces(hostId));
  const [shown, choose] = useContainerdNamespace(hostId);
  const status = host.status.state === 'connected' ? host.status : null;
  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.containerd.title')}</CardTitle>
        </CardHeader>
        <CardContent>
          {status && (
            <KeyValue
              items={[
                [
                  t('overview.containerd.version'),
                  status.containerdVersion ?? status.engineVersion,
                ],
                [
                  t('overview.containerd.revision'),
                  <span key="rev" className="font-mono text-xs">
                    {status.apiVersion || '—'}
                  </span>,
                ],
                [t('overview.containerd.platform'), `${status.os}/${status.arch}`],
              ]}
            />
          )}
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.containerd.namespaces')}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-2">
          {namespaces.isPending ? (
            <Spinner />
          ) : namespaces.error ? (
            <ErrorMessage error={namespaces.error} onRetry={() => void namespaces.refetch()} />
          ) : (
            namespaces.data.map((ns) => (
              <div key={ns.name} className="flex items-center justify-between gap-2 text-sm">
                <button
                  type="button"
                  className="flex items-center gap-2 font-medium hover:underline"
                  onClick={() => choose(ns.name)}
                >
                  {ns.name}
                  {ns.name === shown && (
                    <Badge variant="secondary">{t('overview.containerd.shown')}</Badge>
                  )}
                </button>
                <span className="text-muted-foreground">
                  {t('overview.containerd.holds', { containers: ns.containers, images: ns.images })}
                </span>
              </div>
            ))
          )}
          <div className="flex flex-wrap gap-2 pt-2">
            <Button asChild size="sm" variant="outline">
              <Link to="/containerd/containers">{t('overview.containerd.containers')}</Link>
            </Button>
            <Button asChild size="sm" variant="outline">
              <Link to="/containerd/images">{t('overview.containerd.images')}</Link>
            </Button>
            <Button asChild size="sm" variant="outline">
              <Link to="/containerd/snapshots">{t('overview.containerd.snapshots')}</Link>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
