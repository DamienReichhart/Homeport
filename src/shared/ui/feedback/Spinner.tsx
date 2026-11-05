import { Loader2Icon } from 'lucide-react';
import { t } from '@/shared/i18n';

export function Spinner({ label = t('common.loading') }: { label?: string }) {
  return (
    <p role="status" className="text-muted-foreground flex items-center gap-2 text-sm">
      <Loader2Icon className="size-4 animate-spin" />
      {label}
    </p>
  );
}
