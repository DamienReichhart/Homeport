import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import {
  BrushCleaningIcon,
  DownloadIcon,
  EllipsisVerticalIcon,
  InfoIcon,
  LayersIcon,
  PlayIcon,
  TagIcon,
  Trash2Icon,
} from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { RunContainerDialog } from '@/features/containers';
import { HostGate } from '@/features/hosts';
import { commands, keys, toAppError, type Image } from '@/shared/api';
import { formatBytes, formatRelative, shortId } from '@/shared/lib';
import { Badge } from '@/shared/ui/badge';
import { Button } from '@/shared/ui/button';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '@/shared/ui/dialog';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui/dropdown-menu';
import { Input } from '@/shared/ui/input';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/shared/ui/table';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/shared/ui/tabs';
import {
  DataTable,
  EmptyState,
  ErrorMessage,
  JsonView,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirm,
  type ColumnDef,
} from '@/shared/ui';
import { PullDialog } from './PullDialog';

const imageName = (i: Image) => i.tags[0] ?? '<none>';

function TagDialog({
  hostId,
  image,
  onClose,
}: {
  hostId: string;
  image: Image;
  onClose: () => void;
}) {
  const client = useQueryClient();
  const [target, setTarget] = useState('');
  const tag = useMutation({
    mutationFn: () => commands.imagesTag(hostId, image.id, target),
    onSuccess: async () => {
      await client.invalidateQueries({ queryKey: keys.images(hostId) });
      onClose();
    },
  });
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Tag {imageName(image)}</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            tag.mutate();
          }}
        >
          <Input
            value={target}
            onChange={(e) => setTarget(e.target.value)}
            placeholder="registry.example.com/app:v2"
            autoFocus
            aria-label="New name"
          />
          {tag.error && <ErrorMessage error={toAppError(tag.error)} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit" disabled={tag.isPending || !target.trim()}>
              Add tag
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

function InspectDialog({
  hostId,
  image,
  onClose,
}: {
  hostId: string;
  image: Image;
  onClose: () => void;
}) {
  const history = useQuery({
    queryKey: [...keys.images(hostId), image.id, 'history'],
    queryFn: () => commands.imagesHistory(hostId, image.id),
  });
  const inspect = useQuery({
    queryKey: [...keys.images(hostId), image.id, 'inspect'],
    queryFn: () => commands.imagesInspect(hostId, image.id),
  });
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>{imageName(image)}</DialogTitle>
        </DialogHeader>
        <Tabs defaultValue="layers">
          <TabsList>
            <TabsTrigger value="layers">Layers</TabsTrigger>
            <TabsTrigger value="raw">Raw JSON</TabsTrigger>
          </TabsList>
          <TabsContent value="layers">
            {history.data ? (
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Created</TableHead>
                    <TableHead>Instruction</TableHead>
                    <TableHead className="text-right">Size</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {history.data.map((l, i) => (
                    <TableRow key={i}>
                      <TableCell className="text-muted-foreground whitespace-nowrap">
                        {formatRelative(l.created)}
                      </TableCell>
                      <TableCell className="max-w-xl font-mono text-xs break-all whitespace-normal">
                        {l.createdBy}
                      </TableCell>
                      <TableCell className="text-right whitespace-nowrap">
                        {formatBytes(l.size)}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            ) : (
              <Spinner />
            )}
          </TabsContent>
          <TabsContent value="raw">
            {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  );
}

function ImagesList({ hostId }: { hostId: string }) {
  const client = useQueryClient();
  const images = useQuery({
    queryKey: keys.images(hostId),
    queryFn: () => commands.imagesList(hostId),
  });
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [pulling, setPulling] = useState(false);
  const [running, setRunning] = useState<Image | null>(null);
  const [tagging, setTagging] = useState<Image | null>(null);
  const [inspecting, setInspecting] = useState<Image | null>(null);
  const refresh = () => client.invalidateQueries({ queryKey: keys.images(hostId) });

  const removeImage = (image: Image) =>
    confirm({
      title: `Remove ${imageName(image)}?`,
      description:
        image.tags.length > 1
          ? `This removes every name of the image (${image.tags.join(', ')}) and its layers that nothing else uses.`
          : 'The image and the layers no other image uses are deleted. It can be pulled again later.',
      confirmLabel: 'Remove',
      destructive: true,
      onConfirm: async () => {
        try {
          await commands.imagesRemove(hostId, image.id, image.tags.length > 1);
          toast.success(`Removed ${imageName(image)}`);
        } catch (e) {
          toast.error(toAppError(e).message);
        }
        await refresh();
      },
    });

  const prune = (all: boolean) =>
    confirm({
      title: all ? 'Remove all unused images?' : 'Remove dangling images?',
      description: all
        ? 'Every image no container uses is deleted, tagged ones included.'
        : 'Untagged images left behind by builds and pulls are deleted.',
      confirmLabel: 'Clean up',
      destructive: true,
      onConfirm: async () => {
        const r = await commands.imagesPrune(hostId, all);
        toast.success(
          `Removed ${r.removed.length} images, ${formatBytes(r.spaceReclaimed)} reclaimed`,
        );
        await refresh();
      },
    });

  const columns: ColumnDef<Image, unknown>[] = [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (i) => i.tags.join(' ') || '<none>',
      cell: ({ row }) => (
        <div className="min-w-0">
          <p className="truncate font-medium">{imageName(row.original)}</p>
          {row.original.tags.length > 1 && (
            <p className="text-muted-foreground text-xs">
              +{row.original.tags.length - 1} more tags
            </p>
          )}
        </div>
      ),
    },
    {
      id: 'id',
      header: 'ID',
      accessorFn: (i) => i.id,
      cell: ({ row }) => <span className="font-mono text-xs">{shortId(row.original.id)}</span>,
    },
    {
      id: 'created',
      header: 'Created',
      accessorFn: (i) => i.created,
      cell: ({ row }) => formatRelative(row.original.created),
    },
    {
      id: 'size',
      header: 'Size',
      accessorFn: (i) => i.size,
      cell: ({ row }) => formatBytes(row.original.size),
    },
    {
      id: 'usage',
      header: 'Status',
      accessorFn: (i) => i.containers,
      cell: ({ row }) =>
        row.original.containers > 0 ? (
          <Badge>In use · {row.original.containers}</Badge>
        ) : (
          <Badge variant="outline">Unused</Badge>
        ),
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
            aria-label={`Run ${imageName(row.original)}`}
            onClick={() => setRunning(row.original)}
          >
            <PlayIcon />
          </Button>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" size="icon-sm" aria-label="More actions">
                <EllipsisVerticalIcon />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onSelect={() => setInspecting(row.original)}>
                <InfoIcon /> Inspect
              </DropdownMenuItem>
              <DropdownMenuItem onSelect={() => setTagging(row.original)}>
                <TagIcon /> Tag
              </DropdownMenuItem>
              <DropdownMenuSeparator />
              <DropdownMenuItem variant="destructive" onSelect={() => removeImage(row.original)}>
                <Trash2Icon /> Remove
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </div>
      ),
    },
  ];

  if (images.isPending) return <Spinner />;
  if (images.error) return <ErrorMessage error={toAppError(images.error)} />;
  const total = images.data.reduce((s, i) => s + i.size, 0);

  return (
    <div>
      <PageHeader
        title="Images"
        description={`${images.data.length} images · ${formatBytes(total)}`}
        actions={
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> Clean up
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onSelect={() => prune(false)}>Dangling images</DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  All unused images
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            <Button onClick={() => setPulling(true)}>
              <DownloadIcon /> Pull image
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search images" />
      </div>
      {images.data.length === 0 ? (
        <EmptyState
          icon={LayersIcon}
          title="No images"
          description="Pull one from a registry to get started."
          action={<Button onClick={() => setPulling(true)}>Pull image</Button>}
        />
      ) : (
        <DataTable
          columns={columns}
          data={images.data}
          getRowId={(i) => i.id}
          filter={query}
          onRowClick={setInspecting}
        />
      )}
      <PullDialog hostId={hostId} open={pulling} onOpenChange={setPulling} />
      {running && (
        <RunContainerDialog
          key={running.id}
          hostId={hostId}
          image={running.tags[0] ?? running.id}
          open
          onOpenChange={(open) => !open && setRunning(null)}
        />
      )}
      {tagging && <TagDialog hostId={hostId} image={tagging} onClose={() => setTagging(null)} />}
      {inspecting && (
        <InspectDialog hostId={hostId} image={inspecting} onClose={() => setInspecting(null)} />
      )}
      {dialog}
    </div>
  );
}

export function ImagesPage() {
  return <HostGate>{(hostId) => <ImagesList key={hostId} hostId={hostId} />}</HostGate>;
}
