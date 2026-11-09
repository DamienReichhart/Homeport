import { CheckCircle2Icon, PlugZapIcon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import type { NomadConnection } from '@/shared/api';
import {
  Button,
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
  ErrorMessage,
  Input,
  Label,
  Textarea,
} from '@/shared/ui';
import { useTestNomadCluster } from '../../api/mutations';
import {
  nomadForm,
  nomadValues,
  storedSecrets,
  type NomadSecretField,
  type NomadValues,
} from '../../model/nomadForm';
import { t } from '@/shared/i18n';

function Field({
  id,
  label,
  help,
  children,
}: {
  id: string;
  label: string;
  help?: string;
  children: ReactNode;
}) {
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      {children}
      {help && <p className="text-muted-foreground text-xs">{help}</p>}
    </div>
  );
}

/**
 * The address, region, ACL token and TLS material of a Nomad Cluster, with
 * a way to test them before saving. Editing an existing Cluster (`connection`
 * given) never shows its stored credentials: a field left empty keeps one.
 */
export function NomadClusterForm({
  clusterId = null,
  name,
  connection,
  submitLabel,
  pending,
  error,
  onSubmit,
}: {
  clusterId?: string | null;
  name?: string;
  connection?: NomadConnection;
  submitLabel: string;
  pending: boolean;
  error: unknown;
  onSubmit: (form: ReturnType<typeof nomadForm>) => void;
}) {
  const [values, setValues] = useState<NomadValues>(() => nomadValues(name, connection));
  const stored = storedSecrets(connection);
  const test = useTestNomadCluster(clusterId);
  const set = (field: keyof Omit<NomadValues, 'removed'>) => (value: string) => {
    test.reset();
    setValues((v) => ({ ...v, [field]: value }));
  };
  const toggleRemoved = (field: NomadSecretField) => {
    test.reset();
    setValues((v) => ({
      ...v,
      removed: v.removed.includes(field)
        ? v.removed.filter((f) => f !== field)
        : [...v.removed, field],
    }));
  };
  const secret = (field: NomadSecretField, label: string, multiline: boolean, help?: string) => {
    const id = `nomad-${field}`;
    const isStored = stored.includes(field);
    const removed = values.removed.includes(field);
    const props = {
      id,
      value: removed ? '' : values[field],
      disabled: removed,
      placeholder: removed
        ? t('clusters.nomad.willBeRemoved')
        : isStored
          ? t('clusters.nomad.keep')
          : undefined,
      spellCheck: false,
      autoComplete: 'off',
    };
    return (
      <Field id={id} label={label} help={help}>
        {multiline ? (
          <Textarea
            {...props}
            className="min-h-20 font-mono text-xs"
            onChange={(e) => set(field)(e.target.value)}
          />
        ) : (
          <Input
            {...props}
            type="password"
            className="font-mono"
            onChange={(e) => set(field)(e.target.value)}
          />
        )}
        {isStored && (
          <div>
            <Button
              type="button"
              variant="link"
              size="sm"
              className="h-auto p-0"
              onClick={() => toggleRemoved(field)}
            >
              {removed ? t('clusters.nomad.keepIt') : t('clusters.nomad.remove')}
            </Button>
          </div>
        )}
      </Field>
    );
  };
  const form = nomadForm(values);
  const ready = form.name !== '' && form.address !== '';
  return (
    <form
      className="grid gap-4"
      onSubmit={(e) => {
        e.preventDefault();
        if (ready) onSubmit(form);
      }}
    >
      <Field id="nomad-name" label={t('clusters.nomad.name')}>
        <Input
          id="nomad-name"
          value={values.name}
          onChange={(e) => set('name')(e.target.value)}
          className="max-w-sm"
        />
      </Field>
      <Field
        id="nomad-address"
        label={t('clusters.nomad.address')}
        help={t('clusters.nomad.addressHelp')}
      >
        <Input
          id="nomad-address"
          value={values.address}
          placeholder="http://127.0.0.1:4646"
          onChange={(e) => set('address')(e.target.value)}
          className="font-mono"
          spellCheck={false}
          autoComplete="off"
        />
      </Field>
      <Field
        id="nomad-region"
        label={t('clusters.nomad.region')}
        help={t('clusters.nomad.regionHelp')}
      >
        <Input
          id="nomad-region"
          value={values.region}
          placeholder="global"
          onChange={(e) => set('region')(e.target.value)}
          className="max-w-sm font-mono"
          spellCheck={false}
        />
      </Field>
      {secret('token', t('clusters.nomad.token'), false, t('clusters.nomad.tokenHelp'))}
      <Collapsible
        defaultOpen={
          stored.length > (stored.includes('token') ? 1 : 0) || !!connection?.tlsServerName
        }
      >
        <CollapsibleTrigger asChild>
          <Button type="button" variant="outline" size="sm">
            {t('clusters.nomad.tls')}
          </Button>
        </CollapsibleTrigger>
        <CollapsibleContent className="grid gap-4 pt-3">
          <p className="text-muted-foreground text-sm">{t('clusters.nomad.tlsHelp')}</p>
          {secret('certificateAuthority', t('clusters.nomad.ca'), true, t('clusters.nomad.caHelp'))}
          {secret('clientCertificate', t('clusters.nomad.clientCertificate'), true)}
          {secret('clientKey', t('clusters.nomad.clientKey'), true)}
          <Field
            id="nomad-tls-server-name"
            label={t('clusters.nomad.tlsServerName')}
            help={t('clusters.nomad.tlsServerNameHelp')}
          >
            <Input
              id="nomad-tls-server-name"
              value={values.tlsServerName}
              placeholder="server.global.nomad"
              onChange={(e) => set('tlsServerName')(e.target.value)}
              className="max-w-sm font-mono"
              spellCheck={false}
            />
          </Field>
        </CollapsibleContent>
      </Collapsible>
      {test.data && (
        <p className="flex items-center gap-2 text-sm" role="status">
          <CheckCircle2Icon className="text-success size-4" />
          {test.data.version
            ? t('clusters.nomad.testedVersion', {
                version: test.data.version,
                region: test.data.region ?? 'global',
              })
            : t('clusters.nomad.testedNoVersion', { region: test.data.region ?? 'global' })}
        </p>
      )}
      {test.error && <ErrorMessage error={test.error} />}
      {error ? <ErrorMessage error={error} /> : null}
      <div className="flex gap-2">
        <Button
          type="button"
          variant="outline"
          disabled={!ready || test.isPending}
          onClick={() => test.mutate(form)}
        >
          <PlugZapIcon /> {test.isPending ? t('clusters.nomad.testing') : t('clusters.nomad.test')}
        </Button>
        <Button type="submit" disabled={!ready || pending}>
          {submitLabel}
        </Button>
      </div>
    </form>
  );
}
