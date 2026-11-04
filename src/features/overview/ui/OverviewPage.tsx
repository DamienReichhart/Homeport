import { PlusIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import { HostGate, useHosts, useHostSelection } from '@/core/host';
import { Button, PageHeader } from '@/shared/ui';
import { HostDashboard } from './dashboard/HostDashboard';
import { HostCard } from './hosts/HostCard';
import { t } from '@/shared/i18n';

export function OverviewPage() {
  const hosts = useHosts();
  const { hostId, host } = useHostSelection();
  return (
    <div className="grid gap-6">
      <PageHeader
        title={t('overview.title')}
        actions={
          <Button asChild variant="outline">
            <Link to="/hosts/new">
              <PlusIcon /> {t('overview.addHost')}
            </Link>
          </Button>
        }
      />
      {hosts.data && hosts.data.length > 0 && (
        <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {hosts.data.map((view) => (
            <HostCard key={view.host.id} view={view} selected={view.host.id === hostId} />
          ))}
        </div>
      )}
      <section className="grid gap-3">
        {host && <h2 className="text-lg font-semibold">{host.host.name}</h2>}
        <HostGate>{(id) => <HostDashboard hostId={id} />}</HostGate>
      </section>
    </div>
  );
}
