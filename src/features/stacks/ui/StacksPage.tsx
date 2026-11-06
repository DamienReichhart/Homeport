import { useQuery } from '@tanstack/react-query';
import { BoxesIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { CheckNewerImages, HostGate, indexFreshness, resourceQueries } from '@/core/host';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirm,
  WithShortcut,
} from '@/shared/ui';
import type { ComposeJob } from '../model/composeJob';
import { serviceFreshness, stackFreshness } from '../model/freshness';
import { ComposeRunDialog } from './dialogs/ComposeRunDialog';
import { stackColumns } from './stackColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { usePageQuery } from '@/core/feature';
import { t } from '@/shared/i18n';

function StacksList({ hostId }: { hostId: string }) {
  const stacks = useQuery(resourceQueries.stacks(hostId));
  const containers = useQuery(resourceQueries.containers(hostId));
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);
  const byId = new Map(containers.data?.map((c) => [c.id, c]));
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = usePageQuery();
  const [job, setJob] = useState<ComposeJob | null>(null);
  usePageAction(t('stacks.new'), () => void navigate('/stacks/new'));

  if (stacks.isPending) return <Spinner />;
  if (stacks.error)
    return <ErrorMessage error={stacks.error} onRetry={() => void stacks.refetch()} />;
  return (
    <div>
      <PageHeader
        title={t('stacks.title')}
        description={t('stacks.description')}
        actions={
          <>
            <CheckNewerImages hostId={hostId} />
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('stacks.new')}>
              <Button asChild>
                <Link to="/stacks/new">
                  <PlusIcon /> {t('stacks.new')}
                </Link>
              </Button>
            </WithShortcut>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('stacks.search')} />
      </div>
      {stacks.data.length === 0 ? (
        <EmptyState
          icon={BoxesIcon}
          title={t('stacks.empty.title')}
          description={t('stacks.empty.description')}
          action={
            <Button asChild>
              <Link to="/stacks/new">{t('stacks.new')}</Link>
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={stackColumns({
            hostId,
            confirm,
            onCompose: setJob,
            newerServices: (stack) =>
              stackFreshness(serviceFreshness(stack, byId, freshness)).newer,
          })}
          data={stacks.data}
          getRowId={(s) => s.name}
          filter={query}
          onRowClick={(s) => navigate(`/stacks/${encodeURIComponent(s.name)}`)}
        />
      )}
      <ComposeRunDialog hostId={hostId} job={job} onClose={() => setJob(null)} />
      {dialog}
    </div>
  );
}

export function StacksPage() {
  return <HostGate>{(hostId) => <StacksList key={hostId} hostId={hostId} />}</HostGate>;
}
