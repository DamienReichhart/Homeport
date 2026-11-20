import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import type { Image } from '@/shared/api';
import { formatBytes, formatDateTime, formatRelative } from '@/shared/lib';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  JsonView,
  KeyValue,
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
import { t } from '@/shared/i18n';

/** An image's facts, its layers and its raw JSON. */
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
          <DialogTitle className="font-mono">{imageName(image)}</DialogTitle>
        </DialogHeader>
        <KeyValue
          items={[
            [t('images.column.id'), <span className="font-mono text-xs">{image.id}</span>],
            ...(image.tags.length > 1
              ? [
                  [
                    t('images.inspect.names'),
                    <span className="font-mono text-xs">{image.tags.join(', ')}</span>,
                  ] as [string, ReactNode],
                ]
              : []),
            [t('images.column.size'), formatBytes(image.size)],
            [
              t('images.column.created'),
              <span title={formatDateTime(image.created)}>{formatRelative(image.created)}</span>,
            ],
            [
              t('images.column.status'),
              image.containers > 0
                ? t('images.inUse', { count: image.containers })
                : t('images.unused'),
            ],
          ]}
        />
        <Tabs defaultValue="layers">
          <TabsList>
            <TabsTrigger value="layers">{t('images.inspect.layers')}</TabsTrigger>
            <TabsTrigger value="raw">{t('images.inspect.raw')}</TabsTrigger>
          </TabsList>
          <TabsContent value="layers">
            {history.data ? (
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('images.inspect.created')}</TableHead>
                    <TableHead>{t('images.inspect.instruction')}</TableHead>
                    <TableHead className="text-end">{t('images.inspect.size')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {history.data.length === 0 && (
                    <TableRow>
                      <TableCell colSpan={3} className="text-muted-foreground h-16 text-center">
                        {t('ui.table.empty')}
                      </TableCell>
                    </TableRow>
                  )}
                  {history.data.map((l, i) => (
                    <TableRow key={i}>
                      <TableCell className="text-muted-foreground whitespace-nowrap">
                        {formatRelative(l.created)}
                      </TableCell>
                      <TableCell className="max-w-xl font-mono text-xs break-all whitespace-normal">
                        {l.createdBy}
                      </TableCell>
                      <TableCell className="text-end whitespace-nowrap">
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
