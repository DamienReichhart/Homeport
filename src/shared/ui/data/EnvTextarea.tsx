import { EyeIcon, EyeOffIcon } from 'lucide-react';
import { useState } from 'react';
import { cn } from '@/shared/lib/utils';
import { Button } from '../primitives/button';
import { Textarea } from '../primitives/textarea';
import { t } from '@/shared/i18n';

/** `KEY=value` lines with each value replaced by dots; comments and blank lines kept. */
export function maskEnv(env: string): string {
  return env
    .split('\n')
    .map((line) => {
      if (/^\s*(#|$)/.test(line)) return line;
      const at = line.indexOf('=');
      return at < 0 ? line : `${line.slice(0, at + 1)}${'•'.repeat(8)}`;
    })
    .join('\n');
}

/**
 * A `.env` editor whose values stay hidden until asked for, so a password
 * isn't on screen while sharing it or with someone looking over a shoulder.
 * An empty one opens ready to type in.
 */
export function EnvTextarea({
  id,
  value,
  onChange,
  disabled,
  placeholder,
  className,
}: {
  id: string;
  value: string;
  onChange?: (value: string) => void;
  disabled?: boolean;
  placeholder?: string;
  className?: string;
}) {
  const [shown, setShown] = useState(value.trim() === '');
  return (
    <div className="relative">
      <Textarea
        id={id}
        value={shown ? value : maskEnv(value)}
        onChange={(e) => onChange?.(e.target.value)}
        readOnly={!shown || !onChange}
        disabled={disabled}
        placeholder={placeholder}
        spellCheck={false}
        className={cn('pe-24 font-mono text-xs', !shown && 'text-muted-foreground', className)}
      />
      {value.trim() !== '' && (
        <Button
          type="button"
          variant="outline"
          size="xs"
          className="bg-background absolute end-2 top-2"
          aria-controls={id}
          aria-pressed={shown}
          onClick={() => setShown(!shown)}
        >
          {shown ? <EyeOffIcon /> : <EyeIcon />}
          {shown ? t('ui.env.hide') : t('ui.env.show')}
        </Button>
      )}
    </div>
  );
}
