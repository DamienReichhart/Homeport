import { Loader2Icon } from 'lucide-react';

export function Spinner({ label = 'Loading…' }: { label?: string }) {
  return (
    <p role="status" className="text-muted-foreground flex items-center gap-2 text-sm">
      <Loader2Icon className="size-4 animate-spin" />
      {label}
    </p>
  );
}
