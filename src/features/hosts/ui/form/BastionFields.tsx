import { ArrowDownIcon, ArrowUpIcon, PlusIcon, Trash2Icon } from 'lucide-react';
import { Button, Input, Label, ToggleGroup, ToggleGroupItem } from '@/shared/ui';
import { BASTION_AUTHS, emptyBastion, moveBastion, type BastionFormState } from '../../model/form';
import { Field } from './Field';
import { SecretText } from './SecretText';
import { t } from '@/shared/i18n';

/** The PEM header of an OpenSSH key, shown as it appears in the file. */
const PEM_OPENSSH_KEY = '-----BEGIN OPENSSH PRIVATE KEY-----';

/** The ordered bastions an SSH Host is reached through: added, edited, reordered, removed. */
export function BastionFields({
  bastions,
  onChange,
  keychain,
}: {
  bastions: BastionFormState[];
  onChange: (bastions: BastionFormState[]) => void;
  keychain: boolean;
}) {
  const update = (index: number, patch: Partial<BastionFormState>) =>
    onChange(bastions.map((b, i) => (i === index ? { ...b, ...patch } : b)));

  return (
    <div className="grid gap-3">
      <div className="grid gap-1">
        <Label>{t('hosts.bastions.title')}</Label>
        <p className="text-muted-foreground text-xs">{t('hosts.bastions.description')}</p>
      </div>
      {bastions.map((bastion, index) => {
        const n = index + 1;
        return (
          <fieldset
            key={index}
            className="grid gap-3 rounded-md border p-3"
            aria-label={t('hosts.bastions.one', { n })}
          >
            <div className="flex items-center justify-between gap-2">
              <span className="text-sm font-medium">{t('hosts.bastions.one', { n })}</span>
              <div className="flex">
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  disabled={index === 0}
                  aria-label={t('hosts.bastions.moveUp', { n })}
                  onClick={() => onChange(moveBastion(bastions, index, -1))}
                >
                  <ArrowUpIcon />
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  disabled={index === bastions.length - 1}
                  aria-label={t('hosts.bastions.moveDown', { n })}
                  onClick={() => onChange(moveBastion(bastions, index, 1))}
                >
                  <ArrowDownIcon />
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('hosts.bastions.remove', { n })}
                  onClick={() => onChange(bastions.filter((_, i) => i !== index))}
                >
                  <Trash2Icon />
                </Button>
              </div>
            </div>
            <div className="grid grid-cols-[1fr_6rem] gap-3">
              <Field label={t('hosts.ssh.server')}>
                <Input
                  value={bastion.host}
                  onChange={(e) => update(index, { host: e.target.value })}
                  placeholder="bastion.example.com"
                  aria-label={t('hosts.bastions.server', { n })}
                />
              </Field>
              <Field label={t('hosts.ssh.port')}>
                <Input
                  value={bastion.port}
                  onChange={(e) => update(index, { port: e.target.value })}
                  inputMode="numeric"
                  aria-label={t('hosts.bastions.port', { n })}
                />
              </Field>
            </div>
            <Field label={t('hosts.ssh.user')}>
              <Input
                value={bastion.user}
                onChange={(e) => update(index, { user: e.target.value })}
                aria-label={t('hosts.bastions.user', { n })}
              />
            </Field>
            <Field label={t('hosts.ssh.auth')}>
              <ToggleGroup
                type="single"
                variant="outline"
                value={bastion.auth}
                onValueChange={(auth) =>
                  auth && update(index, { auth: auth as BastionFormState['auth'] })
                }
              >
                {BASTION_AUTHS.map(({ auth, label }) => (
                  <ToggleGroupItem
                    key={auth}
                    value={auth}
                    disabled={auth === 'imported' && !keychain}
                  >
                    {t(label)}
                  </ToggleGroupItem>
                ))}
              </ToggleGroup>
            </Field>
            {bastion.auth === 'imported' && keychain && (
              <>
                <SecretText
                  label={t('hosts.ssh.privateKey')}
                  pickTitle={t('hosts.ssh.privateKey.choose')}
                  value={bastion.privateKey}
                  onChange={(privateKey) => update(index, { privateKey })}
                  placeholder={PEM_OPENSSH_KEY}
                  hint={
                    bastion.storedPublicKey
                      ? t('hosts.form.keepHint')
                      : t('hosts.ssh.privateKeyHint')
                  }
                />
                <Field label={t('hosts.ssh.passphrase')}>
                  <Input
                    type="password"
                    value={bastion.passphrase}
                    onChange={(e) => update(index, { passphrase: e.target.value })}
                  />
                </Field>
              </>
            )}
          </fieldset>
        );
      })}
      <Button
        type="button"
        variant="outline"
        size="sm"
        className="justify-self-start"
        onClick={() => onChange([...bastions, emptyBastion()])}
      >
        <PlusIcon /> {t('hosts.bastions.add')}
      </Button>
    </div>
  );
}
