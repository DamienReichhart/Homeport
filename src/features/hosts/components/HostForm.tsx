import { FileUpIcon, KeyRoundIcon, TriangleAlertIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { commands, toAppError } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { Alert, AlertDescription, AlertTitle } from '@/shared/ui/alert';
import { Button } from '@/shared/ui/button';
import { Input } from '@/shared/ui/input';
import { Label } from '@/shared/ui/label';
import { Textarea } from '@/shared/ui/textarea';
import { ToggleGroup, ToggleGroupItem } from '@/shared/ui/toggle-group';
import { toast } from 'sonner';
import type { HostFormState, HostKind } from '../form';

function Field({
  label,
  hint,
  children,
}: {
  label: string;
  hint?: ReactNode;
  children: ReactNode;
}) {
  return (
    <div className="grid gap-1.5">
      <Label>{label}</Label>
      {children}
      {hint && <p className="text-muted-foreground text-xs">{hint}</p>}
    </div>
  );
}

/** A PEM or key textarea that can also be filled from a file. */
function SecretText({
  label,
  value,
  onChange,
  placeholder,
  hint,
}: {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder: string;
  hint?: ReactNode;
}) {
  return (
    <Field label={label} hint={hint}>
      <div className="grid gap-1.5">
        <Textarea
          value={value}
          onChange={(e) => onChange(e.target.value)}
          placeholder={placeholder}
          className="h-24 font-mono text-xs"
          spellCheck={false}
        />
        <Button
          type="button"
          variant="outline"
          size="sm"
          className="justify-self-start"
          onClick={async () => {
            const path = await pickFile(`Choose the ${label.toLowerCase()}`);
            if (!path) return;
            try {
              onChange(await commands.readTextFile(path));
            } catch (e) {
              toast.error(toAppError(e).message);
            }
          }}
        >
          <FileUpIcon /> Load from file…
        </Button>
      </div>
    </Field>
  );
}

const KINDS: { kind: HostKind; label: string }[] = [
  { kind: 'ssh', label: 'SSH' },
  { kind: 'tls', label: 'TLS' },
  { kind: 'tcp', label: 'TCP' },
  { kind: 'local', label: 'Local socket' },
];

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
          {KINDS.map(({ kind, label }) => (
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

      {form.kind === 'tls' &&
        (keychain ? (
          <>
            <SecretText
              label="CA certificate"
              value={form.caPem}
              onChange={(v) => set('caPem', v)}
              placeholder="-----BEGIN CERTIFICATE-----"
              hint={keepHint}
            />
            <SecretText
              label="Client certificate"
              value={form.certPem}
              onChange={(v) => set('certPem', v)}
              placeholder="-----BEGIN CERTIFICATE-----"
              hint={keepHint}
            />
            <SecretText
              label="Client key"
              value={form.keyPem}
              onChange={(v) => set('keyPem', v)}
              placeholder="-----BEGIN PRIVATE KEY-----"
              hint={keepHint}
            />
          </>
        ) : (
          <KeychainNeeded />
        ))}

      {form.kind === 'ssh' && (
        <>
          <div className="grid grid-cols-[1fr_6rem] gap-3">
            <Field label="Server">
              <Input
                value={form.sshHost}
                onChange={(e) => set('sshHost', e.target.value)}
                placeholder="server.example.com"
              />
            </Field>
            <Field label="Port">
              <Input
                value={form.sshPort}
                onChange={(e) => set('sshPort', e.target.value)}
                inputMode="numeric"
              />
            </Field>
          </div>
          <Field
            label="User"
            hint="Needs access to the Docker socket, usually by being in the docker group."
          >
            <Input
              value={form.sshUser}
              onChange={(e) => set('sshUser', e.target.value)}
              placeholder="deploy"
            />
          </Field>
          <Field
            label="Remote socket path"
            hint="Rootless Docker uses /run/user/<uid>/docker.sock."
          >
            <Input
              value={form.sshSocketPath}
              onChange={(e) => set('sshSocketPath', e.target.value)}
            />
          </Field>
          <Field label="Authentication">
            <ToggleGroup
              type="single"
              variant="outline"
              value={form.sshAuth}
              onValueChange={(auth) => auth && set('sshAuth', auth as HostFormState['sshAuth'])}
            >
              <ToggleGroupItem value="generated" disabled={!keychain}>
                <KeyRoundIcon /> New key for this host
              </ToggleGroupItem>
              <ToggleGroupItem value="imported" disabled={!keychain}>
                Existing key
              </ToggleGroupItem>
              <ToggleGroupItem value="agent">ssh-agent</ToggleGroupItem>
            </ToggleGroup>
          </Field>
          {!keychain && form.sshAuth !== 'agent' && <KeychainNeeded />}
          {form.sshAuth === 'generated' && keychain && (
            <p className="text-muted-foreground text-sm">
              {editing
                ? 'This host keeps its key. Regenerate it from the host page.'
                : 'Homeport creates an Ed25519 key for this host and shows you the command that authorizes it on the server.'}
            </p>
          )}
          {form.sshAuth === 'imported' && keychain && (
            <>
              <SecretText
                label="Private key"
                value={form.privateKey}
                onChange={(v) => set('privateKey', v)}
                placeholder="-----BEGIN OPENSSH PRIVATE KEY-----"
                hint={keepHint ?? 'Stored encrypted; it never leaves this machine.'}
              />
              <Field label="Passphrase">
                <Input
                  type="password"
                  value={form.passphrase}
                  onChange={(e) => set('passphrase', e.target.value)}
                />
              </Field>
            </>
          )}
        </>
      )}
    </div>
  );
}

function KeychainNeeded() {
  return (
    <Alert variant="destructive">
      <TriangleAlertIcon />
      <AlertTitle>No OS keychain</AlertTitle>
      <AlertDescription>
        Keys and certificates are encrypted with a key kept in the OS keychain, and none is
        available. Use ssh-agent, a local socket or TCP instead.
      </AlertDescription>
    </Alert>
  );
}
