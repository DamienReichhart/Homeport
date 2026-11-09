import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { incusQueries, useIncusProject } from '@/core/host';
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

/** An Incus Host's version and platform, and its projects with what each holds. */
export function IncusDashboard({ host }: { host: HostView }) {
  const hostId = host.host.id;
  const projects = useQuery(incusQueries.projects(hostId));
  const [shown, choose] = useIncusProject(hostId);
  const status = host.status.state === 'connected' ? host.status : null;
  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.incus.title')}</CardTitle>
        </CardHeader>
        <CardContent>
          {status && (
            <KeyValue
              items={[
                [t('overview.incus.version'), status.incusVersion ?? status.engineVersion],
                [t('overview.incus.os'), status.os || '—'],
                [t('overview.incus.arch'), status.arch || '—'],
              ]}
            />
          )}
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.incus.projects')}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-2">
          {projects.isPending ? (
            <Spinner />
          ) : projects.error ? (
            <ErrorMessage error={projects.error} onRetry={() => void projects.refetch()} />
          ) : (
            projects.data.map((p) => (
              <div key={p.name} className="flex items-center justify-between gap-2 text-sm">
                <button
                  type="button"
                  className="flex items-center gap-2 font-medium hover:underline"
                  onClick={() => choose(p.name)}
                >
                  {p.name}
                  {p.name === shown && (
                    <Badge variant="secondary">{t('overview.incus.shown')}</Badge>
                  )}
                </button>
                <span className="text-muted-foreground">
                  {t('overview.incus.holds', { instances: p.instances, images: p.images })}
                </span>
              </div>
            ))
          )}
          <div className="flex flex-wrap gap-2 pt-2">
            <Button asChild size="sm" variant="outline">
              <Link to="/incus/instances">{t('overview.incus.instances')}</Link>
            </Button>
            <Button asChild size="sm" variant="outline">
              <Link to="/incus/images">{t('overview.incus.images')}</Link>
            </Button>
            <Button asChild size="sm" variant="outline">
              <Link to="/incus/storage">{t('overview.incus.storage')}</Link>
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
