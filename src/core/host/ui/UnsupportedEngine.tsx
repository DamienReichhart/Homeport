import { CircleSlashIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import type { HostCapability } from '@/core/feature';
import type { HostView } from '@/shared/api';
import { Button, EmptyState } from '@/shared/ui';
import { ENGINE_LABELS } from '../model/engine';
import { t } from '@/shared/i18n';

/** Says why the selected Host can't show a page that needs `requires`. */
export function UnsupportedEngine({
  host,
  requires,
}: {
  host: HostView;
  requires: HostCapability;
}) {
  const name = host.host.name;
  const description =
    requires === 'podman'
      ? t('core.unsupported.podman', { name })
      : t('core.unsupported.docker', { name, engine: t(ENGINE_LABELS[host.host.engine]) });
  return (
    <EmptyState
      icon={CircleSlashIcon}
      title={t('core.unsupported.title', { name })}
      description={description}
      action={
        <div className="flex gap-2">
          <Button asChild variant="outline">
            <Link to="/">{t('core.unsupported.overview')}</Link>
          </Button>
          <Button asChild>
            <Link to={`/hosts/${host.host.id}`}>{t('core.gate.openSettings')}</Link>
          </Button>
        </div>
      }
    />
  );
}
