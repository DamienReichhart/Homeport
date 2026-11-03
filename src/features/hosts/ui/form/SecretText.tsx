import { FileUpIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Button, Textarea } from '@/shared/ui';
import { loadTextFile } from '../../api/queries';
import { Field } from './Field';

/** A PEM or key textarea that can also be filled from a file. */
export function SecretText({
  label,
  value,
  onChange,
  placeholder,
  hint,
}: {
  label: string;
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
            const text = await loadTextFile(`Choose the ${label.toLowerCase()}`);
            if (text !== null) onChange(text);
          }}
        >
          <FileUpIcon /> Load from file…
        </Button>
      </div>
    </Field>
  );
}
