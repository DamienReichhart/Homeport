import { useQuery } from '@tanstack/react-query';
import { BriefcaseIcon, PlayIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { age, ClusterGate } from '@/core/cluster';
import { usePageQuery } from '@/core/feature';
import {
  ALL_NOMAD_NAMESPACES,
  jobTone,
  NomadNamespacePicker,
  NomadStatus,
  nomadKeys,
  useNomadNamespace,
  useNomadWatch,
} from '@/core/nomad';
import type { ClusterView, NomadJobView } from '@/shared/api';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { nomadJobQueries } from '../api/queries';
import { jobCounts, jobPath } from '../model/jobs';
import { RunJobDialog } from './RunJobDialog';
import { t } from '@/shared/i18n';

const namespaceColumn: ColumnDef<NomadJobView, unknown> = {
  id: 'namespace',
  header: t('nomad.jobs.column.namespace'),
  accessorFn: (j) => j.namespace,
};

const nameColumn: ColumnDef<NomadJobView, unknown> = {
  id: 'name',
  header: t('nomad.jobs.column.name'),
  accessorFn: (j) => j.name,
  cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
};

const otherColumns: ColumnDef<NomadJobView, unknown>[] = [
  {
    id: 'type',
    header: t('nomad.jobs.column.type'),
    accessorFn: (j) => j.jobType,
  },
  {
    id: 'status',
    header: t('nomad.jobs.column.status'),
    accessorFn: (j) => (j.stopped ? 'stopped' : j.status),
    cell: ({ row }) =>
      row.original.stopped ? (
        <NomadStatus status="stopped" tone="muted" />
      ) : (
        <NomadStatus
          status={row.original.status}
          tone={jobTone(row.original.status, row.original.stopped)}
        />
      ),
  },
  {
    id: 'groups',
    header: t('nomad.jobs.column.groups'),
    accessorFn: (j) => jobCounts(j).running,
    cell: ({ row }) => {
      const { running, desired } = jobCounts(row.original);
      return (
        <span className="tabular-nums" title={row.original.groups.map((g) => g.name).join(', ')}>
          {desired === null
            ? t('nomad.jobs.running', { running })
            : t('nomad.jobs.runningOf', { running, desired })}
        </span>
      );
    },
  },
  {
    id: 'submitted',
    header: t('nomad.jobs.column.submitted'),
    accessorFn: (j) => j.submitted ?? '',
    cell: ({ row }) => age(row.original.submitted),
  },
];

function JobsList({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const selection = useNomadNamespace(cluster);
  const { namespace } = selection;
  const all = namespace === ALL_NOMAD_NAMESPACES;
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [running, setRunning] = useState(false);
  // Asked once the namespaces are known, so the token's default isn't fetched for nothing.
  const jobs = useQuery({ ...nomadJobQueries.list(id, namespace), enabled: selection.ready });
  useNomadWatch(id, { topic: 'jobs', namespace }, nomadKeys.jobs(id, namespace), selection.ready);
  const shown = (jobs.data ?? []).filter((j) =>
    j.name.toLowerCase().includes(query.trim().toLowerCase()),
  );
  return (
    <div>
      <PageHeader
        title={t('nomad.jobs.title')}
        description={jobs.data && t('nomad.jobs.summary', { count: jobs.data.length })}
        actions={
          <>
            <NomadNamespacePicker selection={selection} />
            <Button onClick={() => setRunning(true)}>
              <PlayIcon /> {t('nomad.jobs.run')}
            </Button>
          </>
        }
      />
      {running && (
        <RunJobDialog
          clusterId={id}
          clusterName={cluster.cluster.name}
          namespace={all ? null : namespace}
          onClose={() => setRunning(false)}
          onRan={(result) => void navigate(jobPath(result.namespace, result.jobId))}
        />
      )}
      <div className="mb-3 flex flex-wrap items-center gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('nomad.jobs.search')} />
      </div>
      {jobs.isPending ? (
        <Spinner />
      ) : jobs.error ? (
        <ErrorMessage error={jobs.error} onRetry={() => void jobs.refetch()} />
      ) : jobs.data.length === 0 ? (
        <EmptyState
          icon={BriefcaseIcon}
          title={t('nomad.jobs.empty.title')}
          description={t('nomad.jobs.empty.description')}
        />
      ) : (
        <DataTable
          columns={
            all ? [nameColumn, namespaceColumn, ...otherColumns] : [nameColumn, ...otherColumns]
          }
          data={shown}
          getRowId={(j) => `${j.namespace}/${j.id}`}
          onRowClick={(j) => void navigate(jobPath(j.namespace, j.id))}
        />
      )}
    </div>
  );
}

/** The selected Nomad Cluster's jobs, kept current while the page is open. */
export function JobsPage() {
  return (
    <ClusterGate>
      {(cluster) => <JobsList key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
