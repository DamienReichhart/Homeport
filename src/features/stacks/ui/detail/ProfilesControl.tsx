import { useState } from 'react';
import type { SourceKind } from '@/shared/api';
import { Button, Checkbox, ErrorMessage, Label } from '@/shared/ui';
import { useSetProfiles } from '../../api/mutations';
import { splitProfiles, toggle } from '../../model/profiles';
import { t } from '@/shared/i18n';

/**
 * Which of the profiles a Stack's definition declares it deploys, besides
 * its services without one. Hidden when the definition declares none.
 */
export function ProfilesControl({
  kind,
  stackId,
  declared,
  active,
  canDeploy,
  onDeploy,
}: {
  kind: SourceKind;
  stackId: string;
  /** From the definition as it is in the editor; null until checked. */
  declared: string[] | null;
  /** As stored. */
  active: string[];
  canDeploy: boolean;
  onDeploy: () => void;
}) {
  const [chosen, setChosen] = useState(active);
  const save = useSetProfiles(kind, stackId);
  if (!declared || (declared.length === 0 && active.length === 0)) return null;
  const { dropped } = splitProfiles(active, declared);
  const next = splitProfiles(chosen, declared).kept;
  const changed = dropped.length > 0 || next.join() !== active.join();
  const submit = (deploy: boolean) =>
    save.mutate(next, { onSuccess: () => (deploy ? onDeploy() : undefined) });

  return (
    <fieldset className="grid gap-2">
      <legend className="mb-1 text-sm font-medium">{t('stacks.profiles.title')}</legend>
      <p className="text-muted-foreground text-xs">{t('stacks.profiles.hint')}</p>
      {declared.map((profile) => (
        <div key={profile} className="flex items-center gap-2">
          <Checkbox
            id={`profile-${profile}`}
            checked={next.includes(profile)}
            onCheckedChange={(on) => setChosen(toggle(declared, next, profile, on === true))}
          />
          <Label htmlFor={`profile-${profile}`} className="font-mono text-sm">
            {profile}
          </Label>
        </div>
      ))}
      {dropped.length > 0 && (
        <p className="text-warning text-xs">
          {t('stacks.profiles.dropped', {
            count: dropped.length,
            profiles: dropped.join(', '),
          })}
        </p>
      )}
      {save.error && <ErrorMessage error={save.error} />}
      {changed && (
        <div className="flex flex-wrap gap-2">
          <Button size="sm" disabled={save.isPending || !canDeploy} onClick={() => submit(true)}>
            {t('stacks.profiles.saveAndRedeploy')}
          </Button>
          <Button
            size="sm"
            variant="outline"
            disabled={save.isPending}
            onClick={() => submit(false)}
          >
            {t('stacks.profiles.save')}
          </Button>
        </div>
      )}
    </fieldset>
  );
}
