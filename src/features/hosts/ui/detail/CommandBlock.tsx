import { CopyButton } from '@/shared/ui';

/** A shell command to run on the server, with a copy button. */
export function CommandBlock({ label, command }: { label: string; command: string }) {
  return (
    <div className="grid gap-1.5">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium">{label}</span>
        <CopyButton text={command} />
      </div>
      <pre className="bg-muted/50 overflow-x-auto rounded-md border p-2 font-mono text-xs whitespace-pre-wrap break-all">
        {command}
      </pre>
    </div>
  );
}
