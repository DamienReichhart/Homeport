import { useQuery } from '@tanstack/react-query';
import { HardDriveIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { ContainerdGate } from '@/core/host';
import type { ContainerdSnapshot, SnapshotKind } from '@/shared/api';
import { cn, formatBytes } from '@/shared/lib';
import {
  Badge,
  Button,
  EmptyState,
  ErrorMessage,
  Label,
  PageHeader,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Switch,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  useConfirmedAction,
} from '@/shared/ui';
import { useRemoveSnapshot } from '../api/mutations';
import { snapshotQueries } from '../api/queries';
import { removable, shortKey, snapshotTree } from '../model/tree';
import { t, type MessageKey } from '@/shared/i18n';

const KINDS: Record<SnapshotKind, MessageKey> = {
  committed: 'containerd.snapshots.kind.committed',
  active: 'containerd.snapshots.kind.active',
  view: 'containerd.snapshots.kind.view',
  unknown: 'containerd.snapshots.kind.unknown',
};

function UsedBy({ snapshot }: { snapshot: ContainerdSnapshot }) {
  if (snapshot.users.length === 0)
    return <span className="text-muted-foreground">{t('containerd.snapshots.unused')}</span>;
  return (
    <span className="flex flex-wrap gap-1">
      {snapshot.users.map((u) => (
        <Badge key={u.kind === 'container' ? `c:${u.id}` : `i:${u.name}`} variant="outline">
          {u.kind === 'container'
            ? t('containerd.snapshots.usedByContainer', { id: u.id })
            : t('containerd.snapshots.usedByImage', { name: u.name })}
        </Badge>
      ))}
    </span>
  );
}

function SnapshotsList({ hostId, namespace }: { hostId: string; namespace: string }) {
  const [snapshotter, setSnapshotter] = useState<string | null>(null);
  const [unusedOnly, setUnusedOnly] = useState(false);
  const snapshots = useQuery(snapshotQueries.list(hostId, namespace, snapshotter));
  const remove = useRemoveSnapshot(hostId, namespace);
  const [dialog, confirm] = useConfirmedAction();

  if (snapshots.isPending) return <Spinner />;
  if (snapshots.error)
    return <ErrorMessage error={snapshots.error} onRetry={() => void snapshots.refetch()} />;
  const data = snapshots.data;
  const rows = snapshotTree(data.snapshots).filter((r) => !unusedOnly || removable(r.snapshot));
  const unused = data.snapshots.filter(removable);
  return (
    <div>
      <PageHeader
        title={t('containerd.snapshots.title')}
        description={t('containerd.snapshots.summary', {
          count: data.snapshots.length,
          total: formatBytes(data.total),
          unused: unused.length,
          unusedSize: formatBytes(unused.reduce((sum, s) => sum + (s.size ?? 0), 0)),
          namespace,
        })}
      />
      <div className="mb-3 flex flex-wrap items-center gap-4">
        <Select value={data.snapshotter} onValueChange={setSnapshotter}>
          <SelectTrigger className="w-44" aria-label={t('containerd.snapshots.snapshotter')}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {(data.snapshotters.length ? data.snapshotters : [data.snapshotter]).map((s) => (
              <SelectItem key={s} value={s}>
                {s}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <div className="flex items-center gap-2">
          <Switch id="unused-only" checked={unusedOnly} onCheckedChange={setUnusedOnly} />
          <Label htmlFor="unused-only">{t('containerd.snapshots.unusedOnly')}</Label>
        </div>
      </div>
      {rows.length === 0 ? (
        <EmptyState
          icon={HardDriveIcon}
          title={t('containerd.snapshots.empty', { snapshotter: data.snapshotter })}
          description={t('containerd.snapshots.emptyDescription')}
        />
      ) : (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t('containerd.snapshots.column.key')}</TableHead>
              <TableHead>{t('containerd.snapshots.column.kind')}</TableHead>
              <TableHead className="text-end">{t('containerd.snapshots.column.size')}</TableHead>
              <TableHead>{t('containerd.snapshots.column.usedBy')}</TableHead>
              <TableHead />
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map(({ snapshot, depth }) => (
              <TableRow key={snapshot.key}>
                <TableCell>
                  <span
                    className={cn('font-mono text-xs', depth > 0 && 'border-s ps-2')}
                    style={{ marginLeft: `${depth * 0.75}rem` }}
                    title={snapshot.key}
                  >
                    {shortKey(snapshot.key)}
                  </span>
                </TableCell>
                <TableCell>{t(KINDS[snapshot.kind])}</TableCell>
                <TableCell className="text-end whitespace-nowrap">
                  {snapshot.size === null ? '—' : formatBytes(snapshot.size)}
                </TableCell>
                <TableCell>
                  <UsedBy snapshot={snapshot} />
                </TableCell>
                <TableCell className="text-end">
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    aria-label={t('containerd.snapshots.removeOne', {
                      key: shortKey(snapshot.key),
                    })}
                    title={removable(snapshot) ? undefined : t('containerd.snapshots.inUse')}
                    disabled={!removable(snapshot) || remove.isPending}
                    onClick={() =>
                      confirm({
                        title: t('containerd.snapshots.remove.title', {
                          key: shortKey(snapshot.key),
                        }),
                        description: t('containerd.snapshots.remove.description', {
                          size: snapshot.size === null ? '—' : formatBytes(snapshot.size),
                        }),
                        confirmLabel: t('common.remove'),
                        destructive: true,
                        action: () =>
                          remove.mutateAsync({ snapshotter: data.snapshotter, key: snapshot.key }),
                      })
                    }
                  >
                    <Trash2Icon />
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
      {dialog}
    </div>
  );
}

export function SnapshotsPage() {
  return (
    <ContainerdGate>
      {(hostId, namespace) => (
        <SnapshotsList key={`${hostId}/${namespace}`} hostId={hostId} namespace={namespace} />
      )}
    </ContainerdGate>
  );
}
