import { TriangleAlertIcon } from 'lucide-react';
import type { EngineEvent } from '@/shared/api';
import { cn, formatDateTime, formatRelative } from '@/shared/lib';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/shared/ui';
import { actorName, isTrouble } from '../model/filterEvents';
import { t } from '@/shared/i18n';

/**
 * The events, newest first: when (relative, the exact time on hover), what
 * kind of object, what happened, to which object, in which Stack. An event
 * worth a look (a failed exit, an out-of-memory kill) is marked in amber.
 */
export function EventsTable({ events }: { events: EngineEvent[] }) {
  return (
    <div className="bg-card rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>{t('events.column.time')}</TableHead>
            <TableHead>{t('events.column.type')}</TableHead>
            <TableHead>{t('events.column.action')}</TableHead>
            <TableHead>{t('events.column.object')}</TableHead>
            <TableHead>{t('events.column.stack')}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {events.map((e, i) => {
            const trouble = isTrouble(e);
            const name = actorName(e);
            return (
              <TableRow key={`${e.time}-${e.actorId}-${e.action}-${i}`}>
                <TableCell className="text-muted-foreground w-36">
                  <time
                    dateTime={new Date(e.time * 1000).toISOString()}
                    title={formatDateTime(e.time)}
                  >
                    {formatRelative(e.time)}
                  </time>
                </TableCell>
                <TableCell className="text-muted-foreground w-28 capitalize">{e.kind}</TableCell>
                <TableCell>
                  <span
                    className={cn(
                      'inline-flex max-w-xs items-center gap-1.5 font-medium',
                      trouble && 'text-warning',
                    )}
                  >
                    {trouble && <TriangleAlertIcon className="size-3.5 shrink-0" aria-hidden />}
                    <span className="truncate" title={e.action}>
                      {e.action}
                    </span>
                  </span>
                </TableCell>
                <TableCell>
                  <span className="block max-w-md truncate" title={name}>
                    {name}
                  </span>
                </TableCell>
                <TableCell className="text-muted-foreground">
                  {e.attributes['com.docker.compose.project'] ?? ''}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </div>
  );
}
