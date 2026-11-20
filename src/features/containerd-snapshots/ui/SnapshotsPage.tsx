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
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  useConfirmedAction,
} from '@/shared/ui';
import { usePruneSnapshots, useRemoveSnapshot } from '../api/mutations';
import { snapshotQueries } from '../api/queries';
import { prunable, removable, shortKey, snapshotTree } from '../model/tree';
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
  const names = snapshot.users.map((u) =>
    u.kind === 'container'
      ? t('containerd.snapshots.usedByContainer', { id: u.id })
      : t('containerd.snapshots.usedByImage', { name: u.name }),
  );
  return (
    <span className="flex max-w-96 gap-1 overflow-hidden" title={names.join('\n')}>
      {names.slice(0, 2).map((name) => (
        <Badge key={name} variant="outline" className="max-w-72 truncate">
          {name}
        </Badge>
      ))}
      {names.length > 2 && <Badge variant="outline">+{names.length - 2}</Badge>}
    </span>
  );
}

function SnapshotsList({ hostId, namespace }: { hostId: string; namespace: string }) {
  const [snapshotter, setSnapshotter] = useState<string | null>(null);
  const [unusedOnly, setUnusedOnly] = useState(false);
  const snapshots = useQuery(snapshotQueries.list(hostId, namespace, snapshotter));
  const remove = useRemoveSnapshot(hostId, namespace);
  const prune = usePruneSnapshots(hostId, namespace);
  const [dialog, confirm] = useConfirmedAction();

  if (snapshots.isPending) return <Spinner />;
  if (snapshots.error)
    return <ErrorMessage error={snapshots.error} onRetry={() => void snapshots.refetch()} />;
  const data = snapshots.data;
  const rows = snapshotTree(data.snapshots).filter((r) => !unusedOnly || removable(r.snapshot));
  const unused = prunable(data.snapshots);
  const unusedSize = formatBytes(unused.reduce((sum, s) => sum + (s.size ?? 0), 0));
  return (
    <div>
      <PageHeader
        title={t('containerd.snapshots.title')}
        description={t('containerd.snapshots.summary', {
          count: data.snapshots.length,
          total: formatBytes(data.total),
          unused: unused.length,
          unusedSize,
          namespace,
        })}
        actions={
          <Button
            variant="outline"
            disabled={unused.length === 0 || prune.isPending}
            onClick={() =>
              confirm({
                title: t('containerd.snapshots.prune.title', { count: unused.length }),
                description: t('containerd.snapshots.prune.description', { size: unusedSize }),
                confirmLabel: t('containerd.snapshots.prune.open'),
                destructive: true,
                action: () =>
                  prune.mutateAsync({
                    snapshotter: data.snapshotter,
                    keys: unused.map((s) => s.key),
                  }),
              })
            }
          >
            <Trash2Icon /> {t('containerd.snapshots.prune.open')}
          </Button>
        }
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
                <TableCell className="text-muted-foreground">{t(KINDS[snapshot.kind])}</TableCell>
                <TableCell className="text-end whitespace-nowrap tabular-nums">
                  {snapshot.size === null ? '—' : formatBytes(snapshot.size)}
                </TableCell>
                <TableCell>
                  <UsedBy snapshot={snapshot} />
                </TableCell>
                <TableCell className="text-end">
                  <Tooltip>
                    <TooltipTrigger asChild>
                      {/* A disabled button shows no tooltip; its wrapper does. */}
                      <span tabIndex={removable(snapshot) ? -1 : 0}>
                        <Button
                          variant="ghost"
                          size="icon-sm"
                          aria-label={t('containerd.snapshots.removeOne', {
                            key: shortKey(snapshot.key),
                          })}
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
                                remove.mutateAsync({
                                  snapshotter: data.snapshotter,
                                  key: snapshot.key,
                                }),
                            })
                          }
                        >
                          <Trash2Icon />
                        </Button>
                      </span>
                    </TooltipTrigger>
                    <TooltipContent>
                      {removable(snapshot)
                        ? t('containerd.snapshots.removeOne', { key: shortKey(snapshot.key) })
                        : t('containerd.snapshots.inUse')}
                    </TooltipContent>
                  </Tooltip>
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
