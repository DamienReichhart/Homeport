import { TriangleAlertIcon } from 'lucide-react';
import { ENGINE_LABELS } from '@/core/host';
import type { EngineKind } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import {
  ENGINE_KINDS,
  engineForSocket,
  HOST_KINDS,
  type HostFormState,
  type HostKind,
} from '../../model/form';
import { Field } from './Field';
import { SshFields } from './SshFields';
import { TlsFields } from './TlsFields';
import { t } from '@/shared/i18n';

/**
 * Fields for defining a Host. `editing` relaxes credentials (blank keeps the
 * stored ones); `keychain: false` disables what would need stored secrets.
 */
export function HostForm({
  form,
  onChange,
  editing,
  keychain,
}: {
  form: HostFormState;
  onChange: (form: HostFormState) => void;
  editing: boolean;
  keychain: boolean;
}) {
  const set = <K extends keyof HostFormState>(key: K, value: HostFormState[K]) =>
    onChange({ ...form, [key]: value });
  const keepHint = editing ? t('hosts.form.keepHint') : undefined;

  return (
    <div className="grid gap-4">
      {!editing && (
        <ToggleGroup
          type="single"
          variant="outline"
          value={form.kind}
          onValueChange={(kind) => kind && set('kind', kind as HostKind)}
        >
          {HOST_KINDS.map(({ kind, label }) => (
            <ToggleGroupItem key={kind} value={kind}>
              {t(label)}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
      )}

      <Field label={t('hosts.form.name')}>
        <Input
          value={form.name}
          onChange={(e) => set('name', e.target.value)}
          placeholder="production"
        />
      </Field>

      <Field label={t('hosts.form.engine')} hint={t('hosts.form.engineHint')}>
        <Select value={form.engine} onValueChange={(engine) => set('engine', engine as EngineKind)}>
          <SelectTrigger className="w-full" aria-label={t('hosts.form.engine')}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {ENGINE_KINDS.map(({ engine, available }) => (
              <SelectItem key={engine} value={engine} disabled={!available}>
                {available
                  ? t(ENGINE_LABELS[engine])
                  : t('hosts.form.engineSoon', { engine: t(ENGINE_LABELS[engine]) })}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Field>

      {form.kind === 'local' && (
        <Field
          label={t('hosts.form.socketPath')}
          hint={t('hosts.form.socketPathHint', { pipe: '//./pipe/docker_engine' })}
        >
          <Input
            value={form.socketPath}
            onChange={(e) =>
              onChange({
                ...form,
                socketPath: e.target.value,
                engine: engineForSocket(e.target.value),
              })
            }
          />
        </Field>
      )}

      {(form.kind === 'tcp' || form.kind === 'tls') && (
        <Field
          label={t('hosts.form.address')}
          hint={t('hosts.form.addressHint', { example: '192.168.1.10:2376' })}
        >
          <Input
            value={form.address}
            onChange={(e) => set('address', e.target.value)}
            placeholder="docker.example.com:2376"
          />
        </Field>
      )}

      {form.kind === 'tcp' && (
        <Alert>
          <TriangleAlertIcon />
          <AlertTitle>{t('hosts.form.tcpWarning.title')}</AlertTitle>
          <AlertDescription>{t('hosts.form.tcpWarning.description')}</AlertDescription>
        </Alert>
      )}

      {form.kind === 'tls' && (
        <TlsFields form={form} set={set} keychain={keychain} keepHint={keepHint} />
      )}

      {form.kind === 'ssh' && (
        <SshFields
          form={form}
          set={set}
          editing={editing}
          keychain={keychain}
          keepHint={keepHint}
        />
      )}
    </div>
  );
}
