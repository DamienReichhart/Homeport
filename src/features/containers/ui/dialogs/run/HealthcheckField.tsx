import { Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import type { AdvancedErrors, HealthcheckForm } from '../../../model/runAdvanced';
import { describedBy } from './describedBy';
import { Field } from './Field';

const MODES: { value: HealthcheckForm['mode']; label: string }[] = [
  { value: 'image', label: "The image's own, if any" },
  { value: 'custom', label: 'A command' },
  { value: 'disabled', label: "None (turn off the image's)" },
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
      <Field id="run-health-mode" label="Healthcheck">
        <Select
          value={value.mode}
          onValueChange={(mode) => onChange({ ...value, mode: mode as HealthcheckForm['mode'] })}
        >
          <SelectTrigger id="run-health-mode" aria-label="Healthcheck">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {MODES.map((m) => (
              <SelectItem key={m.value} value={m.value}>
                {m.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </Field>
      {value.mode === 'custom' && (
        <>
          <Field
            id="run-health-cmd"
            label="Check command"
            hint="Run with /bin/sh -c; exit code 0 means healthy."
          >
            <Input
              id="run-health-cmd"
              value={value.command}
              placeholder="wget -qO- http://localhost/ || exit 1"
              className="font-mono"
              onChange={(e) => onChange({ ...value, command: e.target.value })}
            />
          </Field>
          <div className="grid gap-3 sm:grid-cols-4">
            {duration('interval', 'run-health-interval', 'Interval', '30s')}
            {duration('timeout', 'run-health-timeout', 'Timeout', '30s')}
            <Field id="run-health-retries" label="Retries" error={errors.retries}>
              <Input
                id="run-health-retries"
                inputMode="numeric"
                value={value.retries}
                placeholder="3"
                onChange={(e) => onChange({ ...value, retries: e.target.value })}
                {...describedBy('run-health-retries', errors.retries)}
              />
            </Field>
            {duration('startPeriod', 'run-health-start', 'Start period', '0s')}
          </div>
        </>
      )}
    </div>
  );
}
