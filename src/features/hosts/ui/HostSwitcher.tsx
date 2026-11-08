import { toAppError } from '@/shared/api';
import { PlusIcon, ServerCogIcon, ShipWheelIcon, TriangleAlertIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import {
  Button,
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
  showErrorDetails,
  StatusDot,
} from '@/shared/ui';
import { clusterStatusError, describeClusterStatus, useClusters } from '@/core/cluster';
import { describeStatus, statusError, useHosts } from '@/core/host';
import { useSelection } from '@/core/selection';
import { statusTitle, summaryTitle } from '../model/statusTitle';
import { t } from '@/shared/i18n';

const ADD = '__add__';
const MANAGE = '__manage__';
const ADD_CLUSTER = '__add_cluster__';
const MANAGE_CLUSTERS = '__manage_clusters__';
/** Clusters' values; a Host's value is its id. */
const CLUSTER = 'cluster:';

/**
 * The sidebar's picker of what the pages show: a Host, or a Cluster once
 * there are some.
 */
export function HostSwitcher() {
  const hosts = useHosts();
  const clusters = useClusters();
  const { selection, select } = useSelection();
  const navigate = useNavigate();

  const value = !selection
    ? ''
    : selection.kind === 'host'
      ? selection.id
      : `${CLUSTER}${selection.id}`;
  const selectedHost =
    selection?.kind === 'host' ? hosts.data?.find((h) => h.host.id === selection.id) : undefined;
  const selectedCluster =
    selection?.kind === 'cluster'
      ? clusters.data?.find((c) => c.cluster.id === selection.id)
      : undefined;
  const unusable = selectedHost
    ? {
        name: selectedHost.host.name,
        label: describeStatus(selectedHost.status).label,
        error: statusError(selectedHost.status),
      }
    : selectedCluster
      ? {
          name: selectedCluster.cluster.name,
          label: describeClusterStatus(selectedCluster.status).label,
          error: clusterStatusError(selectedCluster.status),
        }
      : null;
  return (
    <div className="flex items-center gap-1">
      <Select
        value={value}
        onValueChange={(next) => {
          if (next === ADD) navigate('/hosts/new');
          else if (next === MANAGE) navigate('/hosts');
          else if (next === ADD_CLUSTER) navigate('/clusters/new');
          else if (next === MANAGE_CLUSTERS) navigate('/clusters');
          else if (next.startsWith(CLUSTER))
            select({ kind: 'cluster', id: next.slice(CLUSTER.length) });
          else select({ kind: 'host', id: next });
        }}
      >
        <SelectTrigger className="w-full" aria-label={t('hosts.switcher.label')}>
          <SelectValue placeholder={t('hosts.switcher.placeholder')} />
        </SelectTrigger>
        <SelectContent>
          {hosts.data?.length ? (
            <SelectGroup>
              <SelectLabel>{t('hosts.switcher.hosts')}</SelectLabel>
              {hosts.data.map(({ host, status }) => (
                <SelectItem key={host.id} value={host.id} title={statusTitle(status)}>
                  <StatusDot tone={describeStatus(status).tone} />
                  <span className="truncate">{host.name}</span>
                </SelectItem>
              ))}
            </SelectGroup>
          ) : null}
          {clusters.data?.length ? (
            <SelectGroup>
              <SelectLabel>{t('hosts.switcher.clusters')}</SelectLabel>
              {clusters.data.map(({ cluster, status }) => (
                <SelectItem
                  key={cluster.id}
                  value={`${CLUSTER}${cluster.id}`}
                  title={summaryTitle(describeClusterStatus(status))}
                >
                  <StatusDot tone={describeClusterStatus(status).tone} />
                  <span className="truncate">{cluster.name}</span>
                </SelectItem>
              ))}
            </SelectGroup>
          ) : null}
          {hosts.data?.length || clusters.data?.length ? <SelectSeparator /> : null}
          <SelectItem value={ADD}>
            <PlusIcon /> {t('hosts.switcher.add')}
          </SelectItem>
          <SelectItem value={MANAGE}>
            <ServerCogIcon /> {t('hosts.switcher.manage')}
          </SelectItem>
          <SelectSeparator />
          <SelectItem value={ADD_CLUSTER}>
            <PlusIcon /> {t('hosts.switcher.addCluster')}
          </SelectItem>
          {clusters.data?.length ? (
            <SelectItem value={MANAGE_CLUSTERS}>
              <ShipWheelIcon /> {t('hosts.switcher.manageClusters')}
            </SelectItem>
          ) : null}
        </SelectContent>
      </Select>
      {unusable?.error && (
        <Button
          variant="ghost"
          size="icon"
          className="text-destructive shrink-0"
          aria-label={t('hosts.switcher.whyUnusable', { name: unusable.name })}
          title={t('hosts.statusTitle', { label: unusable.label, detail: unusable.error.message })}
          onClick={() => unusable.error && showErrorDetails(toAppError(unusable.error))}
        >
          <TriangleAlertIcon />
        </Button>
      )}
    </div>
  );
}
