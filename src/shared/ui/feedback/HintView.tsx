import type { Hint } from '@/shared/api';
import { CopyButton } from '../data/CopyButton';

/** What to do about a failure: the advice, then each command ready to copy. */
export function HintView({ hint }: { hint: Hint }) {
  return (
    <div className="flex flex-col gap-2">
      {hint.text && <p className="text-foreground/90 text-sm">{hint.text}</p>}
      {hint.commands.map((command) => (
        <div
          key={command}
          className="bg-muted flex items-start justify-between gap-2 rounded-md py-1.5 pe-1.5 ps-3"
        >
          <pre className="text-foreground overflow-x-auto py-1 font-mono text-xs whitespace-pre-wrap">
            {command}
          </pre>
          <CopyButton text={command} />
        </div>
      ))}
    </div>
  );
}
