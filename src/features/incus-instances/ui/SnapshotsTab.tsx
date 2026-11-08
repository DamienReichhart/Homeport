import { useQuery } from '@tanstack/react-query';
import { CameraIcon, HistoryIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import type { IncusInstance, IncusSnapshot } from '@/shared/api';
import { formatBytes, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  DataTable,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  EmptyState,
  ErrorMessage,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Switch,
  useConfirmedAction,
  type ColumnDef,
} from '@/shared/ui';
import { useSnapshots } from '../api/mutations';
import { incusInstanceQueries } from '../api/queries';
import { seconds } from '../model/instances';
import { expiresAt, type Expiry } from '../model/snapshots';
import { t, type MessageKey } from '@/shared/i18n';

const EXPIRIES: Record<Expiry, MessageKey> = {
  never: 'incus.snapshots.expiry.never',
  '1d': 'incus.snapshots.expiry.1d',
  '7d': 'incus.snapshots.expiry.7d',
  '30d': 'incus.snapshots.expiry.30d',
};

/** Takes a snapshot: named (or `snapN`), stateful when the Instance runs and allows it. */
function TakeSnapshotDialog({
  hostId,
  project,
  instance,
  statefulAllowed,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance;
  statefulAllowed: boolean;
  open: boolean;
  onClose: () => void;
}) {
  const { take } = useSnapshots(hostId, project, instance.name);
  const [name, setName] = useState('');
  const [stateful, setStateful] = useState(false);
  const [expiry, setExpiry] = useState<Expiry>('never');
  const canStateful = statefulAllowed && instance.status === 'running';
  const close = () => {
    setName('');
    setStateful(false);
    setExpiry('never');
    take.reset();
    onClose();
  };
  return (
    <Dialog open={open} onOpenChange={(next) => !next && close()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('incus.snapshots.take.title', { name: instance.name })}</DialogTitle>
          <DialogDescription>{t('incus.snapshots.take.description')}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          <div className="grid gap-1.5">
            <Label htmlFor="snapshot-name">{t('incus.snapshots.take.name')}</Label>
            <Input
              id="snapshot-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t('incus.snapshots.take.namePlaceholder')}
            />
          </div>
          <div className="grid gap-1.5">
            <Label>{t('incus.snapshots.take.expiry')}</Label>
            <Select value={expiry} onValueChange={(v) => setExpiry(v as Expiry)}>
              <SelectTrigger className="w-full" aria-label={t('incus.snapshots.take.expiry')}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {(Object.keys(EXPIRIES) as Expiry[]).map((e) => (
                  <SelectItem key={e} value={e}>
                    {t(EXPIRIES[e])}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          {canStateful && (
            <div className="flex items-center gap-2">
              <Switch id="snapshot-stateful" checked={stateful} onCheckedChange={setStateful} />
              <Label htmlFor="snapshot-stateful">{t('incus.snapshots.take.stateful')}</Label>
            </div>
          )}
          {take.error && <ErrorMessage error={take.error} />}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={close}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={take.isPending}
            onClick={() =>
              take
                .mutateAsync({
                  name: name.trim(),
                  stateful: canStateful && stateful,
                  expiresAt: expiresAt(expiry),
                })
                .then(close)
                .catch(() => {})
            }
          >
            {t('incus.snapshots.take')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/** An Instance's snapshots: take one, restore the Instance to one, delete them. */
export function SnapshotsTab({
  hostId,
  project,
  instance,
  statefulAllowed,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance;
  statefulAllowed: boolean;
}) {
  const snapshots = useQuery(incusInstanceQueries.snapshots(hostId, project, instance.name));
  const { restore, remove } = useSnapshots(hostId, project, instance.name);
  const [dialog, confirm] = useConfirmedAction();
  const [taking, setTaking] = useState(false);

  if (snapshots.isPending) return <Spinner />;
  if (snapshots.error)
    return <ErrorMessage error={snapshots.error} onRetry={() => void snapshots.refetch()} />;

  const columns: ColumnDef<IncusSnapshot, unknown>[] = [
    {
      id: 'name',
      header: t('incus.snapshots.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.stateful && (
            <Badge variant="outline">{t('incus.snapshots.stateful')}</Badge>
          )}
        </span>
      ),
    },
    {
      id: 'created',
      header: t('incus.snapshots.column.created'),
      accessorFn: (s) => seconds(s.createdAt),
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {formatRelative(seconds(row.original.createdAt))}
        </span>
      ),
    },
    {
      id: 'expires',
      header: t('incus.snapshots.column.expires'),
      accessorFn: (s) => seconds(s.expiresAt),
      cell: ({ row }) => row.original.expiresAt ?? '—',
    },
    {
      id: 'size',
      header: t('incus.snapshots.column.size'),
      accessorFn: (s) => s.size ?? -1,
      cell: ({ row }) => (row.original.size === null ? '—' : formatBytes(row.original.size)),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const snapshot = row.original;
        return (
          <div className="flex justify-end gap-1">
            <Button
              variant="ghost"
              size="sm"
              disabled={restore.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.snapshots.restore.title', {
                    name: instance.name,
                    snapshot: snapshot.name,
                  }),
                  description: t('incus.snapshots.restore.description', {
                    snapshot: snapshot.name,
                    time: snapshot.createdAt,
                  }),
                  confirmLabel: t('incus.snapshots.restore'),
                  destructive: true,
                  action: () =>
                    restore.mutateAsync({ snapshot: snapshot.name, stateful: snapshot.stateful }),
                })
              }
            >
              <HistoryIcon /> {t('incus.snapshots.restore')}
            </Button>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={t('incus.snapshots.deleteOne', { snapshot: snapshot.name })}
              disabled={remove.isPending}
              onClick={() =>
                confirm({
                  title: t('incus.snapshots.delete.title', { snapshot: snapshot.name }),
                  description: t('incus.snapshots.delete.description'),
                  confirmLabel: t('incus.delete'),
                  destructive: true,
                  action: () => remove.mutateAsync(snapshot.name),
                })
              }
            >
              <Trash2Icon />
            </Button>
          </div>
        );
      },
    },
  ];

  return (
    <div className="grid content-start gap-3">
      <div className="flex justify-end">
        <Button onClick={() => setTaking(true)}>
          <CameraIcon /> {t('incus.snapshots.take')}
        </Button>
      </div>
      {snapshots.data.length === 0 ? (
        <EmptyState
          icon={CameraIcon}
          title={t('incus.snapshots.empty')}
          description={t('incus.snapshots.emptyDescription')}
        />
      ) : (
        <DataTable columns={columns} data={snapshots.data} getRowId={(s) => s.name} />
      )}
      <TakeSnapshotDialog
        hostId={hostId}
        project={project}
        instance={instance}
        statefulAllowed={statefulAllowed}
        open={taking}
        onClose={() => setTaking(false)}
      />
      {dialog}
    </div>
  );
}
