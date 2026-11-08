import { useQuery } from '@tanstack/react-query';
import { Checkbox, Input, Label } from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import type { ObjectPick } from '../../model/serviceForm';
import { t } from '@/shared/i18n';

/** Picks secrets or configs of the Swarm for a service, each with where it's mounted. */
export function ObjectPicker({
  swarmId,
  kind,
  picked,
  onChange,
}: {
  swarmId: string;
  kind: 'secret' | 'config';
  picked: ObjectPick[];
  onChange: (picked: ObjectPick[]) => void;
}) {
  const objects = useQuery(swarmFeatureQueries.objects(swarmId, kind));
  const label = kind === 'secret' ? t('swarm.create.secrets') : t('swarm.create.configs');
  if (!objects.data?.length) return null;
  return (
    <div className="grid gap-1.5">
      <Label>{label}</Label>
      {objects.data.map((o) => {
        const pick = picked.find((p) => p.name === o.name);
        return (
          <div key={o.id} className="flex items-center gap-2">
            <label className="flex min-w-40 items-center gap-2 text-sm">
              <Checkbox
                checked={!!pick}
                onCheckedChange={(on) =>
                  onChange(
                    on === true
                      ? [...picked, { name: o.name, target: '' }]
                      : picked.filter((p) => p.name !== o.name),
                  )
                }
              />
              {o.name}
            </label>
            {pick && (
              <Input
                aria-label={t('swarm.create.mountedAt', { name: o.name })}
                placeholder={
                  kind === 'secret'
                    ? t('swarm.create.secretTarget', { name: o.name })
                    : t('swarm.create.configTarget', { name: o.name })
                }
                value={pick.target}
                onChange={(e) =>
                  onChange(
                    picked.map((p) => (p.name === o.name ? { ...p, target: e.target.value } : p)),
                  )
                }
              />
            )}
          </div>
        );
      })}
    </div>
  );
}
