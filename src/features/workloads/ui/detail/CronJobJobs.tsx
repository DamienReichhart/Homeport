import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { useResourceWatch } from '@/core/cluster';
import { DataTable, ErrorMessage, Spinner } from '@/shared/ui';
import { workloadQueries } from '../../api/queries';
import { workloadPath } from '../../model/kinds';
import { workloadColumns } from '../workloadColumns';
import { t } from '@/shared/i18n';

/** The Jobs a CronJob started, newest first. */
export function CronJobJobs({
  clusterId,
  namespace,
  name,
}: {
  clusterId: string;
  namespace: string;
  name: string;
}) {
  const navigate = useNavigate();
  const jobs = useQuery(workloadQueries.jobsOf(clusterId, namespace, name));
  useResourceWatch(clusterId, 'job', namespace);
  if (jobs.isPending) return <Spinner />;
  if (jobs.error) return <ErrorMessage error={jobs.error} onRetry={() => void jobs.refetch()} />;
  return (
    <DataTable
      columns={workloadColumns('job', false)}
      data={jobs.data}
      getRowId={(j) => j.name}
      empty={t('workloads.detail.noJobs')}
      onRowClick={(j) => void navigate(workloadPath('job', j.namespace, j.name))}
    />
  );
}
