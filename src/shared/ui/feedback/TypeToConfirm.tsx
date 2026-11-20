import { useId } from 'react';
import { Input } from '../primitives/input';
import { Label } from '../primitives/label';
import { t } from '@/shared/i18n';

/** Whether what was typed is the name asked for. */
export const typedMatches = (typed: string, name: string) => typed.trim() === name;

/**
 * The field of a confirmation that loses data for good: the user types the
 * name of what goes, so the button is never pressed by reflex.
 */
export function TypeToConfirm({
  name,
  value,
  onChange,
}: {
  name: string;
  value: string;
  onChange: (value: string) => void;
}) {
  const id = useId();
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={id} className="font-normal">
        {t('ui.confirm.typeName', { name })}
      </Label>
      <Input
        id={id}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        autoComplete="off"
        spellCheck={false}
        className="font-mono"
      />
    </div>
  );
}
