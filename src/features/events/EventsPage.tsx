import { useQuery } from '@tanstack/react-query';
import { ActivityIcon } from 'lucide-react';
import { useState } from 'react';
import { HostGate } from '@/core/host';
import { commands, keys, type EngineEvent } from '@/shared/api';
import { formatDateTime } from '@/shared/lib';
import {
  Badge,
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
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';

const ALL = 'all';

/** What an event is about, in words: the actor's name, else its short id. */
export function actorName(e: EngineEvent): string {
  return e.attributes.name ?? e.attributes.image ?? e.actorId.replace(/^sha256:/, '').slice(0, 12);
}

export function filterEvents(events: EngineEvent[], kind: string, query: string): EngineEvent[] {
  const q = query.trim().toLowerCase();
  return events.filter(
    (e) =>
      (kind === ALL || e.kind === kind) &&
      (!q || e.action.toLowerCase().includes(q) || actorName(e).toLowerCase().includes(q)),
  );
}

function EventsList({ hostId }: { hostId: string }) {
  // Seeded from the backend's history; LiveUpdates appends pushed events to this cache.
  const events = useQuery({
    queryKey: keys.events(hostId),
    queryFn: () => commands.hostsEvents(hostId),
    staleTime: Infinity,
  });
  const [kind, setKind] = useState(ALL);
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
            <SelectItem value={ALL}>All types</SelectItem>
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
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Time</TableHead>
                <TableHead>Type</TableHead>
                <TableHead>Action</TableHead>
                <TableHead>Object</TableHead>
                <TableHead>Stack</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {shown.map((e, i) => (
                <TableRow key={`${e.time}-${e.actorId}-${e.action}-${i}`}>
                  <TableCell className="text-muted-foreground whitespace-nowrap">
                    {formatDateTime(e.time)}
                  </TableCell>
                  <TableCell>
                    <Badge variant="outline" className="capitalize">
                      {e.kind}
                    </Badge>
                  </TableCell>
                  <TableCell className="font-medium">{e.action}</TableCell>
                  <TableCell className="max-w-md truncate">{actorName(e)}</TableCell>
                  <TableCell>{e.attributes['com.docker.compose.project'] ?? ''}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}
    </div>
  );
}

export function EventsPage() {
  return <HostGate>{(hostId) => <EventsList key={hostId} hostId={hostId} />}</HostGate>;
}
