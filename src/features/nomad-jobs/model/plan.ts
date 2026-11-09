import type { NomadPlanGroup, SpecFormat } from '@/shared/api';

/** The format the dialog shows for `spec`: JSON when it starts with `{`, HCL otherwise. */
export const specFormat = (spec: string): SpecFormat =>
  spec.trimStart().startsWith('{') ? 'json' : 'hcl2';

/** Whether a plan changes a task group's allocations at all. */
export function groupChanges(group: NomadPlanGroup): boolean {
  return (
    group.place + group.stop + group.inPlace + group.destructive + group.migrate + group.canary >
      0 || group.fields.length > 0
  );
}

/** What a plan does to a task group's allocations, as counts that aren't zero. */
export function groupUpdates(group: NomadPlanGroup): { key: UpdateKey; count: number }[] {
  const all: [UpdateKey, number][] = [
    ['place', group.place],
    ['stop', group.stop],
    ['inPlace', group.inPlace],
    ['destructive', group.destructive],
    ['migrate', group.migrate],
    ['canary', group.canary],
  ];
  return all.filter(([, count]) => count > 0).map(([key, count]) => ({ key, count }));
}

export type UpdateKey = 'place' | 'stop' | 'inPlace' | 'destructive' | 'migrate' | 'canary';
