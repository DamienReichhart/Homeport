import { useState } from 'react';
import type { NetworkInput } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Switch,
} from '@/shared/ui';
import { useCreateNetwork } from '../../api/mutations';
import { emptyNetwork } from '../../model/networkInput';
import { t } from '@/shared/i18n';

export function CreateNetworkDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (o: boolean) => void;
}) {
  const [input, setInput] = useState<NetworkInput>(emptyNetwork);
  const create = useCreateNetwork(hostId, () => {
    setInput(emptyNetwork);
    onOpenChange(false);
  });
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('networks.createDialog.title')}</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            create.mutate(input);
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="net-name">{t('networks.createDialog.name')}</Label>
            <Input
              id="net-name"
              value={input.name}
              onChange={(e) => setInput({ ...input, name: e.target.value })}
              autoFocus
            />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div className="grid gap-1.5">
              <Label htmlFor="net-driver">{t('networks.createDialog.driver')}</Label>
              <Input
                id="net-driver"
                placeholder="bridge"
                value={input.driver ?? ''}
                onChange={(e) => setInput({ ...input, driver: e.target.value || null })}
              />
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="net-subnet">{t('networks.createDialog.subnet')}</Label>
              <Input
                id="net-subnet"
                placeholder={t('networks.createDialog.automatic')}
                value={input.subnet ?? ''}
                onChange={(e) => setInput({ ...input, subnet: e.target.value || null })}
              />
            </div>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="net-gw">{t('networks.createDialog.gateway')}</Label>
            <Input
              id="net-gw"
              placeholder={t('networks.createDialog.automatic')}
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
            <Label htmlFor="net-internal">{t('networks.createDialog.internal')}</Label>
          </div>
          <div className="flex items-center gap-2">
            <Switch
              id="net-attachable"
              checked={input.attachable}
              onCheckedChange={(v) => setInput({ ...input, attachable: v })}
            />
            <Label htmlFor="net-attachable">{t('networks.createDialog.attachable')}</Label>
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={create.isPending || !input.name.trim()}>
              {t('common.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
