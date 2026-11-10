import { FileUpIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Button, Textarea } from '@/shared/ui';
import { loadTextFile } from '../../api/queries';
import { Field } from './Field';
import { t } from '@/shared/i18n';

/** A PEM or key textarea that can also be filled from a file, picked in a `pickTitle` dialog. */
export function SecretText({
  label,
  pickTitle,
  value,
  onChange,
  placeholder,
  hint,
}: {
  label: string;
  pickTitle: string;
  value: string;
  onChange: (value: string) => void;
  placeholder: string;
  hint?: ReactNode;
}) {
  return (
    <Field label={label} hint={hint}>
      <SecretTextArea
        value={value}
        onChange={onChange}
        placeholder={placeholder}
        pickTitle={pickTitle}
      />
    </Field>
  );
}

/**
 * The textarea and its file button. `Field` hands its child the id its label
 * points to and the hint's id: they belong on the textarea, not on the box
 * around it, or the field has no accessible name.
 */
function SecretTextArea({
  id,
  'aria-describedby': describedBy,
  pickTitle,
  value,
  onChange,
  placeholder,
}: {
  id?: string;
  'aria-describedby'?: string;
  pickTitle: string;
  value: string;
  onChange: (value: string) => void;
  placeholder: string;
}) {
  return (
    <div className="grid gap-1.5">
      <Textarea
        id={id}
        aria-describedby={describedBy}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        className="h-24 font-mono text-xs"
        spellCheck={false}
      />
      <Button
        type="button"
        variant="outline"
        size="sm"
        className="justify-self-start"
        onClick={async () => {
          const text = await loadTextFile(pickTitle);
          if (text !== null) onChange(text);
        }}
      >
        <FileUpIcon /> {t('hosts.form.loadFromFile')}
      </Button>
    </div>
  );
}
