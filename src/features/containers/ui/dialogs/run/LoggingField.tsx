import { Input, Textarea } from '@/shared/ui';
import type { LoggingForm } from '../../../model/runAdvanced';
import { Field } from './Field';

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
      <Field id="run-log-driver" label="Log driver" hint="With none, the Logs tab stays empty.">
        <Input
          id="run-log-driver"
          list="run-log-drivers"
          value={value.driver}
          placeholder="The Engine's default"
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
        label="Driver options"
        hint="One key=value per line; max-size and max-file rotate json-file logs."
      >
        <Textarea
          id="run-log-options"
          value={value.options}
          placeholder={'max-size=10m\nmax-file=3'}
          className="h-16 font-mono text-xs"
          spellCheck={false}
          onChange={(e) => onChange({ ...value, options: e.target.value })}
        />
      </Field>
    </div>
  );
}
