import { useQuery } from '@tanstack/react-query';
import type { Image } from '@/shared/api';
import { formatBytes, formatRelative } from '@/shared/lib';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  JsonView,
  Spinner,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { imageQueries } from '../../api/queries';
import { imageName } from '../../model/imageName';

/** An image's layers and its raw JSON. */
export function InspectDialog({
  hostId,
  image,
  onClose,
}: {
  hostId: string;
  image: Image;
  onClose: () => void;
}) {
  const history = useQuery(imageQueries.history(hostId, image.id));
  const inspect = useQuery(imageQueries.inspect(hostId, image.id));
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
