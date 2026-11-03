import { useQuery } from '@tanstack/react-query';
import { ActivityIcon } from 'lucide-react';
import { useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
import {
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
} from '@/shared/ui';
import { ALL_KINDS, filterEvents } from '../model/filterEvents';
import { EventsTable } from './EventsTable';

function EventsList({ hostId }: { hostId: string }) {
  // Seeded from the backend's history; live updates append pushed events to this cache.
  const events = useQuery({ ...resourceQueries.events(hostId), staleTime: Infinity });
  const [kind, setKind] = useState(ALL_KINDS);
  const [query, setQuery] = useState('');
  if (events.isPending) return <Spinner />;
  if (events.error)
    return <ErrorMessage error={events.error} onRetry={() => void events.refetch()} />;
  const all = events.data;
  const kinds = [...new Set(all.map((e) => e.kind))].sort();
  const shown = filterEvents(all, kind, query).reverse();

  return (
    <div>
      <PageHeader
        title="Events"
        description="What the Engine reported since Homeport connected (the last 500)."
      />
      <div className="mb-3 flex gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search events" />
        <Select value={kind} onValueChange={setKind}>
          <SelectTrigger className="w-40" aria-label="Type">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL_KINDS}>All types</SelectItem>
            {kinds.map((k) => (
              <SelectItem key={k} value={k} className="capitalize">
                {k}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {shown.length === 0 ? (
        <EmptyState
          icon={ActivityIcon}
          title="No events yet"
          description="Start or stop a container and it shows up here."
        />
      ) : (
        <EventsTable events={shown} />
      )}
    </div>
  );
}

export function EventsPage() {
  return <HostGate>{(hostId) => <EventsList key={hostId} hostId={hostId} />}</HostGate>;
}
