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
import type { AdvancedErrors, DeviceRow, SecurityForm } from '../../../model/runAdvanced';
import { describedBy } from './describedBy';
import { Field } from './Field';
import { t, type MessageKey } from '@/shared/i18n';

type Gpus = SecurityForm['gpus'];
const GPU_MODES: { value: Gpus['mode']; label: MessageKey }[] = [
  { value: 'none', label: 'containers.advanced.none' },
  { value: 'all', label: 'containers.advanced.gpus.all' },
  { value: 'count', label: 'containers.advanced.gpus.count' },
  { value: 'ids', label: 'containers.advanced.gpus.ids' },
];

/** Host devices (`--device`) and GPU requests (`--gpus`). */
export function DevicesField({
  value,
  errors,
  onChange,
}: {
  value: SecurityForm;
  errors: AdvancedErrors;
  onChange: (value: SecurityForm) => void;
}) {
  const devices = value.devices;
  const setDevices = (next: DeviceRow[]) => onChange({ ...value, devices: next });
  const update = (i: number, patch: Partial<DeviceRow>) =>
    setDevices(devices.map((d, j) => (j === i ? { ...d, ...patch } : d)));
  const gpus = value.gpus;
  const setGpus = (patch: Partial<Gpus>) => onChange({ ...value, gpus: { ...gpus, ...patch } });

  return (
    <div className="grid gap-3">
      <div className="grid gap-1.5">
        <Label>{t('containers.advanced.devices')}</Label>
        {devices.map((d, i) => (
          <div key={i} className="flex items-center gap-2">
            <Input
              aria-label={t('containers.advanced.hostDevice')}
              placeholder="/dev/dri"
              value={d.hostPath}
              className="font-mono"
              onChange={(e) => update(i, { hostPath: e.target.value })}
            />
            <span className="text-muted-foreground">→</span>
            <Input
              aria-label={t('containers.advanced.containerDevice')}
              placeholder={t('containers.advanced.samePath')}
              value={d.containerPath}
              className="font-mono"
              onChange={(e) => update(i, { containerPath: e.target.value })}
            />
            <Input
              aria-label={t('containers.advanced.permissions')}
              placeholder="rwm"
              value={d.permissions}
              className="w-20 font-mono"
              onChange={(e) => update(i, { permissions: e.target.value })}
            />
            <Button
              type="button"
              variant="ghost"
              size="icon-sm"
              aria-label={t('containers.advanced.removeDevice')}
              onClick={() => setDevices(devices.filter((_, j) => j !== i))}
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
          onClick={() =>
            setDevices([...devices, { hostPath: '', containerPath: '', permissions: 'rwm' }])
          }
        >
          <PlusIcon /> {t('containers.advanced.addDevice')}
        </Button>
      </div>
      <div className="grid gap-3 sm:grid-cols-2">
        <Field
          id="run-gpus"
          label={t('containers.advanced.gpus')}
          hint={t('containers.advanced.gpusHint')}
        >
          <Select
            value={gpus.mode}
            onValueChange={(mode) => setGpus({ mode: mode as Gpus['mode'] })}
          >
            <SelectTrigger id="run-gpus" aria-label={t('containers.advanced.gpus')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {GPU_MODES.map((m) => (
                <SelectItem key={m.value} value={m.value}>
                  {t(m.label)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </Field>
        {gpus.mode === 'count' && (
          <Field
            id="run-gpu-count"
            label={t('containers.advanced.gpuCount')}
            error={errors.gpuCount}
          >
            <Input
              id="run-gpu-count"
              inputMode="numeric"
              value={gpus.count}
              onChange={(e) => setGpus({ count: e.target.value })}
              {...describedBy('run-gpu-count', errors.gpuCount)}
            />
          </Field>
        )}
        {gpus.mode === 'ids' && (
          <Field
            id="run-gpu-ids"
            label={t('containers.advanced.gpuIds')}
            hint={t('containers.advanced.gpuIdsHint')}
          >
            <Input
              id="run-gpu-ids"
              placeholder="0, 1"
              value={gpus.ids}
              className="font-mono"
              onChange={(e) => setGpus({ ids: e.target.value })}
            />
          </Field>
        )}
      </div>
    </div>
  );
}
