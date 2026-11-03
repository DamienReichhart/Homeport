import type { HostFormState } from '../../model/form';
import { KeychainNeeded } from './KeychainNeeded';
import { SecretText } from './SecretText';

/** The certificates of a TLS Host. */
export function TlsFields({
  form,
  set,
  keychain,
  keepHint,
}: {
  form: HostFormState;
  set: <K extends keyof HostFormState>(key: K, value: HostFormState[K]) => void;
  keychain: boolean;
  keepHint?: string;
}) {
  if (!keychain) return <KeychainNeeded />;
  return (
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
  );
}
