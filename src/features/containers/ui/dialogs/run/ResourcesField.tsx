import { Input } from '@/shared/ui';
import type { AdvancedErrors, ResourcesForm } from '../../../model/runAdvanced';
import { describedBy } from './describedBy';
import { Field } from './Field';
import { t } from '@/shared/i18n';

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
      <Field
        id="run-cpus"
        label={t('containers.advanced.cpus')}
        error={errors.cpus}
        hint={t('containers.advanced.cpusHint')}
      >
        {input('cpus', 'run-cpus', t('containers.advanced.cpusPlaceholder'))}
      </Field>
      <Field
        id="run-cpu-shares"
        label={t('containers.advanced.cpuShares')}
        error={errors.cpuShares}
        hint={t('containers.advanced.cpuSharesHint')}
      >
        {input('cpuShares', 'run-cpu-shares', '1024')}
      </Field>
      <Field
        id="run-memory"
        label={t('containers.advanced.memory')}
        error={errors.memory}
        hint={t('containers.advanced.memoryHint')}
      >
        {input('memory', 'run-memory', t('containers.advanced.memoryPlaceholder'))}
      </Field>
      <Field
        id="run-memory-reservation"
        label={t('containers.advanced.memoryReservation')}
        error={errors.memoryReservation}
        hint={t('containers.advanced.memoryReservationHint')}
      >
        {input('memoryReservation', 'run-memory-reservation', t('containers.advanced.none'))}
      </Field>
      <Field
        id="run-memory-swap"
        label={t('containers.advanced.memorySwap')}
        error={errors.memorySwap}
        hint={t('containers.advanced.memorySwapHint')}
      >
        {input('memorySwap', 'run-memory-swap', t('containers.advanced.memorySwapPlaceholder'))}
      </Field>
    </div>
  );
}
