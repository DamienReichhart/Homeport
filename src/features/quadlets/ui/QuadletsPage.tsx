import { useQuery } from '@tanstack/react-query';
import { InfoIcon, PlusIcon, ScrollTextIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useActionRequest, usePageQuery } from '@/core/feature';
import { HostGate } from '@/core/host';
import {
  Alert,
  AlertDescription,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  WithShortcut,
} from '@/shared/ui';
import { quadletQueries } from '../api/queries';
import { filesBlocked, unitKey } from '../model/units';
import { UnitDialog } from './dialogs/UnitDialog';
import { unitColumns } from './unitColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function QuadletsList({ hostId }: { hostId: string }) {
  const units = useQuery(quadletQueries.list(hostId));
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [creating, setCreating] = useState(false);
  const blocked = units.data ? filesBlocked(units.data) : null;
  const create = () => !blocked && setCreating(true);
  usePageAction(t('quadlets.new'), create);
  useActionRequest('new', create);

  if (units.isPending) return <Spinner />;
  if (units.error) return <ErrorMessage error={units.error} onRetry={() => void units.refetch()} />;
  const data = units.data;
  return (
    <div>
      <PageHeader
        title={t('quadlets.title')}
        description={t('quadlets.summary', { count: data.units.length })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('quadlets.new')}>
            <span title={blocked ? t(blocked, { version: data.podmanVersion }) : undefined}>
              <Button disabled={blocked !== null} onClick={create}>
                <PlusIcon /> {t('quadlets.new')}
              </Button>
            </span>
          </WithShortcut>
        }
      />
      {(data.source === 'labels' || !data.managesFiles || !data.runsSystemctl) && (
        <Alert className="mb-3">
          <InfoIcon />
          <AlertDescription className="grid gap-1">
            {data.source === 'labels' && (
              <p>{t('quadlets.note.labels', { version: data.podmanVersion })}</p>
            )}
            {data.source === 'api' && !data.managesFiles && (
              <p>{t('quadlets.note.readOnly', { version: data.podmanVersion })}</p>
            )}
            {!data.runsSystemctl && <p>{t('quadlets.note.noCommands')}</p>}
          </AlertDescription>
        </Alert>
      )}
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('quadlets.search')} />
      </div>
      {data.units.length === 0 ? (
        <EmptyState
          icon={ScrollTextIcon}
          title={t('quadlets.empty')}
          description={t('quadlets.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={unitColumns({ hostId, units: data })}
          data={data.units}
          getRowId={unitKey}
          filter={query}
          onRowClick={(u) => void navigate(`/quadlets/${encodeURIComponent(unitKey(u))}`)}
        />
      )}
      <UnitDialog hostId={hostId} open={creating} onOpenChange={setCreating} />
    </div>
  );
}

export function QuadletsPage() {
  return (
    <HostGate requires="podman">
      {(hostId) => <QuadletsList key={hostId} hostId={hostId} />}
    </HostGate>
  );
}
