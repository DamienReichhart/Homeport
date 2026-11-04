import { PlusIcon, XIcon } from 'lucide-react';
import {
  Button,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui';
import type { PortRow } from '../../../model/runForm';
import { t } from '@/shared/i18n';

/** The published ports of the Run dialog, one row each. */
export function PortsField({
  ports,
  onChange,
}: {
  ports: PortRow[];
  onChange: (ports: PortRow[]) => void;
}) {
  const update = (i: number, patch: Partial<PortRow>) =>
    onChange(ports.map((x, j) => (j === i ? { ...x, ...patch } : x)));
  return (
    <div className="grid gap-1.5">
      <Label>{t('containers.run.ports')}</Label>
      {ports.map((p, i) => (
        <div key={i} className="flex items-center gap-2">
          <Input
            aria-label={t('containers.run.hostPort')}
            placeholder={t('containers.run.hostPortPlaceholder')}
            value={p.hostPort}
            inputMode="numeric"
            onChange={(e) => update(i, { hostPort: e.target.value })}
          />
          <span className="text-muted-foreground">→</span>
          <Input
            aria-label={t('containers.run.containerPort')}
            placeholder={t('containers.run.containerPort')}
            value={p.containerPort}
            inputMode="numeric"
            onChange={(e) => update(i, { containerPort: e.target.value })}
          />
          <Select value={p.protocol} onValueChange={(v) => update(i, { protocol: v })}>
            <SelectTrigger className="w-24" aria-label={t('containers.run.protocol')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="tcp">tcp</SelectItem>
              <SelectItem value="udp">udp</SelectItem>
            </SelectContent>
          </Select>
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            aria-label={t('containers.run.removePort')}
            onClick={() => onChange(ports.filter((_, j) => j !== i))}
          >
            <XIcon />
          </Button>
        </div>
      ))}
      <Button
        type="button"
        variant="outline"
        size="sm"
        className="justify-self-start"
        onClick={() => onChange([...ports, { hostPort: '', containerPort: '', protocol: 'tcp' }])}
      >
        <PlusIcon /> {t('containers.run.addPort')}
      </Button>
    </div>
  );
}
