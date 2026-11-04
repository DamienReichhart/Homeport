import { RefreshCwIcon } from 'lucide-react';
import { Button } from '@/shared/ui';
import { useCheckForNewerImages } from '../api/mutations';
import { t } from '@/shared/i18n';

/** "Check for newer images": asks every registry again, skipping the cache. */
export function CheckNewerImages({ hostId }: { hostId: string }) {
  const check = useCheckForNewerImages(hostId);
  return (
    <Button variant="outline" disabled={check.isPending} onClick={() => check.mutate()}>
      <RefreshCwIcon className={check.isPending ? 'animate-spin' : undefined} />
      {check.isPending ? t('core.freshness.checking') : t('core.freshness.check')}
    </Button>
  );
}
