import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import type { NomadNamespaceSelection } from '../selection/useNomadNamespace';
import { t } from '@/shared/i18n';

/** Stands for every namespace in the picker, which can't hold `null`. */
const ALL = '\u0000all';

/** Picks the namespace a Nomad page shows, "All namespaces" first; nothing when unlistable. */
export function NomadNamespacePicker({ selection }: { selection: NomadNamespaceSelection }) {
  if (!selection.listable) return null;
  return (
    <Select
      value={selection.choice ?? ALL}
      onValueChange={(value) => selection.choose(value === ALL ? null : value)}
    >
      <SelectTrigger className="w-48" aria-label={t('nomad.namespace.label')}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={ALL}>{t('nomad.namespace.all')}</SelectItem>
        {selection.namespaces.map((n) => (
          <SelectItem key={n.name} value={n.name}>
            {n.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
