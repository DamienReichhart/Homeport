import { useState } from 'react';
import { ArrowUpCircleIcon, XIcon } from 'lucide-react';
import { Button } from '@/shared/ui';
import { useUpdateCheck } from '../api';
import { UpdateActions } from './UpdateActions';

/** A bar above every page while a newer version is available, until dismissed. */
export function UpdateNotice() {
  const check = useUpdateCheck();
  const [dismissed, setDismissed] = useState<string>();
  const available = check.data?.available;
  if (!check.data || !available || dismissed === available.version) return null;

  const viaPackageManager = check.data.method.kind === 'packageManager';
  return (
    <div
      role="status"
      aria-label="Update available"
      className="bg-primary/10 flex flex-wrap items-center gap-x-3 gap-y-2 border-b px-6 py-2 text-sm"
    >
      <ArrowUpCircleIcon className="text-primary size-4" />
      <span>
        <strong className="font-medium">Homeport {available.version}</strong> is available
        {viaPackageManager ? '. Update it with your package manager:' : '.'}
      </span>
      <UpdateActions check={check.data} />
      <Button
        variant="ghost"
        size="icon-sm"
        className="ml-auto"
        aria-label="Dismiss"
        onClick={() => setDismissed(available.version)}
      >
        <XIcon />
      </Button>
    </div>
  );
}
