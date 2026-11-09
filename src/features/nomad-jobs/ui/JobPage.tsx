import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, SquareIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import { age, ClusterGate } from '@/core/cluster';
import { AllocationsTable, jobTone, NomadStatus, nomadKeys, useNomadWatch } from '@/core/nomad';
import type { ClusterView, NomadJobDetail, NomadTaskGroupView } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  DataTable,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import type { JobRef } from '../api/mutations';
import { nomadJobQueries } from '../api/queries';
import { shortOf } from '../model/jobs';
import { ScaleGroupDialog } from './ScaleGroupDialog';
import { StopJobDialog } from './StopJobDialog';
import { t } from '@/shared/i18n';

function groupColumns(
  onScale: (group: NomadTaskGroupView) => void,
  scalable: boolean,
): ColumnDef<NomadTaskGroupView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('nomad.groups.column.name'),
      accessorFn: (g) => g.counts.name,
      cell: ({ row }) => <span className="font-medium">{row.original.counts.name}</span>,
    },
    {
      id: 'running',
      header: t('nomad.groups.column.running'),
      accessorFn: (g) => g.counts.running,
      cell: ({ row }) => {
        const c = row.original.counts;
        return (
          <span className={shortOf(c) ? 'text-warning tabular-nums' : 'tabular-nums'}>
            {c.desired === null
              ? c.running
              : t('nomad.groups.ofDesired', { running: c.running, desired: c.desired })}
          </span>
        );
      },
    },
    {
      id: 'others',
      header: t('nomad.groups.column.others'),
      accessorFn: (g) => g.counts.starting + g.counts.queued + g.counts.failed + g.counts.lost,
      cell: ({ row }) => {
        const c = row.original.counts;
        return t('nomad.groups.others', {
          starting: c.starting,
          queued: c.queued,
          failed: c.failed,
          lost: c.lost,
        });
      },
    },
    {
      id: 'tasks',
      header: t('nomad.groups.column.tasks'),
      accessorFn: (g) => g.tasks.map((task) => `${task.name} (${task.driver})`).join(', '),
    },
    {
      id: 'scaling',
      header: t('nomad.groups.column.scaling'),
      accessorFn: (g) => g.scaling?.max ?? 0,
      cell: ({ row }) => {
        const s = row.original.scaling;
        return s ? t('nomad.groups.bounds', { min: s.min ?? 0, max: s.max ?? '∞' }) : '—';
      },
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) =>
        scalable && (
          <Button
            size="sm"
            variant="outline"
            onClick={(e) => {
              e.stopPropagation();
              onScale(row.original);
            }}
          >
            {t('nomad.groups.scale')}
          </Button>
        ),
    },
  ];
}

function JobDetail({
  cluster,
  job,
  detail,
}: {
  cluster: ClusterView;
  job: JobRef;
  detail: NomadJobDetail;
}) {
  const [stopping, setStopping] = useState(false);
  const [scaling, setScaling] = useState<NomadTaskGroupView | null>(null);
  const view = detail.job;
  // System jobs run one allocation per eligible node: they have no count to scale.
  const scalable = !view.stopped && (view.jobType === 'service' || view.jobType === 'batch');
  return (
    <div className="grid gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-3">
            {view.name}
            {view.stopped ? (
              <NomadStatus status="stopped" tone="muted" />
            ) : (
              <NomadStatus status={view.status} tone={jobTone(view.status, view.stopped)} />
            )}
          </span>
        }
        description={t('nomad.job.description', {
          type: view.jobType,
          namespace: view.namespace,
          version: detail.version,
        })}
        actions={
          <>
            <Button asChild variant="ghost">
              <Link to="/nomad/jobs">
                <ArrowLeftIcon /> {t('nomad.job.back')}
              </Link>
            </Button>
            {!view.stopped && (
              <Button variant="outline" onClick={() => setStopping(true)}>
                <SquareIcon /> {t('nomad.job.stop')}
              </Button>
            )}
          </>
        }
      />
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.job.settings')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [t('nomad.job.type'), view.jobType],
              [t('nomad.job.priority'), view.priority],
              [t('nomad.job.datacenters'), detail.datacenters.join(', ') || null],
              [t('nomad.job.nodePool'), detail.nodePool],
              [t('nomad.job.submitted'), age(view.submitted)],
              [t('nomad.job.parent'), view.parentId],
              [t('nomad.job.statusDescription'), view.statusDescription],
            ]}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.job.groups')}</CardTitle>
        </CardHeader>
        <CardContent>
          <DataTable
            columns={groupColumns(setScaling, scalable)}
            data={detail.groups}
            getRowId={(g) => g.counts.name}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('nomad.job.allocations')}</CardTitle>
        </CardHeader>
        <CardContent>
          <AllocationsTable allocations={detail.allocations} />
        </CardContent>
      </Card>
      {stopping && (
        <StopJobDialog
          job={job}
          clusterName={cluster.cluster.name}
          onClose={() => setStopping(false)}
        />
      )}
      {scaling && <ScaleGroupDialog job={job} group={scaling} onClose={() => setScaling(null)} />}
    </div>
  );
}

function JobLoader({ cluster }: { cluster: ClusterView }) {
  const params = useParams();
  const job: JobRef = {
    clusterId: cluster.cluster.id,
    namespace: params.namespace ?? 'default',
    jobId: params.jobId ?? '',
  };
  const detail = useQuery(nomadJobQueries.detail(job.clusterId, job.namespace, job.jobId));
  useNomadWatch(
    job.clusterId,
    { topic: 'job', namespace: job.namespace, jobId: job.jobId },
    nomadKeys.job(job.clusterId, job.namespace, job.jobId),
  );
  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />;
  return <JobDetail cluster={cluster} job={job} detail={detail.data} />;
}

/** A Nomad job: its settings, task groups and allocations, with Stop and Scale. */
export function JobPage() {
  return (
    <ClusterGate>
      {(cluster) => <JobLoader key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
