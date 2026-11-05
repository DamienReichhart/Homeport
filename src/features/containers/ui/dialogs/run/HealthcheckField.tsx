import { Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import type { AdvancedErrors, HealthcheckForm } from '../../../model/runAdvanced';
import { describedBy } from './describedBy';
import { Field } from './Field';
import { t, type MessageKey } from '@/shared/i18n';

const MODES: { value: HealthcheckForm['mode']; label: MessageKey }[] = [
  { value: 'image', label: 'containers.advanced.health.image' },
  { value: 'custom', label: 'containers.advanced.health.custom' },
  { value: 'disabled', label: 'containers.advanced.health.disabled' },
];

/** A healthcheck run in the container's shell (`--health-cmd` and friends). */
export function HealthcheckField({
  value,
  errors,
  onChange,
}: {
  value: HealthcheckForm;
  errors: AdvancedErrors;
  onChange: (value: HealthcheckForm) => void;
}) {
  const duration = (
    key: 'interval' | 'timeout' | 'startPeriod',
    id: string,
    label: string,
    placeholder: string,
  ) => (
    <Field id={id} label={label} error={errors[key]}>
      <Input
        id={id}
        value={value[key]}
        placeholder={placeholder}
        onChange={(e) => onChange({ ...value, [key]: e.target.value })}
        {...describedBy(id, errors[key])}
      />
    </Field>
  );
  return (
    <div className="grid gap-3">
      <Field id="run-health-mode" label={t('containers.advanced.healthcheck')}>
        <Select
          value={value.mode}
          onValueChange={(mode) => onChange({ ...value, mode: mode as HealthcheckForm['mode'] })}
        >
          <SelectTrigger id="run-health-mode" aria-label={t('containers.advanced.healthcheck')}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {MODES.map((m) => (
              <SelectItem key={m.value} value={m.value}>
                {t(m.label)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Field>
      {value.mode === 'custom' && (
        <>
          <Field
            id="run-health-cmd"
            label={t('containers.advanced.healthCommand')}
            hint={t('containers.advanced.healthCommandHint')}
          >
            <Input
              id="run-health-cmd"
              value={value.command}
              // eslint-disable-next-line homeport/no-literal-ui-string -- a shell command
              placeholder="wget -qO- http://localhost/ || exit 1"
              className="font-mono"
              onChange={(e) => onChange({ ...value, command: e.target.value })}
            />
          </Field>
          <div className="grid gap-3 sm:grid-cols-4">
            {duration('interval', 'run-health-interval', t('containers.advanced.interval'), '30s')}
            {duration('timeout', 'run-health-timeout', t('containers.advanced.timeout'), '30s')}
            <Field
              id="run-health-retries"
              label={t('containers.advanced.retries')}
              error={errors.retries}
            >
              <Input
                id="run-health-retries"
                inputMode="numeric"
                value={value.retries}
                placeholder="3"
                onChange={(e) => onChange({ ...value, retries: e.target.value })}
                {...describedBy('run-health-retries', errors.retries)}
              />
            </Field>
            {duration(
              'startPeriod',
              'run-health-start',
              t('containers.advanced.startPeriod'),
              '0s',
            )}
          </div>
        </>
      )}
    </div>
  );
}
