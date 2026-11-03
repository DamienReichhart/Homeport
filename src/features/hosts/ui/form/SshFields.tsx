import { KeyRoundIcon } from 'lucide-react';
import { Input, ToggleGroup, ToggleGroupItem } from '@/shared/ui';
import type { HostFormState } from '../../model/form';
import { Field } from './Field';
import { KeychainNeeded } from './KeychainNeeded';
import { SecretText } from './SecretText';

/** The server, user, socket and authentication of an SSH Host. */
export function SshFields({
  form,
  set,
  editing,
  keychain,
  keepHint,
}: {
  form: HostFormState;
  set: <K extends keyof HostFormState>(key: K, value: HostFormState[K]) => void;
  editing: boolean;
  keychain: boolean;
  keepHint?: string;
}) {
  return (
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
      <Field label="Remote socket path" hint="Rootless Docker uses /run/user/<uid>/docker.sock.">
        <Input value={form.sshSocketPath} onChange={(e) => set('sshSocketPath', e.target.value)} />
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
  );
}
