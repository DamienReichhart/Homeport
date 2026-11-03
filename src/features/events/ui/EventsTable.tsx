import type { EngineEvent } from '@/shared/api';
import { formatDateTime } from '@/shared/lib';
import { Badge, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/shared/ui';
import { actorName } from '../model/filterEvents';

export function EventsTable({ events }: { events: EngineEvent[] }) {
  return (
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
          {events.map((e, i) => (
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
  );
}
