import { RotateCcwIcon } from 'lucide-react';
import { Button, Label, YamlEditor } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The values helm renders the chart with, as YAML, with a way back to `reset`. */
export function ValuesField({
  value,
  onChange,
  reset,
  resetLabel,
}: {
  value: string;
  onChange: (values: string) => void;
  /** What Reset puts back, `null` while it isn't known. */
  reset: string | null;
  resetLabel: string;
}) {
  return (
    <div className="grid gap-2">
      <div className="flex items-center justify-between gap-2">
        <Label>{t('helm.install.values')}</Label>
        <Button
          type="button"
          size="sm"
          variant="ghost"
          disabled={reset === null || reset === value}
          onClick={() => reset !== null && onChange(reset)}
        >
          <RotateCcwIcon /> {resetLabel}
        </Button>
      </div>
      <YamlEditor
        value={value}
        onChange={onChange}
        label={t('helm.install.valuesLabel')}
        height="18rem"
      />
    </div>
  );
}
