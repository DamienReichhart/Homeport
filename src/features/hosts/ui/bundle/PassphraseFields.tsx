import { Input, Label } from '@/shared/ui';
import { MIN_PASSPHRASE } from '../../model/bundle';
import { t } from '@/shared/i18n';

/** A passphrase typed twice, for protecting an export. */
export function PassphraseFields({
  passphrase,
  confirmation,
  onChange,
}: {
  passphrase: string;
  confirmation: string;
  onChange: (passphrase: string, confirmation: string) => void;
}) {
  return (
    <div className="grid gap-3 sm:grid-cols-2">
      <div className="grid gap-1.5">
        <Label htmlFor="bundle-passphrase">{t('hosts.bundle.passphrase')}</Label>
        <Input
          id="bundle-passphrase"
          type="password"
          autoComplete="new-password"
          value={passphrase}
          onChange={(e) => onChange(e.target.value, confirmation)}
        />
        <p className="text-muted-foreground text-xs">
          {t('hosts.bundle.passphraseHint', { min: MIN_PASSPHRASE })}
        </p>
      </div>
      <div className="grid gap-1.5">
        <Label htmlFor="bundle-confirmation">{t('hosts.bundle.confirmation')}</Label>
        <Input
          id="bundle-confirmation"
          type="password"
          autoComplete="new-password"
          value={confirmation}
          onChange={(e) => onChange(passphrase, e.target.value)}
        />
      </div>
    </div>
  );
}
