import { Link } from 'react-router-dom';
import { useContainers } from '@/core/host';
import { containerTone } from '@/features/containers';
import type { Stack } from '@/shared/api';
import {
  StatusDot,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';

/** Each service of a Stack with its containers and their status. */
export function StackServices({ hostId, stack }: { hostId: string; stack: Stack }) {
  const containers = useContainers(hostId);
  const byId = new Map(containers.data?.map((c) => [c.id, c]));
  if (stack.services.length === 0)
    return <p className="text-muted-foreground text-sm">Not deployed yet.</p>;
  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Service</TableHead>
            <TableHead>Container</TableHead>
            <TableHead>Image</TableHead>
            <TableHead>Status</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {stack.services.flatMap((s) =>
            s.containerIds.map((id) => {
              const c = byId.get(id);
              return (
                <TableRow key={id}>
                  <TableCell className="font-medium">{s.name}</TableCell>
                  <TableCell>
                    <Link to={`/containers/${id}/logs`} className="text-primary hover:underline">
                      {c?.name ?? id.slice(0, 12)}
                    </Link>
                  </TableCell>
                  <TableCell>{c?.image}</TableCell>
                  <TableCell>
                    <span className="flex items-center gap-1.5">
                      {c && <StatusDot tone={containerTone(c.state, c.health)} />}
                      {c?.status}
                    </span>
                  </TableCell>
                </TableRow>
              );
            }),
          )}
        </TableBody>
      </Table>
    </div>
  );
}
