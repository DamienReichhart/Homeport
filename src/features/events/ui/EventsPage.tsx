import { useQuery } from '@tanstack/react-query';
import { ActivityIcon, Trash2Icon } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
import { settingsQuery } from '@/core/settings';
import {
  Button,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import { useClearEvents, useOlderEvents } from '../api/history';
import { ALL_KINDS, filterEvents } from '../model/filterEvents';
import { EventsTable } from './EventsTable';
import { t } from '@/shared/i18n';

function EventsList({ hostId }: { hostId: string }) {
  // Seeded from the kept history; live updates append pushed events to this cache.
  const events = useQuery({ ...resourceQueries.events(hostId), staleTime: Infinity });
  const settings = useQuery(settingsQuery());
  const older = useOlderEvents(hostId, events.data ?? []);
  const clear = useClearEvents(hostId, older.reset);
  const [dialog, confirm] = useConfirm();
  const [kind, setKind] = useState(ALL_KINDS);
  const [query, setQuery] = useState('');
  if (events.isPending) return <Spinner />;
  if (events.error)
    return <ErrorMessage error={events.error} onRetry={() => void events.refetch()} />;
  const all = [...older.older, ...events.data];
  const kinds = [...new Set(all.map((e) => e.kind))].sort();
  const shown = filterEvents(all, kind, query).reverse();
  const days = settings.data?.keepEventsDays;

  return (
    <div>
      <PageHeader
        title={t('events.title')}
        description={
          days === undefined ? t('events.description') : t('events.descriptionKept', { days })
        }
        actions={
          <Button
            variant="outline"
            disabled={all.length === 0}
            onClick={() =>
              confirm({
                title: t('events.clear.title'),
                description: t('events.clear.description'),
                confirmLabel: t('events.clear.confirm'),
                destructive: true,
                onConfirm: () => clear.mutateAsync(),
              })
            }
          >
            <Trash2Icon /> {t('events.clear')}
          </Button>
        }
      />
      <div className="mb-3 flex gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('events.search')} />
        <Select value={kind} onValueChange={setKind}>
          <SelectTrigger className="w-40" aria-label={t('events.type')}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL_KINDS}>{t('events.allTypes')}</SelectItem>
            {kinds.map((k) => (
              <SelectItem key={k} value={k} className="capitalize">
                {k}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {shown.length === 0 && !older.hasMore ? (
        <EmptyState
          icon={ActivityIcon}
          title={t('events.empty.title')}
          description={t('events.empty.description')}
        />
      ) : (
        <>
          <EventsTable events={shown} />
          {older.hasMore && <LoadOlder loading={older.loading} onLoad={older.loadMore} />}
        </>
      )}
      {dialog}
    </div>
  );
}

/** Loads older events when scrolled into view, or when clicked. */
function LoadOlder({ loading, onLoad }: { loading: boolean; onLoad: () => void }) {
  const button = useRef<HTMLButtonElement>(null);
  const load = useRef(onLoad);
  useEffect(() => {
    load.current = onLoad;
  });
  useEffect(() => {
    const target = button.current;
    if (!target || typeof IntersectionObserver === 'undefined') return;
    const observer = new IntersectionObserver((entries) => {
      if (entries.some((e) => e.isIntersecting)) load.current();
    });
    observer.observe(target);
    return () => observer.disconnect();
  }, []);
  return (
    <div className="mt-3 flex justify-center">
      <Button ref={button} variant="outline" disabled={loading} onClick={onLoad}>
        {loading ? t('common.loading') : t('events.loadOlder')}
      </Button>
    </div>
  );
}

export function EventsPage() {
  return <HostGate>{(hostId) => <EventsList key={hostId} hostId={hostId} />}</HostGate>;
}
