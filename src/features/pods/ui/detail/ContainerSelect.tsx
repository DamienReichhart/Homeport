import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Picks one of a Pod's containers, init containers marked. */
export function ContainerSelect({
  containers,
  value,
  onChange,
}: {
  containers: { name: string; init?: boolean }[];
  value: string;
  onChange: (name: string) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger className="w-44" size="sm" aria-label={t('pods.logs.container')}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {containers.map((c) => (
          <SelectItem key={c.name} value={c.name}>
            {c.init ? t('pods.logs.initContainer', { name: c.name }) : c.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
