import { BoxIcon, PlusIcon } from 'lucide-react';
import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import {
  CheckNewerImages,
  containerFreshness,
  hostAddress,
  HostGate,
  indexFreshness,
  resourceQueries,
  useContainers,
  useHostSelection,
} from '@/core/host';
import type { Container } from '@/shared/api';
import {
  Button,
  EmptyState,
  ErrorMessage,
  Label,
  PageHeader,
  SearchInput,
  Spinner,
  Switch,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  useConfirm,
} from '@/shared/ui';
import { useUsage } from '../../api/streams';
import { groupByStack } from '../../model/grouping';
import { RenameDialog } from '../dialogs/RenameDialog';
import { RunContainerDialog } from '../dialogs/RunContainerDialog';
import { ContainerRow } from './ContainerRow';
import { SelectionActions } from './SelectionActions';
import { StackGroupRows } from './StackGroupRows';

function ContainersList({ hostId }: { hostId: string }) {
  const { host } = useHostSelection();
  const containers = useContainers(hostId);
  const usage = useUsage(hostId);
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);
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
  const toggleOpen = (stack: string) =>
    setCollapsed((s) => {
      const next = new Set(s);
      if (next.has(stack)) next.delete(stack);
      else next.add(stack);
      return next;
    });

  const row = (c: Container, nested: boolean) => (
    <ContainerRow
      key={c.id}
      hostId={hostId}
      container={c}
      nested={nested}
      usage={usage.get(c.id)}
      newer={containerFreshness(freshness, c) === 'newer'}
      address={address}
      selected={selected.has(c.id)}
      onSelect={(on) => toggle([c.id], on)}
      onRename={setRenaming}
      confirm={confirm}
    />
  );

  return (
    <div>
      <PageHeader
        title="Containers"
        description={`${all.filter((c) => c.state === 'running').length} running of ${all.length}`}
        actions={
          <>
            <CheckNewerImages hostId={hostId} />
            <Button onClick={() => setRunning(true)}>
              <PlusIcon /> Run container
            </Button>
          </>
        }
      />
      <div className="mb-3 flex flex-wrap items-center gap-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search containers" />
        <div className="flex items-center gap-2">
          <Switch id="only-running" checked={onlyRunning} onCheckedChange={setOnlyRunning} />
          <Label htmlFor="only-running">Only running</Label>
        </div>
        {selectedContainers.length > 0 && (
          <SelectionActions
            hostId={hostId}
            ids={[...selected]}
            selected={selectedContainers}
            onRemoved={() => setSelected(new Set())}
            confirm={confirm}
          />
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
                const ids = g.containers.map((c) => c.id);
                return (
                  <StackGroupRows
                    key={g.stack}
                    group={g}
                    open={!collapsed.has(g.stack)}
                    onToggleOpen={() => toggleOpen(g.stack)}
                    usage={usage}
                    allSelected={ids.every((id) => selected.has(id))}
                    onSelect={(on) => toggle(ids, on)}
                  >
                    {g.containers.map((c) => row(c, true))}
                  </StackGroupRows>
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
