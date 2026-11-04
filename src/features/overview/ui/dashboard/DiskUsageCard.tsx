import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon } from 'lucide-react';
import { useState } from 'react';
import { resourceQueries } from '@/core/host';
import { formatBytes } from '@/shared/lib';
import {
  Button,
  Card,
  CardAction,
  CardContent,
  CardHeader,
  CardTitle,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { PruneDialog } from '../dialogs/PruneDialog';

/** What takes space on a Host's disk, and the clean-up. */
export function DiskUsageCard({ hostId }: { hostId: string }) {
  const [pruning, setPruning] = useState(false);
  const usage = useQuery(resourceQueries.diskUsage(hostId));
  const partial = usage.data?.partial ?? false;
  // With an older Engine, volume and build cache sizes are unknown, not zero.
  const rows = usage.data
    ? ([
        ['Images', usage.data.images, true],
        ['Containers', usage.data.containers, true],
        ['Volumes', usage.data.volumes, !partial],
        ['Build cache', usage.data.buildCache, !partial],
      ] as const)
    : [];

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>Disk usage</CardTitle>
          <CardAction>
            <Button variant="outline" size="sm" onClick={() => setPruning(true)}>
              <BrushCleaningIcon /> Clean up…
            </Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead className="text-right">Total</TableHead>
                <TableHead className="text-right">In use</TableHead>
                <TableHead className="text-right">Size</TableHead>
                <TableHead className="text-right">Reclaimable</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map(([label, line, sized]) => (
                <TableRow key={label}>
                  <TableCell>{label}</TableCell>
                  <TableCell className="text-right">{line.total}</TableCell>
                  <TableCell className="text-right">{line.active}</TableCell>
                  <TableCell className="text-right">
                    {sized ? formatBytes(line.size) : '—'}
                  </TableCell>
                  <TableCell className="text-right">
                    {sized ? formatBytes(line.reclaimable) : '—'}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {partial && (
            <p className="text-muted-foreground mt-2 text-xs">
              This Docker Engine is older than version 29 and reports less: image sizes are
              estimated, and volume and build cache sizes aren’t available.
            </p>
          )}
        </CardContent>
      </Card>
      <PruneDialog hostId={hostId} open={pruning} onOpenChange={setPruning} />
    </>
  );
}
