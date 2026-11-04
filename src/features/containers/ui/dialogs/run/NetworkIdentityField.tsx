import { Input, Textarea } from '@/shared/ui';
import type { IdentityForm } from '../../../model/runAdvanced';
import { Field } from './Field';
import { t } from '@/shared/i18n';

/** Hostname, domain name, DNS and `/etc/hosts` entries. */
export function NetworkIdentityField({
  value,
  onChange,
}: {
  value: IdentityForm;
  onChange: (value: IdentityForm) => void;
}) {
  const set = (key: keyof IdentityForm) => (e: { target: { value: string } }) =>
    onChange({ ...value, [key]: e.target.value });
  const list = 'h-16 font-mono text-xs';
  return (
    <div className="grid gap-3 sm:grid-cols-2">
      <Field id="run-hostname" label={t('containers.advanced.hostname')}>
        <Input
          id="run-hostname"
          value={value.hostname}
          onChange={set('hostname')}
          placeholder={t('containers.advanced.hostnamePlaceholder')}
        />
      </Field>
      <Field id="run-domainname" label={t('containers.advanced.domainname')}>
        <Input
          id="run-domainname"
          value={value.domainname}
          onChange={set('domainname')}
          placeholder={t('containers.advanced.none')}
        />
      </Field>
      <Field
        id="run-dns"
        label={t('containers.advanced.dns')}
        hint={t('containers.advanced.dnsHint')}
      >
        <Textarea
          id="run-dns"
          value={value.dns}
          onChange={set('dns')}
          placeholder="192.168.1.1"
          className={list}
          spellCheck={false}
        />
      </Field>
      <Field
        id="run-dns-search"
        label={t('containers.advanced.dnsSearch')}
        hint={t('containers.advanced.dnsSearchHint')}
      >
        <Textarea
          id="run-dns-search"
          value={value.dnsSearch}
          onChange={set('dnsSearch')}
          placeholder="home.arpa"
          className={list}
          spellCheck={false}
        />
      </Field>
      <div className="sm:col-span-2">
        <Field
          id="run-extra-hosts"
          label={t('containers.advanced.extraHosts')}
          hint={t('containers.advanced.extraHostsHint')}
        >
          <Textarea
            id="run-extra-hosts"
            value={value.extraHosts}
            onChange={set('extraHosts')}
            // eslint-disable-next-line homeport/no-literal-ui-string -- sample host entries
            placeholder={'nas.home.arpa:192.168.1.10\nhost.docker.internal:host-gateway'}
            className={list}
            spellCheck={false}
          />
        </Field>
      </div>
    </div>
  );
}
