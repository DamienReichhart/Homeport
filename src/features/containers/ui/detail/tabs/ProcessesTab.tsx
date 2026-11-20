import { useQuery } from '@tanstack/react-query';
import {
  ErrorMessage,
  Spinner,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { containerQueries } from '../../../api/queries';
import { t } from '@/shared/i18n';

export function ProcessesTab({
  hostId,
  containerId,
  running,
}: {
  hostId: string;
  containerId: string;
  running: boolean;
}) {
  const top = useQuery({
    ...containerQueries.processes(hostId, containerId),
    enabled: running,
    refetchInterval: 5000,
  });
  if (!running)
    return <p className="text-muted-foreground text-sm">{t('containers.notRunning')}</p>;
  if (top.isPending) return <Spinner />;
  if (top.error) return <ErrorMessage error={top.error} onRetry={() => void top.refetch()} />;
  // Columns that only hold numbers (PID, PPID, C, %CPU…) line up on the end.
  const numeric = top.data.titles.map((_, j) =>
    top.data.processes.every((p) => /^\d+(\.\d+)?$/.test(p[j] ?? '')),
  );
  const last = top.data.titles.length - 1;
  return (
    <div className="bg-card rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            {top.data.titles.map((title, j) => (
              <TableHead key={title} className={numeric[j] ? 'text-end' : undefined}>
                {title}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {top.data.processes.map((p, i) => (
            <TableRow key={i}>
              {p.map((cell, j) => (
                <TableCell
                  key={j}
                  title={j === last ? cell : undefined}
                  className={
                    j === last
                      ? 'max-w-md truncate font-mono text-xs'
                      : numeric[j]
                        ? 'text-end tabular-nums'
                        : 'tabular-nums'
                  }
                >
                  {cell}
                </TableCell>
              ))}
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
