import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { BrushCleaningIcon, HardDriveIcon, InfoIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { HostGate } from '@/features/hosts';
import { commands, keys, type VolumeView } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import {
  Badge,
  Button,
  type ColumnDef,
  DataTable,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  EmptyState,
  ErrorMessage,
  Input,
  JsonView,
  Label,
  PageHeader,
  SearchInput,
  Spinner,
  toastError,
  useConfirm,
} from '@/shared/ui';

function CreateVolumeDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (o: boolean) => void;
}) {
  const client = useQueryClient();
  const [name, setName] = useState('');
  const [driver, setDriver] = useState('');
  const create = useMutation({
    meta: { silent: true },
    mutationFn: () => commands.volumesCreate(hostId, { name, driver: driver || null, labels: {} }),
    onSuccess: async () => {
      await client.invalidateQueries({ queryKey: keys.volumes(hostId) });
      setName('');
      onOpenChange(false);
    },
  });
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create a volume</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            create.mutate();
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="vol-name">Name</Label>
            <Input id="vol-name" value={name} onChange={(e) => setName(e.target.value)} autoFocus />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="vol-driver">Driver</Label>
            <Input
              id="vol-driver"
              value={driver}
              onChange={(e) => setDriver(e.target.value)}
              placeholder="local"
            />
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={create.isPending || !name.trim()}>
              Create
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

function InspectVolume({
  hostId,
  volume,
  onClose,
}: {
  hostId: string;
  volume: VolumeView;
  onClose: () => void;
}) {
  const inspect = useQuery({
    queryKey: [...keys.volumes(hostId), volume.name],
    queryFn: () => commands.volumesInspect(hostId, volume.name),
  });
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{volume.name}</DialogTitle>
        </DialogHeader>
        <p className="text-sm">
          {volume.usedBy.length
            ? `Used by ${volume.usedBy.join(', ')}.`
            : 'No container uses this volume.'}
        </p>
        {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
      </DialogContent>
    </Dialog>
  );
}

function VolumesList({ hostId }: { hostId: string }) {
  const client = useQueryClient();
  const volumes = useQuery({
    queryKey: keys.volumes(hostId),
    queryFn: () => commands.volumesList(hostId),
  });
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [inspecting, setInspecting] = useState<VolumeView | null>(null);
  const refresh = () => client.invalidateQueries({ queryKey: keys.volumes(hostId) });

  const remove = (v: VolumeView) =>
    confirm({
      title: `Delete volume ${v.name}?`,
      description: (
        <>
          <strong>All data in this volume is permanently deleted.</strong>
          {v.usedBy.length > 0 &&
            ` It is mounted by ${v.usedBy.join(', ')}; remove those containers first.`}
        </>
      ),
      confirmLabel: 'Delete volume and data',
      destructive: true,
      onConfirm: async () => {
        try {
          await commands.volumesRemove(hostId, v.name, false);
          toast.success(`Deleted ${v.name}`);
        } catch (e) {
          toastError(e);
        }
        await refresh();
      },
    });

  const prune = (all: boolean) =>
    confirm({
      title: all ? 'Delete every unused volume?' : 'Delete unused anonymous volumes?',
      description: (
        <strong>
          {all
            ? 'The data in every volume no container uses is permanently deleted, named volumes included.'
            : 'The data in unnamed volumes no container uses is permanently deleted.'}
        </strong>
      ),
      confirmLabel: 'Delete volumes and data',
      destructive: true,
      onConfirm: async () => {
        const r = await commands.volumesPrune(hostId, all);
        toast.success(
          `Deleted ${r.removed.length} volumes, ${formatBytes(r.spaceReclaimed)} reclaimed`,
        );
        await refresh();
      },
    });

  const columns: ColumnDef<VolumeView, unknown>[] = [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (v) => v.name,
      cell: ({ row }) => <span className="font-medium break-all">{row.original.name}</span>,
    },
    {
      id: 'stack',
      header: 'Stack',
      accessorFn: (v) => v.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: 'Driver', accessorFn: (v) => v.driver },
    {
      id: 'usedBy',
      header: 'Used by',
      accessorFn: (v) => v.usedBy.join(' '),
      cell: ({ row }) =>
        row.original.usedBy.length ? (
          row.original.usedBy.join(', ')
        ) : (
          <Badge variant="outline">Unused</Badge>
        ),
    },
    {
      id: 'created',
      header: 'Created',
      accessorFn: (v) => v.createdAt ?? '',
      cell: ({ row }) =>
        row.original.createdAt ? new Date(row.original.createdAt).toLocaleDateString() : '—',
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={`Inspect ${row.original.name}`}
            onClick={() => setInspecting(row.original)}
          >
            <InfoIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={`Delete ${row.original.name}`}
            onClick={() => remove(row.original)}
          >
            <Trash2Icon />
          </Button>
        </div>
      ),
    },
  ];

  if (volumes.isPending) return <Spinner />;
  if (volumes.error)
    return <ErrorMessage error={volumes.error} onRetry={() => void volumes.refetch()} />;
  return (
    <div>
      <PageHeader
        title="Volumes"
        description={`${volumes.data.length} volumes · ${volumes.data.filter((v) => v.usedBy.length === 0).length} unused`}
        actions={
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> Clean up
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem variant="destructive" onSelect={() => prune(false)}>
                  Unused anonymous volumes
                </DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  All unused volumes
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> Create volume
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search volumes" />
      </div>
      {volumes.data.length === 0 ? (
        <EmptyState
          icon={HardDriveIcon}
          title="No volumes"
          description="Volumes keep container data across restarts and re-creations."
        />
      ) : (
        <DataTable
          columns={columns}
          data={volumes.data}
          getRowId={(v) => v.name}
          filter={query}
          onRowClick={setInspecting}
        />
      )}
      <CreateVolumeDialog hostId={hostId} open={creating} onOpenChange={setCreating} />
      {inspecting && (
        <InspectVolume hostId={hostId} volume={inspecting} onClose={() => setInspecting(null)} />
      )}
      {dialog}
    </div>
  );
}

export function VolumesPage() {
  return <HostGate>{(hostId) => <VolumesList key={hostId} hostId={hostId} />}</HostGate>;
}
