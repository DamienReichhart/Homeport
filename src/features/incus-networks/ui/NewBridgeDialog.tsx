import { useState } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Switch,
} from '@/shared/ui';
import { useNetworks } from '../api/mutations';
import { validAddress, validNetworkName } from '../model/bridge';
import { t } from '@/shared/i18n';

/** Makes a managed bridge: its name, IPv4 and IPv6 (automatic, none or a subnet) and NAT. */
export function NewBridgeDialog({
  hostId,
  project,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  open: boolean;
  onClose: () => void;
}) {
  const { create } = useNetworks(hostId, project);
  const [name, setName] = useState('');
  const [ipv4, setIpv4] = useState('auto');
  const [ipv6, setIpv6] = useState('auto');
  const [nat, setNat] = useState(true);
  const close = () => {
    setName('');
    setIpv4('auto');
    setIpv6('auto');
    setNat(true);
    create.reset();
    onClose();
  };
  const valid = validNetworkName(name) && validAddress(ipv4, 4) && validAddress(ipv6, 6);
  return (
    <Dialog open={open} onOpenChange={(next) => !next && close()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('incus.networks.new.title')}</DialogTitle>
          <DialogDescription>{t('incus.networks.new.description')}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          <div className="grid gap-1.5">
            <Label htmlFor="bridge-name">{t('incus.networks.new.name')}</Label>
            <Input
              id="bridge-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="lab0"
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="bridge-ipv4">{t('incus.networks.new.ipv4')}</Label>
            <Input id="bridge-ipv4" value={ipv4} onChange={(e) => setIpv4(e.target.value)} />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="bridge-ipv6">{t('incus.networks.new.ipv6')}</Label>
            <Input id="bridge-ipv6" value={ipv6} onChange={(e) => setIpv6(e.target.value)} />
          </div>
          <p className="text-muted-foreground text-xs">{t('incus.networks.new.addressHint')}</p>
          {ipv4.trim() !== 'none' && (
            <div className="flex items-center gap-2">
              <Switch id="bridge-nat" checked={nat} onCheckedChange={setNat} />
              <Label htmlFor="bridge-nat">{t('incus.networks.new.nat')}</Label>
            </div>
          )}
          {create.error && <ErrorMessage error={create.error} />}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={close}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={!valid || create.isPending}
            onClick={() =>
              create
                .mutateAsync({ name: name.trim(), ipv4, ipv6, nat, description: '' })
                .then(close)
                .catch(() => {})
            }
          >
            {t('common.create')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
