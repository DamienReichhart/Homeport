import { Alert, AlertDescription, AlertTitle, Label, Switch } from '@/shared/ui';
import { CAPABILITIES, normaliseCapability } from '../../../model/capabilities';
import type { SecurityForm } from '../../../model/runAdvanced';
import { Field } from './Field';
import { TokenInput } from './TokenInput';
import { t } from '@/shared/i18n';

/** Capabilities and privileged mode. */
export function SecurityField({
  value,
  onChange,
}: {
  value: SecurityForm;
  onChange: (value: SecurityForm) => void;
}) {
  return (
    <div className="grid gap-3">
      <div className="grid gap-3 sm:grid-cols-2">
        <Field
          id="run-cap-add"
          label={t('containers.advanced.capAdd')}
          hint={t('containers.advanced.capAddHint')}
        >
          <TokenInput
            id="run-cap-add"
            values={value.capAdd}
            suggestions={CAPABILITIES}
            placeholder="NET_ADMIN"
            normalise={normaliseCapability}
            onChange={(capAdd) => onChange({ ...value, capAdd })}
          />
        </Field>
        <Field
          id="run-cap-drop"
          label={t('containers.advanced.capDrop')}
          hint={t('containers.advanced.capDropHint')}
        >
          <TokenInput
            id="run-cap-drop"
            values={value.capDrop}
            suggestions={CAPABILITIES}
            placeholder="ALL"
            normalise={normaliseCapability}
            onChange={(capDrop) => onChange({ ...value, capDrop })}
          />
        </Field>
      </div>
      <div className="flex items-center gap-2">
        <Switch
          id="run-privileged"
          checked={value.privileged}
          onCheckedChange={(privileged) => onChange({ ...value, privileged })}
        />
        <Label htmlFor="run-privileged">{t('containers.advanced.privileged')}</Label>
      </div>
      {value.privileged && (
        <Alert variant="destructive">
          <AlertTitle>{t('containers.advanced.privilegedTitle')}</AlertTitle>
          <AlertDescription>{t('containers.advanced.privilegedWarning')}</AlertDescription>
        </Alert>
      )}
    </div>
  );
}
