import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, Trash2Icon } from 'lucide-react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { IncusGate } from '@/core/host';
import {
  Badge,
  Button,
  DangerZone,
  DataTable,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import type { IncusLease } from '@/shared/api';
import { useNetworks } from '../api/mutations';
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
  const { remove } = useNetworks(hostId, project);
  const [dialog, confirm] = useConfirmedAction();
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
  const inUse = network.usedBy.length > 0;
  return (
    <div className="grid content-start gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon className="rtl:-scale-x-100" />
            </Button>
            {network.name}
            <Badge variant="outline">{network.type}</Badge>
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
          [
            t('incus.networks.field.addresses'),
            <span key="addresses" className="font-mono text-xs">
              {state.data?.addresses.join(', ') || '—'}
            </span>,
          ],
          [
            t('incus.networks.field.ipv4'),
            <span key="ipv4" className="font-mono text-xs">
              {network.ipv4 ?? '—'}
            </span>,
          ],
          [
            t('incus.networks.field.ipv6'),
            <span key="ipv6" className="font-mono text-xs">
              {network.ipv6 ?? '—'}
            </span>,
          ],
          [
            t('incus.networks.field.nat'),
            network.nat ? t('incus.networks.yes') : t('incus.networks.no'),
          ],
          [t('incus.networks.field.mtu'), state.data?.mtu ?? '—'],
          [
            t('incus.networks.field.hwaddr'),
            <span key="hwaddr" className="font-mono text-xs">
              {state.data?.hwaddr || '—'}
            </span>,
          ],
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
      {network.managed && (
        <DangerZone
          title={t('incus.networks.delete.zone')}
          description={
            inUse
              ? t('incus.networks.inUse', { names: network.usedBy.map((u) => u.name).join(', ') })
              : t('incus.networks.delete.description')
          }
          action={
            <Button
              variant="destructive"
              disabled={inUse || remove.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.networks.delete.title', { name: network.name }),
                  description: t('incus.networks.delete.description'),
                  confirmLabel: t('incus.delete'),
                  destructive: true,
                  action: () => remove.mutateAsync(network.name).then(back),
                })
              }
            >
              <Trash2Icon /> {t('incus.delete')}
            </Button>
          }
        />
      )}
      {dialog}
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
