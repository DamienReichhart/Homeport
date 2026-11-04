import { Input } from '@/shared/ui';
import type { AdvancedErrors, ResourcesForm } from '../../../model/runAdvanced';
import { describedBy } from './describedBy';
import { Field } from './Field';

/** CPU and memory limits (`--cpus`, `--cpu-shares`, `--memory…`). */
export function ResourcesField({
  value,
  errors,
  onChange,
}: {
  value: ResourcesForm;
  errors: AdvancedErrors;
  onChange: (value: ResourcesForm) => void;
}) {
  const input = (key: keyof ResourcesForm, id: string, placeholder: string) => (
    <Input
      id={id}
      value={value[key]}
      placeholder={placeholder}
      onChange={(e) => onChange({ ...value, [key]: e.target.value })}
      {...describedBy(id, errors[key])}
    />
  );
  return (
    <div className="grid gap-3 sm:grid-cols-2">
      <Field id="run-cpus" label="CPU limit" error={errors.cpus} hint="CPUs, like 0.5 or 2.">
        {input('cpus', 'run-cpus', 'Every CPU')}
      </Field>
      <Field
        id="run-cpu-shares"
        label="CPU shares"
        error={errors.cpuShares}
        hint="Weight against other containers; 1024 is the default."
      >
        {input('cpuShares', 'run-cpu-shares', '1024')}
      </Field>
      <Field id="run-memory" label="Memory limit" error={errors.memory} hint="Like 512m or 2g.">
        {input('memory', 'run-memory', 'No limit')}
      </Field>
      <Field
        id="run-memory-reservation"
        label="Memory reservation"
        error={errors.memoryReservation}
        hint="A soft limit, kept when the host runs short."
      >
        {input('memoryReservation', 'run-memory-reservation', 'None')}
      </Field>
      <Field
        id="run-memory-swap"
        label="Memory and swap limit"
        error={errors.memorySwap}
        hint="Memory plus swap; -1 for unlimited swap."
      >
        {input('memorySwap', 'run-memory-swap', 'Twice the memory limit')}
      </Field>
    </div>
  );
}
