import { useQuery } from '@tanstack/react-query';
import { FolderTreeIcon, SettingsIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui';
import { containerdQueries, useContainerdNamespace } from '../api/containerd';
import { supports } from '../model/engine';
import { useHostSelection } from '../selection/context';
import { t } from '@/shared/i18n';

const MANAGE = '__manage__';

function Picker({ hostId }: { hostId: string }) {
  const navigate = useNavigate();
  const [namespace, choose] = useContainerdNamespace(hostId);
  const namespaces = useQuery(containerdQueries.namespaces(hostId));
  const names = namespaces.data?.map((n) => n.name) ?? [];
  // The remembered one may be gone, or still loading: list it anyway.
  if (namespace && !names.includes(namespace)) names.unshift(namespace);
  return (
    <Select
      value={namespace ?? ''}
      onValueChange={(next) => {
        if (next === MANAGE) void navigate('/containerd/namespaces');
        else choose(next);
      }}
    >
      <SelectTrigger size="sm" className="w-full" aria-label={t('core.containerd.namespace')}>
        <FolderTreeIcon className="text-muted-foreground" />
        <SelectValue placeholder={t('core.containerd.namespace')} />
      </SelectTrigger>
      <SelectContent>
        {names.map((name) => {
          const n = namespaces.data?.find((d) => d.name === name);
          return (
            <SelectItem key={name} value={name}>
              <span className="truncate">{name}</span>
              {n && (
                <span className="text-muted-foreground text-xs">
                  {t('core.containerd.holds', { containers: n.containers, images: n.images })}
                </span>
              )}
            </SelectItem>
          );
        })}
        <SelectSeparator />
        <SelectItem value={MANAGE}>
          <SettingsIcon /> {t('core.containerd.manage')}
        </SelectItem>
      </SelectContent>
    </Select>
  );
}

/**
 * The containerd namespace the pages show, under the Host switcher while a
 * connected containerd Host is selected.
 */
export function ContainerdNamespacePicker() {
  const { host } = useHostSelection();
  if (!host || !supports(host, 'containerd') || host.status.state !== 'connected') return null;
  return <Picker key={host.host.id} hostId={host.host.id} />;
}
