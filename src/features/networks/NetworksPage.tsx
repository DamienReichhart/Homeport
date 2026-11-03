import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  BrushCleaningIcon,
  InfoIcon,
  NetworkIcon,
  PlusIcon,
  Trash2Icon,
  UnplugIcon,
} from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { useContainers } from '@/features/containers';
import { HostGate } from '@/features/hosts';
import { commands, keys, type NetworkInput, type NetworkView } from '@/shared/api';
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
  EmptyState,
  ErrorMessage,
  Input,
  JsonView,
  Label,
  PageHeader,
  SearchInput,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Switch,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  toastError,
  useConfirm,
} from '@/shared/ui';

const emptyNetwork: NetworkInput = {
  name: '',
  driver: null,
  subnet: null,
  gateway: null,
  internal: false,
  attachable: false,
};

function CreateNetworkDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (o: boolean) => void;
}) {
  const client = useQueryClient();
  const [input, setInput] = useState<NetworkInput>(emptyNetwork);
  const create = useMutation({
    meta: { silent: true },
    mutationFn: () => commands.networksCreate(hostId, input),
    onSuccess: async () => {
      await client.invalidateQueries({ queryKey: keys.networks(hostId) });
      setInput(emptyNetwork);
      onOpenChange(false);
    },
  });
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create a network</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            create.mutate();
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="net-name">Name</Label>
            <Input
              id="net-name"
              value={input.name}
              onChange={(e) => setInput({ ...input, name: e.target.value })}
              autoFocus
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div className="grid gap-1.5">
              <Label htmlFor="net-driver">Driver</Label>
              <Input
                id="net-driver"
                placeholder="bridge"
                value={input.driver ?? ''}
                onChange={(e) => setInput({ ...input, driver: e.target.value || null })}
              />
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="net-subnet">Subnet</Label>
              <Input
                id="net-subnet"
                placeholder="automatic"
                value={input.subnet ?? ''}
                onChange={(e) => setInput({ ...input, subnet: e.target.value || null })}
              />
            </div>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="net-gw">Gateway</Label>
            <Input
              id="net-gw"
              placeholder="automatic"
              value={input.gateway ?? ''}
              onChange={(e) => setInput({ ...input, gateway: e.target.value || null })}
            />
          </div>
          <div className="flex items-center gap-2">
            <Switch
              id="net-internal"
              checked={input.internal}
              onCheckedChange={(v) => setInput({ ...input, internal: v })}
            />
            <Label htmlFor="net-internal">Internal — no access to outside networks</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch
              id="net-attachable"
              checked={input.attachable}
              onCheckedChange={(v) => setInput({ ...input, attachable: v })}
            />
            <Label htmlFor="net-attachable">Attachable</Label>
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={create.isPending || !input.name.trim()}>
              Create
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

function NetworkDialog({
  hostId,
  network,
  onClose,
}: {
  hostId: string;
  network: NetworkView;
  onClose: () => void;
}) {
  const client = useQueryClient();
  const containers = useContainers(hostId);
  const inspect = useQuery({
    queryKey: [...keys.networks(hostId), network.id],
    queryFn: () => commands.networksInspect(hostId, network.id),
  });
  const [joining, setJoining] = useState('');
  const refresh = () => client.invalidateQueries({ queryKey: keys.networks(hostId) });
  const connect = useMutation({
    mutationFn: (container: string) => commands.networksConnect(hostId, network.id, container),
    onSuccess: () => {
      setJoining('');
      void refresh();
      onClose();
      toast.success('Connected');
    },
    onError: (e) => toastError(e),
  });
  const disconnect = useMutation({
    mutationFn: (container: string) =>
      commands.networksDisconnect(hostId, network.id, container, false),
    onSuccess: () => {
      void refresh();
      onClose();
      toast.success('Disconnected');
    },
    onError: (e) => toastError(e),
  });
  const candidates = containers.data?.filter((c) => !network.containers.includes(c.name)) ?? [];

  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{network.name}</DialogTitle>
        </DialogHeader>
        <Tabs defaultValue="containers">
          <TabsList>
            <TabsTrigger value="containers">Containers</TabsTrigger>
            <TabsTrigger value="raw">Raw JSON</TabsTrigger>
          </TabsList>
          <TabsContent value="containers" className="grid gap-3">
            {network.containers.length === 0 && (
              <p className="text-muted-foreground text-sm">No container is attached.</p>
            )}
            {network.containers.map((name) => (
              <div
                key={name}
                className="flex items-center justify-between rounded-md border px-3 py-1.5"
              >
                <span className="text-sm">{name}</span>
                {!network.builtIn && (
                  <Button variant="ghost" size="sm" onClick={() => disconnect.mutate(name)}>
                    <UnplugIcon /> Disconnect
                  </Button>
                )}
              </div>
            ))}
            {!network.builtIn && network.name !== 'host' && (
              <div className="flex gap-2">
                <Select value={joining} onValueChange={setJoining}>
                  <SelectTrigger className="w-72" aria-label="Container to connect">
                    <SelectValue placeholder="Connect a container…" />
                  </SelectTrigger>
                  <SelectContent>
                    {candidates.map((c) => (
                      <SelectItem key={c.id} value={c.name}>
                        {c.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <Button
                  disabled={!joining || connect.isPending}
                  onClick={() => connect.mutate(joining)}
                >
                  Connect
                </Button>
              </div>
            )}
          </TabsContent>
          <TabsContent value="raw">
            {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
          </TabsContent>
        </Tabs>
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            Close
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function NetworksList({ hostId }: { hostId: string }) {
  const client = useQueryClient();
  const networks = useQuery({
    queryKey: keys.networks(hostId),
    queryFn: () => commands.networksList(hostId),
  });
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [open, setOpen] = useState<NetworkView | null>(null);
  const refresh = () => client.invalidateQueries({ queryKey: keys.networks(hostId) });

  const remove = (n: NetworkView) =>
    confirm({
      title: `Remove network ${n.name}?`,
      description: n.containers.length
        ? `It still has ${n.containers.length} attached container(s); disconnect them first.`
        : 'No container uses it. Containers that reference it by name will fail to start.',
      confirmLabel: 'Remove',
      destructive: true,
      onConfirm: async () => {
        try {
          await commands.networksRemove(hostId, n.id);
          toast.success(`Removed ${n.name}`);
        } catch (e) {
          toastError(e);
        }
        await refresh();
      },
    });

  const columns: ColumnDef<NetworkView, unknown>[] = [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (n) => n.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.builtIn && <Badge variant="secondary">built-in</Badge>}
          {row.original.internal && <Badge variant="outline">internal</Badge>}
        </span>
      ),
    },
    {
      id: 'stack',
      header: 'Stack',
      accessorFn: (n) => n.stack ?? '',
      cell: ({ row }) =>
        row.original.stack && <Badge variant="outline">{row.original.stack}</Badge>,
    },
    { id: 'driver', header: 'Driver', accessorFn: (n) => n.driver },
    {
      id: 'subnet',
      header: 'Subnet',
      accessorFn: (n) => n.subnets.join(', '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.subnets.join(', ') || '—'}</span>
      ),
    },
    { id: 'containers', header: 'Containers', accessorFn: (n) => n.containers.length },
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
            onClick={() => setOpen(row.original)}
          >
            <InfoIcon />
          </Button>
          {!row.original.builtIn && (
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label={`Remove ${row.original.name}`}
              onClick={() => remove(row.original)}
            >
              <Trash2Icon />
            </Button>
          )}
        </div>
      ),
    },
  ];

  if (networks.isPending) return <Spinner />;
  if (networks.error)
    return <ErrorMessage error={networks.error} onRetry={() => void networks.refetch()} />;
  return (
    <div>
      <PageHeader
        title="Networks"
        description={`${networks.data.length} networks`}
        actions={
          <>
            <Button
              variant="outline"
              onClick={() =>
                confirm({
                  title: 'Remove unused networks?',
                  description: 'Every custom network no container is attached to is removed.',
                  confirmLabel: 'Clean up',
                  destructive: true,
                  onConfirm: async () => {
                    const r = await commands.networksPrune(hostId);
                    toast.success(`Removed ${r.removed.length} networks`);
                    await refresh();
                  },
                })
              }
            >
              <BrushCleaningIcon /> Clean up
            </Button>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> Create network
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search networks" />
      </div>
      {networks.data.length === 0 ? (
        <EmptyState icon={NetworkIcon} title="No networks" />
      ) : (
        <DataTable
          columns={columns}
          data={networks.data}
          getRowId={(n) => n.id}
          filter={query}
          onRowClick={setOpen}
        />
      )}
      <CreateNetworkDialog hostId={hostId} open={creating} onOpenChange={setCreating} />
      {open && <NetworkDialog hostId={hostId} network={open} onClose={() => setOpen(null)} />}
      {dialog}
    </div>
  );
}

export function NetworksPage() {
  return <HostGate>{(hostId) => <NetworksList key={hostId} hostId={hostId} />}</HostGate>;
}
