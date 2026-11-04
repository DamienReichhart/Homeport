import { XIcon } from 'lucide-react';
import { useId, useState } from 'react';
import { Badge, Input } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * A list of short values typed one at a time, with suggestions; Enter or a
 * comma adds the typed value, Backspace on an empty input removes the last.
 */
export function TokenInput({
  id,
  values,
  suggestions,
  placeholder,
  normalise = (v) => v,
  onChange,
}: {
  id: string;
  values: string[];
  suggestions: readonly string[];
  placeholder: string;
  normalise?: (value: string) => string;
  onChange: (values: string[]) => void;
}) {
  const [draft, setDraft] = useState('');
  const listId = useId();
  const add = () => {
    const value = normalise(draft.trim().replace(/,$/, ''));
    setDraft('');
    if (value && !values.includes(value)) onChange([...values, value]);
  };
  return (
    <div className="grid gap-1.5">
      {values.length > 0 && (
        <div className="flex flex-wrap gap-1">
          {values.map((v) => (
            <Badge key={v} variant="secondary" className="gap-1 font-mono">
              {v}
              <button
                type="button"
                aria-label={t('containers.run.removeValue', { value: v })}
                onClick={() => onChange(values.filter((x) => x !== v))}
              >
                <XIcon className="size-3" />
              </button>
            </Badge>
          ))}
        </div>
      )}
      <Input
        id={id}
        list={listId}
        value={draft}
        placeholder={placeholder}
        className="font-mono"
        onChange={(e) => setDraft(e.target.value)}
        onBlur={add}
        onKeyDown={(e) => {
          if (e.key === 'Enter' || e.key === ',') {
            e.preventDefault();
            add();
          } else if (e.key === 'Backspace' && draft === '' && values.length > 0) {
            onChange(values.slice(0, -1));
          }
        }}
      />
      <datalist id={listId}>
        {suggestions
          .filter((s) => !values.includes(s))
          .map((s) => (
            <option key={s} value={s} />
          ))}
      </datalist>
    </div>
  );
}
