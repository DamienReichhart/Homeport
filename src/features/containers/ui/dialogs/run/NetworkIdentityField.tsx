import { Input, Textarea } from '@/shared/ui';
import type { IdentityForm } from '../../../model/runAdvanced';
import { Field } from './Field';

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
      <Field id="run-hostname" label="Hostname">
        <Input
          id="run-hostname"
          value={value.hostname}
          onChange={set('hostname')}
          placeholder="The container ID"
        />
      </Field>
      <Field id="run-domainname" label="Domain name">
        <Input
          id="run-domainname"
          value={value.domainname}
          onChange={set('domainname')}
          placeholder="None"
        />
      </Field>
      <Field id="run-dns" label="DNS servers" hint="One address per line.">
        <Textarea
          id="run-dns"
          value={value.dns}
          onChange={set('dns')}
          placeholder="192.168.1.1"
          className={list}
          spellCheck={false}
        />
      </Field>
      <Field id="run-dns-search" label="DNS search domains" hint="One per line.">
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
          label="Extra hosts"
          hint="One name:address per line; host-gateway stands for the host's address."
        >
          <Textarea
            id="run-extra-hosts"
            value={value.extraHosts}
            onChange={set('extraHosts')}
            placeholder={'nas.home.arpa:192.168.1.10\nhost.docker.internal:host-gateway'}
            className={list}
            spellCheck={false}
          />
        </Field>
      </div>
    </div>
  );
}
