import { useQuery } from '@tanstack/react-query';
import { FolderKanbanIcon } from 'lucide-react';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import { incusQueries, useIncusProject } from '../api/incus';
import { supports } from '../model/engine';
import { useHostSelection } from '../selection/context';
import { t } from '@/shared/i18n';

function Picker({ hostId }: { hostId: string }) {
  const [project, choose] = useIncusProject(hostId);
  const projects = useQuery(incusQueries.projects(hostId));
  const names = projects.data?.map((p) => p.name) ?? [];
  // The chosen one may still be loading: list it anyway.
  if (project && !names.includes(project)) names.unshift(project);
  return (
    <Select value={project ?? ''} onValueChange={choose}>
      <SelectTrigger size="sm" className="w-full" aria-label={t('core.incus.project')}>
        <FolderKanbanIcon className="text-muted-foreground" />
        <SelectValue placeholder={t('core.incus.project')} />
      </SelectTrigger>
      <SelectContent>
        {names.map((name) => {
          const p = projects.data?.find((d) => d.name === name);
          return (
            <SelectItem key={name} value={name}>
              <span className="truncate">{name}</span>
              {p && (
                <span className="text-muted-foreground text-xs">
                  {t('core.incus.holds', { instances: p.instances, images: p.images })}
                </span>
              )}
            </SelectItem>
          );
        })}
      </SelectContent>
    </Select>
  );
}

/**
 * The Incus project the pages show, under the Host switcher while a
 * connected Incus Host is selected.
 */
export function IncusProjectPicker() {
  const { host } = useHostSelection();
  if (!host || !supports(host, 'incus') || host.status.state !== 'connected') return null;
  return <Picker key={host.host.id} hostId={host.host.id} />;
}
