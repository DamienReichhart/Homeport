import { TriangleAlertIcon } from 'lucide-react';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Input,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { HOST_KINDS, type HostFormState, type HostKind } from '../../model/form';
import { Field } from './Field';
import { SshFields } from './SshFields';
import { TlsFields } from './TlsFields';

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
  const keepHint = editing ? 'Leave blank to keep the stored value.' : undefined;

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
              {label}
            </ToggleGroupItem>
          ))}
        </ToggleGroup>
      )}

      <Field label="Name">
        <Input
          value={form.name}
          onChange={(e) => set('name', e.target.value)}
          placeholder="production"
        />
      </Field>

      {form.kind === 'local' && (
        <Field
          label="Socket path"
          hint="A Unix socket, or a named pipe like //./pipe/docker_engine on Windows."
        >
          <Input value={form.socketPath} onChange={(e) => set('socketPath', e.target.value)} />
        </Field>
      )}

      {(form.kind === 'tcp' || form.kind === 'tls') && (
        <Field label="Address" hint="host:port, for example 192.168.1.10:2376.">
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
          <AlertTitle>Unencrypted and unauthenticated</AlertTitle>
          <AlertDescription>
            Anyone who can reach this port controls the Engine. Prefer SSH or TLS outside a trusted
            network.
          </AlertDescription>
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
