import { Input, Label, Switch } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Whether helm waits for the release's resources to be ready, and for how long. */
export function WaitFields({
  wait,
  minutes,
  onWait,
  onMinutes,
}: {
  wait: boolean;
  minutes: string;
  onWait: (wait: boolean) => void;
  onMinutes: (minutes: string) => void;
}) {
  return (
    <div className="flex flex-wrap items-center gap-3">
      <div className="flex items-center gap-2">
        <Switch id="helm-wait" checked={wait} onCheckedChange={onWait} />
        <Label htmlFor="helm-wait">{t('helm.install.wait')}</Label>
      </div>
      {wait && (
        <div className="flex items-center gap-2">
          <Label htmlFor="helm-timeout" className="font-normal">
            {t('helm.install.timeout')}
          </Label>
          <Input
            id="helm-timeout"
            className="w-20"
            inputMode="decimal"
            value={minutes}
            onChange={(e) => onMinutes(e.target.value)}
          />
          <span className="text-muted-foreground text-sm">{t('helm.install.minutes')}</span>
        </div>
      )}
    </div>
  );
}
