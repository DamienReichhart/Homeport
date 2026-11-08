import { ArrowRightLeftIcon } from 'lucide-react';
import { useState } from 'react';
import type { ForwardTarget } from '@/shared/api';
import { Button } from '@/shared/ui';
import { ForwardDialog } from './ForwardDialog';
import { t } from '@/shared/i18n';

/** "Forward…" next to a Pod's or Service's port. */
export function ForwardButton({
  clusterId,
  target,
  port,
}: {
  clusterId: string;
  target: ForwardTarget;
  port: number;
}) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button
        variant="ghost"
        size="sm"
        onClick={(e) => {
          e.stopPropagation();
          setOpen(true);
        }}
        aria-label={t('portforwards.forwardPort', { port })}
      >
        <ArrowRightLeftIcon /> {t('portforwards.forward')}
      </Button>
      {open && (
        <ForwardDialog
          clusterId={clusterId}
          target={target}
          port={port}
          onClose={() => setOpen(false)}
        />
      )}
    </>
  );
}
