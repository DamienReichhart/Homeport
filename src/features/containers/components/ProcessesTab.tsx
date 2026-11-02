import { useQuery } from '@tanstack/react-query';
import { commands, toAppError } from '@/shared/api';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/shared/ui/table';
import { ErrorMessage, Spinner } from '@/shared/ui';

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
    queryKey: ['processes', hostId, containerId],
    queryFn: () => commands.containersProcesses(hostId, containerId),
    enabled: running,
    refetchInterval: 5000,
  });
  if (!running)
    return <p className="text-muted-foreground text-sm">The container isn&apos;t running.</p>;
  if (top.isPending) return <Spinner />;
  if (top.error) return <ErrorMessage error={toAppError(top.error)} />;
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            {top.data.titles.map((t) => (
              <TableHead key={t}>{t}</TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {top.data.processes.map((p, i) => (
            <TableRow key={i}>
              {p.map((cell, j) => (
                <TableCell
                  key={j}
                  className={j === p.length - 1 ? 'font-mono text-xs' : 'tabular-nums'}
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
