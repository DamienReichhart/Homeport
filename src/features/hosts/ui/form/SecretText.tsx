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
      <div className="grid gap-1.5">
        <Textarea
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
    </Field>
  );
}
