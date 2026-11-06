import { formatBytes } from '@/shared/lib';
import { Skeleton, Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Each volume's size by name, `null` when its driver doesn't report one; `undefined` while measuring. */
export type VolumeSizes = Partial<Record<string, number | null>> | undefined;

/** A volume's size, a placeholder while it's measured, or "—" when its driver doesn't say. */
export function VolumeSize({ sizes, name }: { sizes: VolumeSizes; name: string }) {
  if (sizes === undefined)
    return <Skeleton className="h-4 w-14" aria-label={t('volumes.size.measuring')} />;
  const size = sizes[name];
  if (typeof size === 'number') return formatBytes(size);
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span className="text-muted-foreground">—</span>
      </TooltipTrigger>
      <TooltipContent>{t('volumes.size.unknown')}</TooltipContent>
    </Tooltip>
  );
}
