import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import {
  Input,
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui';
import { clusterResourceQueries } from '../api/resources';
import { t } from '@/shared/i18n';

const ALL = '__all__';

/**
 * Picks the Namespace a Cluster page shows, or every one. Credentials that
 * can't list Namespaces get a field to type the one they may use.
 */
export function NamespacePicker({
  clusterId,
  value,
  onChange,
}: {
  clusterId: string;
  value: string | null;
  onChange: (namespace: string | null) => void;
}) {
  const namespaces = useQuery(clusterResourceQueries.namespaces(clusterId));
  const [typed, setTyped] = useState(value ?? '');
  if (namespaces.error) {
    return (
      <Input
        aria-label={t('core.namespace.label')}
        className="w-48"
        placeholder={t('core.namespace.type')}
        title={t('core.namespace.cantList')}
        value={typed}
        onChange={(e) => setTyped(e.target.value)}
        onBlur={() => onChange(typed.trim() || null)}
        onKeyDown={(e) => {
          if (e.key === 'Enter') onChange(typed.trim() || null);
        }}
      />
    );
  }
  const names = namespaces.data?.map((n) => n.name) ?? [];
  if (value && !names.includes(value)) names.unshift(value);
  return (
    <Select value={value ?? ALL} onValueChange={(v) => onChange(v === ALL ? null : v)}>
      <SelectTrigger className="w-48" aria-label={t('core.namespace.label')}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={ALL}>{t('core.namespace.all')}</SelectItem>
        <SelectSeparator />
        {names.map((name) => (
          <SelectItem key={name} value={name}>
            {name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
