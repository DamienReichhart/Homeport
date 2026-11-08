import { useQuery } from '@tanstack/react-query';
import { ServerIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePageQuery } from '@/core/feature';
import { IncusGate } from '@/core/host';
import type { IncusInstance } from '@/shared/api';
import {
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { incusInstanceQueries } from '../api/queries';
import { filterByType, type TypeFilter } from '../model/instances';
import { DeleteInstanceDialog } from './DeleteInstanceDialog';
import { instanceColumns } from './instanceColumns';
import { t } from '@/shared/i18n';

function InstancesList({ hostId, project }: { hostId: string; project: string }) {
  const instances = useQuery(incusInstanceQueries.list(hostId, project));
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [type, setType] = useState<TypeFilter>('all');
  const [deleting, setDeleting] = useState<IncusInstance | null>(null);

  if (instances.isPending) return <Spinner />;
  if (instances.error)
    return <ErrorMessage error={instances.error} onRetry={() => void instances.refetch()} />;
  const shown = filterByType(instances.data, type);
  return (
    <div>
      <PageHeader
        title={t('incus.instances.title')}
        description={t('incus.instances.summary', {
          count: instances.data.length,
          running: instances.data.filter((i) => i.status === 'running').length,
          project,
        })}
      />
      <div className="mb-3 flex flex-wrap items-center gap-4">
        <SearchInput value={query} onChange={setQuery} placeholder={t('incus.instances.search')} />
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          value={type}
          onValueChange={(v) => v && setType(v as TypeFilter)}
          aria-label={t('incus.instances.column.type')}
        >
          <ToggleGroupItem value="all">{t('incus.instances.filter.all')}</ToggleGroupItem>
          <ToggleGroupItem value="container">{t('incus.type.container')}</ToggleGroupItem>
          <ToggleGroupItem value="virtual-machine">
            {t('incus.type.virtualMachine')}
          </ToggleGroupItem>
        </ToggleGroup>
      </div>
      {shown.length === 0 ? (
        <EmptyState
          icon={ServerIcon}
          title={t('incus.instances.empty', { project })}
          description={t('incus.instances.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={instanceColumns({ hostId, project, onDelete: setDeleting })}
          data={shown}
          getRowId={(i) => i.name}
          filter={query}
          onRowClick={(i) => void navigate(`/incus/instances/${encodeURIComponent(i.name)}`)}
        />
      )}
      <DeleteInstanceDialog
        hostId={hostId}
        project={project}
        instance={deleting}
        onClose={() => setDeleting(null)}
      />
    </div>
  );
}

export function InstancesPage() {
  return (
    <IncusGate>
      {(hostId, project) => (
        <InstancesList key={`${hostId}/${project}`} hostId={hostId} project={project} />
      )}
    </IncusGate>
  );
}
