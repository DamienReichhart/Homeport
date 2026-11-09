import type { NomadPlan, NomadPlanGroup } from '@/shared/api';
import { Badge, ErrorMessage } from '@/shared/ui';
import { groupChanges, groupUpdates, type UpdateKey } from '../model/plan';
import { t } from '@/shared/i18n';
import type { MessageKey } from '@/shared/i18n';

const UPDATE_LABELS: Record<UpdateKey, MessageKey> = {
  place: 'nomad.plan.place',
  stop: 'nomad.plan.stop',
  inPlace: 'nomad.plan.inPlace',
  destructive: 'nomad.plan.destructive',
  migrate: 'nomad.plan.migrate',
  canary: 'nomad.plan.canary',
};

function Group({ group }: { group: NomadPlanGroup }) {
  const updates = groupUpdates(group);
  return (
    <div className="grid gap-2 rounded-md border p-3">
      <div className="flex flex-wrap items-center gap-2">
        <span className="font-medium">{group.name}</span>
        <Badge variant="outline">{group.change}</Badge>
        {updates.map((u) => (
          <span key={u.key} className="text-muted-foreground text-sm">
            {t(UPDATE_LABELS[u.key], { count: u.count })}
          </span>
        ))}
        {!groupChanges(group) && (
          <span className="text-muted-foreground text-sm">{t('nomad.plan.unchanged')}</span>
        )}
      </div>
      {group.fields.length > 0 && (
        <ul className="grid gap-1 font-mono text-xs">
          {group.fields.map((f) => (
            <li key={`${f.path}/${f.change}`}>
              <span className="text-muted-foreground">{f.path}: </span>
              {f.old !== null && <span className="text-destructive line-through">{f.old}</span>}
              {f.old !== null && f.new !== null && ' → '}
              {f.new !== null && <span className="text-success">{f.new}</span>}
            </li>
          ))}
        </ul>
      )}
      {group.failure && (
        <p className="text-warning text-sm">
          {t('nomad.plan.failure', {
            evaluated: group.failure.nodesEvaluated,
            filtered: group.failure.nodesFiltered,
            exhausted: group.failure.nodesExhausted,
          })}
          {group.failure.exhausted.length > 0 &&
            ` ${t('nomad.plan.exhausted', { resources: group.failure.exhausted.join(', ') })}`}
          {group.failure.constraints.length > 0 &&
            ` ${t('nomad.plan.constraints', { constraints: group.failure.constraints.join(', ') })}`}
        </p>
      )}
    </div>
  );
}

/** What Nomad says running a job would change, task group by task group. */
export function PlanView({ plan }: { plan: NomadPlan }) {
  return (
    <div className="grid gap-3">
      <p className="text-sm">
        {plan.newJob
          ? t('nomad.plan.new', { job: plan.jobId, namespace: plan.namespace })
          : plan.change === 'None'
            ? t('nomad.plan.noChange', { job: plan.jobId, namespace: plan.namespace })
            : t('nomad.plan.edited', { job: plan.jobId, namespace: plan.namespace })}
      </p>
      {plan.placement && <ErrorMessage error={plan.placement} />}
      {plan.warnings && (
        <p className="text-warning text-sm whitespace-pre-wrap">
          {t('nomad.plan.warnings', { warnings: plan.warnings })}
        </p>
      )}
      <div className="grid gap-2">
        {plan.groups.map((g) => (
          <Group key={g.name} group={g} />
        ))}
      </div>
    </div>
  );
}
