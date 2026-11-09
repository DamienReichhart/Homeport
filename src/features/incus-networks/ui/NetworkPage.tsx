import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { IncusGate } from '@/core/host';
import {
  Badge,
  Button,
  DataTable,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import type { IncusLease } from '@/shared/api';
import { incusNetworkQueries } from '../api/queries';
import { t } from '@/shared/i18n';

const leaseColumns = (): ColumnDef<IncusLease, unknown>[] => [
  { id: 'hostname', header: t('incus.networks.lease.hostname'), accessorFn: (l) => l.hostname },
  {
    id: 'address',
    header: t('incus.networks.lease.address'),
    accessorFn: (l) => l.address,
    cell: ({ row }) => <span className="font-mono text-xs">{row.original.address}</span>,
  },
  {
    id: 'hwaddr',
    header: t('incus.networks.lease.hwaddr'),
    accessorFn: (l) => l.hwaddr,
    cell: ({ row }) => <span className="font-mono text-xs">{row.original.hwaddr || '—'}</span>,
  },
  {
    id: 'type',
    header: t('incus.networks.lease.type'),
    accessorFn: (l) => l.type,
    cell: ({ row }) => <Badge variant="outline">{row.original.type}</Badge>,
  },
];

function NetworkDetail({
  hostId,
  project,
  name,
}: {
  hostId: string;
  project: string;
  name: string;
}) {
  const navigate = useNavigate();
  const back = () => void navigate('/incus/networks');
  const networks = useQuery(incusNetworkQueries.list(hostId, project));
  const state = useQuery(incusNetworkQueries.state(hostId, project, name));
  const network = networks.data?.find((n) => n.name === name);
  if (networks.isPending) return <Spinner />;
  if (networks.error)
    return <ErrorMessage error={networks.error} onRetry={() => void networks.refetch()} />;
  if (!network)
    return (
      <div className="grid gap-3">
        <p className="text-muted-foreground text-sm">{t('incus.networks.gone', { name })}</p>
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('incus.networks.backToList')}
        </Button>
      </div>
    );
  return (
    <div className="grid content-start gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon className="rtl:-scale-x-100" />
            </Button>
            {network.name}
            <Badge variant="secondary">{network.type}</Badge>
          </span>
        }
        description={network.description || undefined}
      />
      <KeyValue
        items={[
          [
            t('incus.networks.field.managed'),
            network.managed ? t('incus.networks.managed') : t('incus.networks.unmanaged'),
          ],
          [t('incus.networks.field.state'), state.data?.state ?? '—'],
          [t('incus.networks.field.addresses'), state.data?.addresses.join(', ') || '—'],
          [t('incus.networks.field.ipv4'), network.ipv4 ?? '—'],
          [t('incus.networks.field.ipv6'), network.ipv6 ?? '—'],
          [
            t('incus.networks.field.nat'),
            network.nat ? t('incus.networks.yes') : t('incus.networks.no'),
          ],
          [t('incus.networks.field.mtu'), state.data?.mtu ?? '—'],
          [t('incus.networks.field.hwaddr'), state.data?.hwaddr || '—'],
        ]}
      />
      <div>
        <h3 className="mb-2 text-sm font-medium">{t('incus.networks.usedBy')}</h3>
        <div className="flex flex-wrap gap-1.5">
          {network.usedBy.length === 0 && <span className="text-muted-foreground text-sm">—</span>}
          {network.usedBy.map((user) =>
            user.kind === 'instance' ? (
              <Badge key={`${user.kind}/${user.name}`} variant="outline" asChild>
                <Link to={`/incus/instances/${encodeURIComponent(user.name)}`}>{user.name}</Link>
              </Badge>
            ) : (
              <Badge key={`${user.kind}/${user.name}`} variant="outline">
                {user.kind} {user.name}
              </Badge>
            ),
          )}
        </div>
      </div>
      {network.managed && (
        <div>
          <h3 className="mb-2 text-sm font-medium">{t('incus.networks.leases')}</h3>
          {state.error ? (
            <ErrorMessage error={state.error} onRetry={() => void state.refetch()} />
          ) : state.isPending ? (
            <Spinner />
          ) : state.data.leases.length === 0 ? (
            <p className="text-muted-foreground text-sm">{t('incus.networks.noLeases')}</p>
          ) : (
            <DataTable
              columns={leaseColumns()}
              data={state.data.leases}
              getRowId={(l) => `${l.address}/${l.hwaddr}`}
            />
          )}
        </div>
      )}
    </div>
  );
}

export function NetworkPage() {
  const { name = '' } = useParams();
  return (
    <IncusGate>
      {(hostId, project) => (
        <NetworkDetail
          key={`${hostId}/${project}/${name}`}
          hostId={hostId}
          project={project}
          name={decodeURIComponent(name)}
        />
      )}
    </IncusGate>
  );
}
