import type { HostFormState } from '../../model/form';
import { KeychainNeeded } from './KeychainNeeded';
import { SecretText } from './SecretText';
import { t } from '@/shared/i18n';

/** PEM headers, shown as they appear in the files. */
const PEM_CERTIFICATE = '-----BEGIN CERTIFICATE-----';
const PEM_PRIVATE_KEY = '-----BEGIN PRIVATE KEY-----';

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
        label={t('hosts.tls.ca')}
        pickTitle={t('hosts.tls.ca.choose')}
        value={form.caPem}
        onChange={(v) => set('caPem', v)}
        placeholder={PEM_CERTIFICATE}
        hint={keepHint}
      />
      <SecretText
        label={t('hosts.tls.cert')}
        pickTitle={t('hosts.tls.cert.choose')}
        value={form.certPem}
        onChange={(v) => set('certPem', v)}
        placeholder={PEM_CERTIFICATE}
        hint={keepHint}
      />
      <SecretText
        label={t('hosts.tls.key')}
        pickTitle={t('hosts.tls.key.choose')}
        value={form.keyPem}
        onChange={(v) => set('keyPem', v)}
        placeholder={PEM_PRIVATE_KEY}
        hint={keepHint}
      />
    </>
  );
}
