import { Input, Textarea } from '@/shared/ui';
import type { LoggingForm } from '../../../model/runAdvanced';
import { Field } from './Field';
import { t } from '@/shared/i18n';

/** Log drivers most hosts have; any other name can be typed. */
const DRIVERS = ['json-file', 'local', 'journald', 'syslog', 'none'];

/** Where the container's output goes (`--log-driver`, `--log-opt`). */
export function LoggingField({
  value,
  onChange,
}: {
  value: LoggingForm;
  onChange: (value: LoggingForm) => void;
}) {
  return (
    <div className="grid gap-3 sm:grid-cols-2">
      <Field
        id="run-log-driver"
        label={t('containers.advanced.logDriver')}
        hint={t('containers.advanced.logDriverHint')}
      >
        <Input
          id="run-log-driver"
          list="run-log-drivers"
          value={value.driver}
          placeholder={t('containers.advanced.logDriverPlaceholder')}
          className="font-mono"
          onChange={(e) => onChange({ ...value, driver: e.target.value })}
        />
        <datalist id="run-log-drivers">
          {DRIVERS.map((d) => (
            <option key={d} value={d} />
          ))}
        </datalist>
      </Field>
      <Field
        id="run-log-options"
        label={t('containers.advanced.logOptions')}
        hint={t('containers.advanced.logOptionsHint')}
      >
        <Textarea
          id="run-log-options"
          value={value.options}
          // eslint-disable-next-line homeport/no-literal-ui-string -- sample driver options
          placeholder={'max-size=10m\nmax-file=3'}
          className="h-16 font-mono text-xs"
          spellCheck={false}
          onChange={(e) => onChange({ ...value, options: e.target.value })}
        />
      </Field>
    </div>
  );
}
