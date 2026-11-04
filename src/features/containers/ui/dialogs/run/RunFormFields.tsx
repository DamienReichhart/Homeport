import { useQuery } from '@tanstack/react-query';
import type { Dispatch, SetStateAction } from 'react';
import { resourceQueries } from '@/core/host';
import { RegistryLogin } from '@/features/registries';
import type { RestartPolicy } from '@/shared/api';
import {
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Switch,
  Textarea,
} from '@/shared/ui';
import type { RunForm, RunFormErrors } from '../../../model/runForm';
import { AdvancedOptions } from './AdvancedOptions';
import { MountsField } from './MountsField';
import { PortsField } from './PortsField';

const RESTART: { value: RestartPolicy; label: string }[] = [
  { value: 'no', label: 'Never' },
  { value: 'on-failure', label: 'On failure' },
  { value: 'unless-stopped', label: 'Unless stopped' },
  { value: 'always', label: 'Always' },
];
const DEFAULT_NETWORK = '__default__';

/** Every field of the Run dialog, shared by Run, Duplicate and Edit. */
export function RunFormFields({
  hostId,
  form,
  setForm,
  errors,
  advancedOpen = false,
}: {
  hostId: string;
  form: RunForm;
  setForm: Dispatch<SetStateAction<RunForm>>;
  errors: RunFormErrors;
  advancedOpen?: boolean;
}) {
  const images = useQuery(resourceQueries.images(hostId));
  const networks = useQuery(resourceQueries.networks(hostId));
  const set = <K extends keyof RunForm>(key: K, value: RunForm[K]) =>
    setForm((f) => ({ ...f, [key]: value }));
  const networkNames = (networks.data ?? []).map((n) => n.name).filter((n) => n !== 'bridge');
  // `host`, `none` or a network that's gone still shows as chosen.
  if (form.network && !networkNames.includes(form.network)) networkNames.push(form.network);

  return (
    <>
      <div className="grid grid-cols-2 gap-3">
        <div className="grid gap-1.5">
          <Label htmlFor="run-image">Image</Label>
          <Input
            id="run-image"
            list="run-images"
            value={form.image}
            onChange={(e) => set('image', e.target.value)}
            placeholder="nginx:latest"
            autoFocus
          />
          <datalist id="run-images">
            {images.data
              ?.flatMap((i) => i.tags)
              .map((t) => (
                <option key={t} value={t} />
              ))}
          </datalist>
          {images.data && !images.data.some((i) => i.tags.includes(form.image.trim())) && (
            <RegistryLogin reference={form.image} verb="Pulled" />
          )}
        </div>
        <div className="grid gap-1.5">
          <Label htmlFor="run-name">Name</Label>
          <Input
            id="run-name"
            value={form.name}
            onChange={(e) => set('name', e.target.value)}
            placeholder="Generated if empty"
          />
        </div>
      </div>
      <div className="grid gap-1.5">
        <Label htmlFor="run-cmd">Command</Label>
        <Input
          id="run-cmd"
          value={form.command}
          onChange={(e) => set('command', e.target.value)}
          placeholder="The image's default"
          className="font-mono"
        />
      </div>

      <PortsField ports={form.ports} onChange={(ports) => set('ports', ports)} />
      <MountsField mounts={form.mounts} onChange={(mounts) => set('mounts', mounts)} />

      <div className="grid gap-1.5">
        <Label htmlFor="run-env">Environment</Label>
        <Textarea
          id="run-env"
          value={form.env}
          onChange={(e) => set('env', e.target.value)}
          placeholder={'KEY=value\nOTHER=value'}
          className="h-20 font-mono text-xs"
          spellCheck={false}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="grid gap-1.5">
          <Label>Network</Label>
          <Select
            value={form.network || DEFAULT_NETWORK}
            onValueChange={(v) => set('network', v === DEFAULT_NETWORK ? '' : v)}
          >
            <SelectTrigger aria-label="Network">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={DEFAULT_NETWORK}>Default (bridge)</SelectItem>
              {networkNames.map((n) => (
                <SelectItem key={n} value={n}>
                  {n}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <div className="grid gap-1.5">
          <Label>Restart</Label>
          <Select
            value={form.restart}
            onValueChange={(v) => set('restart', v as RestartPolicy)}
            disabled={form.autoRemove}
          >
            <SelectTrigger aria-label="Restart policy">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {RESTART.map((r) => (
                <SelectItem key={r.value} value={r.value}>
                  {r.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
      <div className="flex items-center gap-2">
        <Switch
          id="run-rm"
          checked={form.autoRemove}
          onCheckedChange={(v) =>
            setForm((f) => ({ ...f, autoRemove: v, restart: v ? 'no' : f.restart }))
          }
        />
        <Label htmlFor="run-rm">Remove the container when it exits</Label>
      </div>

      <AdvancedOptions
        value={form.advanced}
        errors={errors}
        defaultOpen={advancedOpen}
        onChange={(advanced) => set('advanced', advanced)}
      />
    </>
  );
}
