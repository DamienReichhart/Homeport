import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { resourceQueries } from '@/core/host';
import { formatRelative } from '@/shared/lib';
import { Button, Card, CardAction, CardContent, CardHeader, CardTitle } from '@/shared/ui';

/** The last few Engine events of a Host, newest first. */
export function RecentEventsCard({ hostId }: { hostId: string }) {
  const events = useQuery(resourceQueries.events(hostId));
  return (
    <Card className="lg:col-span-2">
      <CardHeader>
        <CardTitle>Recent events</CardTitle>
        <CardAction>
          <Button variant="ghost" size="sm" asChild>
            <Link to="/events">All events</Link>
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>
        {events.data?.length ? (
          <ul className="grid gap-1 text-sm">
            {events.data
              .slice(-8)
              .reverse()
              .map((e, i) => (
                <li key={`${e.time}-${i}`} className="flex gap-3">
                  <span className="text-muted-foreground w-28 shrink-0">
                    {formatRelative(e.time)}
                  </span>
                  <span className="w-20 shrink-0 capitalize">{e.kind}</span>
                  <span className="font-medium">{e.action}</span>
                  <span className="text-muted-foreground truncate">
                    {e.attributes.name ?? e.actorId.slice(0, 12)}
                  </span>
                </li>
              ))}
          </ul>
        ) : (
          <p className="text-muted-foreground text-sm">
            Nothing has happened since Homeport connected.
          </p>
        )}
      </CardContent>
    </Card>
  );
}
