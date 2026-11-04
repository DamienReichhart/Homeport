import { useQuery } from '@tanstack/react-query';
import { UnplugIcon } from 'lucide-react';
import { useState } from 'react';
import { useContainers } from '@/core/host';
import type { NetworkView } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  JsonView,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { useNetworkMembership } from '../../api/mutations';
import { networkQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/** A network's containers, with connect and disconnect, and its raw JSON. */
export function NetworkDialog({
  hostId,
  network,
  onClose,
}: {
  hostId: string;
  network: NetworkView;
  onClose: () => void;
}) {
  const containers = useContainers(hostId);
  const inspect = useQuery(networkQueries.inspect(hostId, network.id));
  const [joining, setJoining] = useState('');
  const { connect, disconnect } = useNetworkMembership(hostId, network.id, () => {
    setJoining('');
    onClose();
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
            <TabsTrigger value="containers">{t('networks.dialog.containers')}</TabsTrigger>
            <TabsTrigger value="raw">{t('networks.dialog.raw')}</TabsTrigger>
          </TabsList>
          <TabsContent value="containers" className="grid gap-3">
            {network.containers.length === 0 && (
              <p className="text-muted-foreground text-sm">{t('networks.dialog.noContainer')}</p>
            )}
            {network.containers.map((name) => (
              <div
                key={name}
                className="flex items-center justify-between rounded-md border px-3 py-1.5"
              >
                <span className="text-sm">{name}</span>
                {!network.builtIn && (
                  <Button variant="ghost" size="sm" onClick={() => disconnect.mutate(name)}>
                    <UnplugIcon /> {t('networks.dialog.disconnect')}
                  </Button>
                )}
              </div>
            ))}
            {!network.builtIn && network.name !== 'host' && (
              <div className="flex gap-2">
                <Select value={joining} onValueChange={setJoining}>
                  <SelectTrigger
                    className="w-72"
                    aria-label={t('networks.dialog.containerToConnect')}
                  >
                    <SelectValue placeholder={t('networks.dialog.connectPlaceholder')} />
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
                  {t('networks.dialog.connect')}
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
            {t('common.close')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
