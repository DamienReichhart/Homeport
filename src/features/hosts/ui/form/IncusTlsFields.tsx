import { useQuery } from '@tanstack/react-query';
import { ShieldCheckIcon } from 'lucide-react';
import { CopyButton, ErrorMessage, Textarea, ToggleGroup, ToggleGroupItem } from '@/shared/ui';
import { trustTokenQuery } from '../../api/queries';
import type { HostFormState, IncusTrust } from '../../model/form';
import { Field } from './Field';
import { KeychainNeeded } from './KeychainNeeded';
import { SecretText } from './SecretText';
import { t } from '@/shared/i18n';

const PEM_CERTIFICATE = '-----BEGIN CERTIFICATE-----';
const PEM_PRIVATE_KEY = '-----BEGIN PRIVATE KEY-----';
const TOKEN_COMMAND = 'incus config trust add homeport';

/** What a pasted token says, read by the backend as the user types. */
function TokenSummary({ token }: { token: string }) {
  const summary = useQuery(trustTokenQuery(token));
  if (!token.trim() || summary.isPending) return null;
  if (summary.error) return <ErrorMessage error={summary.error} />;
  const s = summary.data;
  return (
    <div className="bg-muted/50 grid gap-0.5 rounded-md p-3 text-sm">
      <p className="flex items-center gap-2 font-medium">
        <ShieldCheckIcon className="text-success size-4" />
        {t('hosts.incus.tokenReads', { name: s.clientName, addresses: s.addresses.join(', ') })}
      </p>
      <p className="text-muted-foreground font-mono text-xs break-all">
        {t('hosts.incus.tokenFingerprint', { fingerprint: s.fingerprint })}
      </p>
      {s.expiresAt && (
        <p className="text-muted-foreground text-xs">
          {t('hosts.incus.tokenExpires', { time: s.expiresAt })}
        </p>
      )}
    </div>
  );
}

/**
 * How an Incus Host over HTTPS is trusted: a trust token for a new Host,
 * or a client certificate with the server's certificate to pin.
 */
export function IncusTlsFields({
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
  if (!keychain) return <KeychainNeeded />;
  return (
    <>
      {!editing && (
        <Field label={t('hosts.incus.trust')}>
          <ToggleGroup
            type="single"
            variant="outline"
            value={form.incusTrust}
            onValueChange={(v) => v && set('incusTrust', v as IncusTrust)}
            className="justify-self-start"
          >
            <ToggleGroupItem value="token">{t('hosts.incus.trust.token')}</ToggleGroupItem>
            <ToggleGroupItem value="certificate">
              {t('hosts.incus.trust.certificate')}
            </ToggleGroupItem>
          </ToggleGroup>
        </Field>
      )}
      {!editing && form.incusTrust === 'token' ? (
        <>
          <Field label={t('hosts.incus.token')} hint={t('hosts.incus.tokenHint')}>
            <Textarea
              aria-label={t('hosts.incus.token')}
              value={form.trustToken}
              onChange={(e) => set('trustToken', e.target.value)}
              className="h-20 font-mono text-xs"
              spellCheck={false}
            />
          </Field>
          <div className="flex items-center gap-2">
            <code className="bg-muted rounded px-2 py-1 font-mono text-xs">{TOKEN_COMMAND}</code>
            <CopyButton text={TOKEN_COMMAND} />
          </div>
          <TokenSummary token={form.trustToken} />
        </>
      ) : (
        <>
          <SecretText
            label={t('hosts.incus.serverCert')}
            pickTitle={t('hosts.incus.serverCert.choose')}
            value={form.caPem}
            onChange={(v) => set('caPem', v)}
            placeholder={PEM_CERTIFICATE}
            hint={keepHint ?? t('hosts.incus.serverCertHint')}
          />
          <SecretText
            label={t('hosts.incus.clientCert')}
            pickTitle={t('hosts.tls.cert.choose')}
            value={form.certPem}
            onChange={(v) => set('certPem', v)}
            placeholder={PEM_CERTIFICATE}
            hint={keepHint ?? t('hosts.incus.clientCertHint')}
          />
          <SecretText
            label={t('hosts.incus.clientKey')}
            pickTitle={t('hosts.tls.key.choose')}
            value={form.keyPem}
            onChange={(v) => set('keyPem', v)}
            placeholder={PEM_PRIVATE_KEY}
            hint={keepHint}
          />
        </>
      )}
    </>
  );
}
