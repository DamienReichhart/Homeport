import { CopyButton } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Text helm printed (values, notes, a manifest), read-only with Copy. */
export function TextBlock({ text, label, empty }: { text: string; label: string; empty: string }) {
  if (!text.trim()) return <p className="text-muted-foreground text-sm">{empty}</p>;
  return (
    <div className="relative">
      <div className="absolute top-2 end-2">
        <CopyButton text={text} label={t('helm.copy')} />
      </div>
      <pre
        aria-label={label}
        className="bg-muted max-h-[70vh] overflow-auto rounded-md p-3 font-mono text-xs whitespace-pre"
      >
        {text}
      </pre>
    </div>
  );
}
