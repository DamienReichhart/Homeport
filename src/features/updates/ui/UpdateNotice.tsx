import { useState } from 'react';
import { ArrowUpCircleIcon, XIcon } from 'lucide-react';
import { Button } from '@/shared/ui';
import { useUpdateCheck } from '../api/queries';
import { UpdateActions } from './UpdateActions';
import { t, tRich } from '@/shared/i18n';

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
      aria-label={t('updates.notice.label')}
      className="bg-primary/10 flex flex-wrap items-center gap-x-3 gap-y-2 border-b px-6 py-2 text-sm"
    >
      <ArrowUpCircleIcon className="text-primary size-4" />
      <span>
        {tRich(
          viaPackageManager ? 'updates.notice.availablePackageManager' : 'updates.notice.available',
          {
            version: available.version,
            strong: (chunks) => <strong className="font-medium">{chunks}</strong>,
          },
        )}
      </span>
      <UpdateActions check={check.data} />
      <Button
        variant="ghost"
        size="icon-sm"
        className="ml-auto"
        aria-label={t('updates.notice.dismiss')}
        onClick={() => setDismissed(available.version)}
      >
        <XIcon />
      </Button>
    </div>
  );
}
