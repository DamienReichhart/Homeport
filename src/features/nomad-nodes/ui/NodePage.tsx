import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, ShieldOffIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { age, ClusterGate } from '@/core/cluster';
import { AllocationsTable, nomadKeys, useNomadWatch } from '@/core/nomad';
import type { ClusterView, NomadNodeDetail } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  Label,
  PageHeader,
  Spinner,
  Switch,
  useConfirmedAction,
} from '@/shared/ui';
import { useCancelDrain, useNodeEligibility } from '../api/mutations';
import { nomadNodeQueries } from '../api/queries';
import { DrainDialog } from './DrainDialog';
import { NodeStatus } from './NodesPage';
import { t } from '@/shared/i18n';
import { formatDateTime } from '@/shared/lib';

function NodeDetail({ cluster, detail }: { cluster: ClusterView; detail: NomadNodeDetail }) {
  const id = cluster.cluster.id;
  const node = detail.node;
  const navigate = useNavigate();
  const [draining, setDraining] = useState(false);
  const [dialog, confirm] = useConfirmedAction();
  const cancel = useCancelDrain(id, node.id);
  const eligibility = useNodeEligibility(id, node.id);
  const running = node.running ?? 0;
  return (
    <div className="grid gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-3">
            {node.name}
            <NodeStatus node={node} />
          </span>
        }
        description={t('nomad.node.description', {
          datacenter: node.datacenter,
          pool: node.nodePool ?? 'default',
        })}
        actions={
          <>
            <Button asChild variant="ghost">
              <Link to="/nomad/nodes">
                <ArrowLeftIcon /> {t('nomad.node.back')}
              </Link>
            </Button>
            <div className="flex items-center gap-2">
              <Switch
                id="node-eligible"
                checked={node.eligible}
                disabled={eligibility.isPending || detail.drain !== null}
                onCheckedChange={(v) => eligibility.mutate(v)}
              />
              <Label htmlFor="node-eligible">{t('nomad.node.eligible')}</Label>
            </div>
            {detail.drain ? (
              <Button
                variant="outline"
                onClick={() =>
                  confirm({
                    title: t('nomad.drain.cancelTitle', { node: node.name }),
                    description: t('nomad.drain.cancelDescription'),
                    confirmLabel: t('nomad.drain.cancel'),
                    action: () => cancel.mutateAsync(true),
                  })
                }
              >
                {t('nomad.drain.cancel')}
              </Button>
            ) : (
              <Button variant="outline" onClick={() => setDraining(true)}>
                <ShieldOffIcon /> {t('nomad.node.drain')}
              </Button>
            )}
          </>
        }
      />
      {detail.drain && (
        <Card>
          <CardHeader>
            <CardTitle>{t('nomad.drain.progressTitle')}</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-2 text-sm">
            <p>
              {running === 0
                ? t('nomad.drain.empty')
                : t('nomad.drain.remaining', { count: running })}
            </p>
            <KeyValue
              items={[
                [t('nomad.drain.started'), age(detail.drain.started)],
                [
                  t('nomad.drain.deadline'),
                  detail.drain.forced
                    ? t('nomad.drain.forced')
                    : detail.drain.deadline
                      ? formatDateTime(detail.drain.deadline)
                      : t('nomad.drain.none'),
                ],
                [
                  t('nomad.drain.systemJobs'),
                  detail.drain.ignoreSystemJobs
                    ? t('nomad.drain.systemJobsKept')
                    : t('nomad.drain.systemJobsMoved'),
                ],
              ]}
            />
          </CardContent>
        </Card>
      )}
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.node.settings')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [t('nomad.node.address'), node.address],
              [t('nomad.node.version'), node.version],
              [t('nomad.node.class'), node.nodeClass],
              [t('nomad.node.lastDrain'), detail.lastDrain],
              [
                t('nomad.node.drivers'),
                detail.drivers
                  .filter((d) => d.detected)
                  .map((d) => (d.healthy ? d.name : t('nomad.node.unhealthy', { driver: d.name })))
                  .join(', ') || null,
              ],
              ...detail.attributes.map(([k, v]): [string, string] => [k, v]),
            ]}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.node.allocations')}</CardTitle>
        </CardHeader>
        <CardContent>
          <AllocationsTable
            allocations={detail.allocations}
            onOpen={(a) => void navigate(`/nomad/allocations/${a.id}`)}
          />
        </CardContent>
      </Card>
      {draining && <DrainDialog clusterId={id} node={node} onClose={() => setDraining(false)} />}
      {dialog}
    </div>
  );
}

function NodeLoader({ cluster }: { cluster: ClusterView }) {
  const { nodeId = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(nomadNodeQueries.detail(id, nodeId));
  useNomadWatch(id, { topic: 'node', nodeId }, nomadKeys.node(id, nodeId));
  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />;
  return <NodeDetail cluster={cluster} detail={detail.data} />;
}

/** A Nomad client node: its drain, eligibility, drivers and allocations. */
export function NodePage() {
  return (
    <ClusterGate>
      {(cluster) => <NodeLoader key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
