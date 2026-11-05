import { KeyRoundIcon } from 'lucide-react';
import { Input, ToggleGroup, ToggleGroupItem } from '@/shared/ui';
import type { HostFormState } from '../../model/form';
import { BastionFields } from './BastionFields';
import { Field } from './Field';
import { KeychainNeeded } from './KeychainNeeded';
import { SecretText } from './SecretText';
import { t } from '@/shared/i18n';

/** The PEM header of an OpenSSH key, shown as it appears in the file. */
const PEM_OPENSSH_KEY = '-----BEGIN OPENSSH PRIVATE KEY-----';

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
        <Field label={t('hosts.ssh.server')}>
          <Input
            value={form.sshHost}
            onChange={(e) => set('sshHost', e.target.value)}
            placeholder="server.example.com"
          />
        </Field>
        <Field label={t('hosts.ssh.port')}>
          <Input
            value={form.sshPort}
            onChange={(e) => set('sshPort', e.target.value)}
            inputMode="numeric"
          />
        </Field>
      </div>
      <Field label={t('hosts.ssh.user')} hint={t('hosts.ssh.userHint')}>
        <Input
          value={form.sshUser}
          onChange={(e) => set('sshUser', e.target.value)}
          placeholder="deploy"
        />
      </Field>
      <Field
        label={t('hosts.ssh.socketPath')}
        hint={t('hosts.ssh.socketPathHint', { path: '/run/user/<uid>/docker.sock' })}
      >
        <Input value={form.sshSocketPath} onChange={(e) => set('sshSocketPath', e.target.value)} />
      </Field>
      <Field label={t('hosts.ssh.auth')}>
        <ToggleGroup
          type="single"
          variant="outline"
          value={form.sshAuth}
          onValueChange={(auth) => auth && set('sshAuth', auth as HostFormState['sshAuth'])}
        >
          <ToggleGroupItem value="generated" disabled={!keychain}>
            <KeyRoundIcon /> {t('hosts.ssh.auth.generated')}
          </ToggleGroupItem>
          <ToggleGroupItem value="imported" disabled={!keychain}>
            {t('hosts.ssh.auth.imported')}
          </ToggleGroupItem>
          <ToggleGroupItem value="agent">ssh-agent</ToggleGroupItem>
        </ToggleGroup>
      </Field>
      {!keychain && form.sshAuth !== 'agent' && <KeychainNeeded />}
      {form.sshAuth === 'generated' && keychain && (
        <p className="text-muted-foreground text-sm">
          {editing ? t('hosts.ssh.generated.editing') : t('hosts.ssh.generated.new')}
        </p>
      )}
      {form.sshAuth === 'imported' && keychain && (
        <>
          <SecretText
            label={t('hosts.ssh.privateKey')}
            pickTitle={t('hosts.ssh.privateKey.choose')}
            value={form.privateKey}
            onChange={(v) => set('privateKey', v)}
            placeholder={PEM_OPENSSH_KEY}
            hint={keepHint ?? t('hosts.ssh.privateKeyHint')}
          />
          <Field label={t('hosts.ssh.passphrase')}>
            <Input
              type="password"
              value={form.passphrase}
              onChange={(e) => set('passphrase', e.target.value)}
            />
          </Field>
        </>
      )}
      <BastionFields
        bastions={form.bastions}
        onChange={(bastions) => set('bastions', bastions)}
        keychain={keychain}
      />
    </>
  );
}
