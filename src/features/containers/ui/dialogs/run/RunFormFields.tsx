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
import { t, type MessageKey } from '@/shared/i18n';

const RESTART: { value: RestartPolicy; label: MessageKey }[] = [
  { value: 'no', label: 'containers.run.restart.no' },
  { value: 'on-failure', label: 'containers.run.restart.onFailure' },
  { value: 'unless-stopped', label: 'containers.run.restart.unlessStopped' },
  { value: 'always', label: 'containers.run.restart.always' },
];
const DEFAULT_NETWORK = '__default__';

/** Every field of the Run dialog, shared by Run, Duplicate and Edit. */
export function RunFormFields({
  hostId,
  form,
  setForm,
  errors,
  advancedOpen = false,
  lockName = false,
}: {
  hostId: string;
  form: RunForm;
  setForm: Dispatch<SetStateAction<RunForm>>;
  errors: RunFormErrors;
  advancedOpen?: boolean;
  /** Edit keeps the name; Rename changes it. */
  lockName?: boolean;
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
          <Label htmlFor="run-image">{t('containers.run.image')}</Label>
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
              .map((tag) => (
                <option key={tag} value={tag} />
              ))}
          </datalist>
          {images.data && !images.data.some((i) => i.tags.includes(form.image.trim())) && (
            <RegistryLogin reference={form.image} verb="pull" />
          )}
        </div>
        <div className="grid gap-1.5">
          <Label htmlFor="run-name">{t('containers.run.name')}</Label>
          <Input
            id="run-name"
            value={form.name}
            onChange={(e) => set('name', e.target.value)}
            placeholder={t('containers.run.namePlaceholder')}
            disabled={lockName}
          />
          {lockName && (
            <p className="text-muted-foreground text-xs">{t('containers.run.nameKept')}</p>
          )}
        </div>
      </div>
      <div className="grid gap-1.5">
        <Label htmlFor="run-cmd">{t('containers.run.command')}</Label>
        <Input
          id="run-cmd"
          value={form.command}
          onChange={(e) => set('command', e.target.value)}
          placeholder={t('containers.run.commandPlaceholder')}
          className="font-mono"
        />
      </div>

      <PortsField ports={form.ports} onChange={(ports) => set('ports', ports)} />
      <MountsField mounts={form.mounts} onChange={(mounts) => set('mounts', mounts)} />

      <div className="grid gap-1.5">
        <Label htmlFor="run-env">{t('containers.run.environment')}</Label>
        <Textarea
          id="run-env"
          value={form.env}
          onChange={(e) => set('env', e.target.value)}
          // eslint-disable-next-line homeport/no-literal-ui-string -- sample variables
          placeholder={'KEY=value\nOTHER=value'}
          className="h-20 font-mono text-xs"
          spellCheck={false}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="grid gap-1.5">
          <Label>{t('containers.run.network')}</Label>
          <Select
            value={form.network || DEFAULT_NETWORK}
            onValueChange={(v) => set('network', v === DEFAULT_NETWORK ? '' : v)}
          >
            <SelectTrigger aria-label={t('containers.run.network')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={DEFAULT_NETWORK}>{t('containers.run.defaultNetwork')}</SelectItem>
              {networkNames.map((n) => (
                <SelectItem key={n} value={n}>
                  {n}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <div className="grid gap-1.5">
          <Label>{t('containers.run.restart')}</Label>
          <Select
            value={form.restart}
            onValueChange={(v) => set('restart', v as RestartPolicy)}
            disabled={form.autoRemove}
          >
            <SelectTrigger aria-label={t('containers.run.restartPolicy')}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {RESTART.map((r) => (
                <SelectItem key={r.value} value={r.value}>
                  {t(r.label)}
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
        <Label htmlFor="run-rm">{t('containers.run.autoRemove')}</Label>
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
