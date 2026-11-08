import { SearchIcon } from 'lucide-react';
import { useRef } from 'react';
import { useShortcut } from '@/shared/hooks';
import { Input } from '../primitives/input';
import { t } from '@/shared/i18n';

/** A page's search field; `/` focuses it. */
export function SearchInput({
  value,
  onChange,
  placeholder = t('ui.search'),
}: {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}) {
  const input = useRef<HTMLInputElement>(null);
  useShortcut({
    keys: '/',
    label: t('ui.shortcuts.focusSearch'),
    group: t('ui.shortcuts.thisPage'),
    handler: () => {
      input.current?.focus();
      input.current?.select();
    },
  });
  return (
    <div className="relative w-64">
      <SearchIcon className="text-muted-foreground pointer-events-none absolute top-1/2 left-2 size-4 -translate-y-1/2" />
      <Input
        ref={input}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === 'Escape' && value === '') e.currentTarget.blur();
        }}
        placeholder={placeholder}
        className="pl-8"
        aria-label={placeholder}
        aria-keyshortcuts="/"
      />
    </div>
  );
}
