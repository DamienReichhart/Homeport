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
import { t } from '@/shared/i18n';

/** What takes space on a Host's disk, and the clean-up. */
export function DiskUsageCard({ hostId }: { hostId: string }) {
  const [pruning, setPruning] = useState(false);
  const usage = useQuery(resourceQueries.diskUsage(hostId));
  const partial = usage.data?.partial ?? false;
  // With an older Engine, volume and build cache sizes are unknown, not zero.
  const rows = usage.data
    ? ([
        [t('overview.disk.images'), usage.data.images, true],
        [t('overview.disk.containers'), usage.data.containers, true],
        [t('overview.disk.volumes'), usage.data.volumes, !partial],
        [t('overview.disk.buildCache'), usage.data.buildCache, !partial],
      ] as const)
    : [];

  return (
    <>
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.disk.title')}</CardTitle>
          <CardAction>
            <Button variant="outline" size="sm" onClick={() => setPruning(true)}>
              <BrushCleaningIcon /> {t('overview.disk.cleanUp')}
            </Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t('overview.disk.column.type')}</TableHead>
                <TableHead className="text-right">{t('overview.disk.column.total')}</TableHead>
                <TableHead className="text-right">{t('overview.disk.column.active')}</TableHead>
                <TableHead className="text-right">{t('overview.disk.column.size')}</TableHead>
                <TableHead className="text-right">
                  {t('overview.disk.column.reclaimable')}
                </TableHead>
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
            <p className="text-muted-foreground mt-2 text-xs">{t('overview.disk.partial')}</p>
          )}
        </CardContent>
      </Card>
      <PruneDialog hostId={hostId} open={pruning} onOpenChange={setPruning} />
    </>
  );
}
