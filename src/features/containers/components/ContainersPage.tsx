import { openUrl } from '@tauri-apps/plugin-opener';
import {
  BoxIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  PlayIcon,
  PlusIcon,
  RotateCwIcon,
  SquareIcon,
  Trash2Icon,
} from 'lucide-react';
import { Fragment, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { hostAddress, HostGate, useContainers, useHostSelection } from '@/core/host';
import { useContainerMutations, useUsage } from '../api';
import type { Container, ContainerUsage } from '@/shared/api';
import { formatBytes, formatPercent, formatRelative, shortId } from '@/shared/lib';
import {
  Badge,
  Button,
  Checkbox,
  containerTone,
  EmptyState,
  ErrorMessage,
  Label,
  PageHeader,
  SearchInput,
  Spinner,
  StatusDot,
  Switch,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  toastError,
  useConfirm,
} from '@/shared/ui';
import { groupByStack, portLabels } from '../grouping';
import { ContainerMenu, removeConfirmation } from './ContainerMenu';
import { RenameDialog } from './RenameDialog';
import { RunContainerDialog } from './RunContainerDialog';

function UsageCells({ usage }: { usage?: ContainerUsage }) {
  return (
    <>
      <TableCell className="text-right tabular-nums">
        {usage ? formatPercent(usage.cpuPercent ?? 0) : '—'}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {usage ? formatBytes(usage.memoryUsage) : '—'}
      </TableCell>
    </>
  );
}

function ContainersList({ hostId }: { hostId: string }) {
  const navigate = useNavigate();
  const { host } = useHostSelection();
  const containers = useContainers(hostId);
  const usage = useUsage(hostId);
  const { act, remove } = useContainerMutations(hostId);
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [onlyRunning, setOnlyRunning] = useState(false);
  const [collapsed, setCollapsed] = useState(new Set<string>());
  const [selected, setSelected] = useState(new Set<string>());
  const [renaming, setRenaming] = useState<Container | null>(null);
  const [running, setRunning] = useState(false);

  if (containers.isPending) return <Spinner />;
  if (containers.error)
    return <ErrorMessage error={containers.error} onRetry={() => void containers.refetch()} />;

  const all = containers.data;
  const { groups, standalone } = groupByStack(all, query, onlyRunning);
  const address = host ? hostAddress(host.host.connection) : 'localhost';
  const selectedContainers = all.filter((c) => selected.has(c.id));
  const toggle = (ids: string[], on: boolean) =>
    setSelected((s) => {
      const next = new Set(s);
      ids.forEach((id) => (on ? next.add(id) : next.delete(id)));
      return next;
    });

  const row = (c: Container, nested: boolean) => {
    const u = usage.get(c.id);
    return (
      <TableRow
        key={c.id}
        className="cursor-pointer"
        data-state={selected.has(c.id) ? 'selected' : undefined}
        onClick={() => navigate(`/containers/${c.id}/logs`)}
      >
        <TableCell onClick={(e) => e.stopPropagation()}>
          <Checkbox
            aria-label={`Select ${c.name}`}
            checked={selected.has(c.id)}
            onCheckedChange={(v) => toggle([c.id], v === true)}
          />
        </TableCell>
        <TableCell className={nested ? 'pl-8' : undefined}>
          <div className="flex items-center gap-2">
            <StatusDot tone={containerTone(c.state, c.health)} />
            <div className="min-w-0">
              <p className="truncate font-medium">{nested && c.service ? c.service : c.name}</p>
              <p className="text-muted-foreground font-mono text-xs">{shortId(c.id)}</p>
            </div>
          </div>
        </TableCell>
        <TableCell className="max-w-56 truncate">{c.image}</TableCell>
        <TableCell>
          <span className="text-sm">{c.status || c.state}</span>
          {c.health && (
            <Badge
              variant={c.health === 'unhealthy' ? 'destructive' : 'outline'}
              className="ml-1.5"
            >
              {c.health}
            </Badge>
          )}
        </TableCell>
        <TableCell onClick={(e) => e.stopPropagation()}>
          <div className="flex max-w-48 flex-wrap gap-1">
            {portLabels(c)
              .slice(0, 3)
              .map((p) =>
                p.publicPort ? (
                  <button
                    key={p.label}
                    type="button"
                    className="text-primary font-mono text-xs hover:underline"
                    onClick={() => openUrl(`http://${address}:${p.publicPort}`).catch(toastError)}
                  >
                    {p.label}
                  </button>
                ) : (
                  <span key={p.label} className="text-muted-foreground font-mono text-xs">
                    {p.label}
                  </span>
                ),
              )}
          </div>
        </TableCell>
        <UsageCells usage={u} />
        <TableCell className="text-muted-foreground whitespace-nowrap">
          {formatRelative(c.created)}
        </TableCell>
        <TableCell>
          <ContainerMenu hostId={hostId} container={c} onRename={setRenaming} confirm={confirm} />
        </TableCell>
      </TableRow>
    );
  };

  return (
    <div>
      <PageHeader
        title="Containers"
        description={`${all.filter((c) => c.state === 'running').length} running of ${all.length}`}
        actions={
          <Button onClick={() => setRunning(true)}>
            <PlusIcon /> Run container
          </Button>
        }
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search containers" />
        <div className="flex items-center gap-2">
          <Switch id="only-running" checked={onlyRunning} onCheckedChange={setOnlyRunning} />
          <Label htmlFor="only-running">Only running</Label>
        </div>
        {selectedContainers.length > 0 && (
          <div className="ml-auto flex items-center gap-1.5">
            <span className="text-muted-foreground text-sm">
              {selectedContainers.length} selected
            </span>
            <Button
              size="sm"
              variant="outline"
              onClick={() => act.mutate({ ids: [...selected], action: 'start' })}
            >
              <PlayIcon /> Start
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() => act.mutate({ ids: [...selected], action: 'stop' })}
            >
              <SquareIcon /> Stop
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() => act.mutate({ ids: [...selected], action: 'restart' })}
            >
              <RotateCwIcon /> Restart
            </Button>
            <Button
              size="sm"
              variant="destructive"
              onClick={() =>
                confirm(
                  removeConfirmation(selectedContainers, async () => {
                    await remove.mutateAsync({ ids: [...selected], volumes: true });
                    setSelected(new Set());
                  }),
                )
              }
            >
              <Trash2Icon /> Remove
            </Button>
          </div>
        )}
      </div>

      {all.length === 0 ? (
        <EmptyState
          icon={BoxIcon}
          title="No containers"
          description="Run one from an image, or deploy a Stack."
          action={<Button onClick={() => setRunning(true)}>Run container</Button>}
        />
      ) : (
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="w-8" />
                <TableHead>Name</TableHead>
                <TableHead>Image</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Ports</TableHead>
                <TableHead className="text-right">CPU</TableHead>
                <TableHead className="text-right">Memory</TableHead>
                <TableHead>Created</TableHead>
                <TableHead className="w-20" />
              </TableRow>
            </TableHeader>
            <TableBody>
              {groups.map((g) => {
                const open = !collapsed.has(g.stack);
                const ids = g.containers.map((c) => c.id);
                const cpu = g.containers.reduce(
                  (s, c) => s + (usage.get(c.id)?.cpuPercent ?? 0),
                  0,
                );
                const mem = g.containers.reduce(
                  (s, c) => s + (usage.get(c.id)?.memoryUsage ?? 0),
                  0,
                );
                return (
                  <Fragment key={g.stack}>
                    <TableRow
                      className="bg-muted/30 cursor-pointer"
                      onClick={() =>
                        setCollapsed((s) => {
                          const next = new Set(s);
                          if (open) next.add(g.stack);
                          else next.delete(g.stack);
                          return next;
                        })
                      }
                    >
                      <TableCell onClick={(e) => e.stopPropagation()}>
                        <Checkbox
                          aria-label={`Select stack ${g.stack}`}
                          checked={ids.every((id) => selected.has(id))}
                          onCheckedChange={(v) => toggle(ids, v === true)}
                        />
                      </TableCell>
                      <TableCell colSpan={4}>
                        <span className="flex items-center gap-2 font-medium">
                          {open ? (
                            <ChevronDownIcon className="size-4" />
                          ) : (
                            <ChevronRightIcon className="size-4" />
                          )}
                          <StatusDot
                            tone={
                              g.running === 0
                                ? 'muted'
                                : g.running === g.containers.length
                                  ? 'success'
                                  : 'warning'
                            }
                          />
                          {g.stack}
                          <span className="text-muted-foreground text-xs font-normal">
                            {g.running}/{g.containers.length} running
                          </span>
                        </span>
                      </TableCell>
                      <TableCell className="text-right tabular-nums">
                        {g.running ? formatPercent(cpu) : '—'}
                      </TableCell>
                      <TableCell className="text-right tabular-nums">
                        {g.running ? formatBytes(mem) : '—'}
                      </TableCell>
                      <TableCell colSpan={2} className="text-right">
                        <Button
                          variant="link"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation();
                            navigate(`/stacks/${encodeURIComponent(g.stack)}`);
                          }}
                        >
                          Open stack
                        </Button>
                      </TableCell>
                    </TableRow>
                    {open && g.containers.map((c) => row(c, true))}
                  </Fragment>
                );
              })}
              {standalone.map((c) => row(c, false))}
              {groups.length === 0 && standalone.length === 0 && (
                <TableRow>
                  <TableCell colSpan={9} className="text-muted-foreground h-20 text-center">
                    No containers match.
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </div>
      )}
      {renaming && (
        <RenameDialog hostId={hostId} container={renaming} onClose={() => setRenaming(null)} />
      )}
      <RunContainerDialog hostId={hostId} open={running} onOpenChange={setRunning} />
      {dialog}
    </div>
  );
}

export function ContainersPage() {
  return <HostGate>{(hostId) => <ContainersList key={hostId} hostId={hostId} />}</HostGate>;
}
