import type { NomadPlanGroup } from '@/shared/api';
import { groupChanges, groupUpdates, specFormat } from './plan';

const group = (overrides: Partial<NomadPlanGroup>): NomadPlanGroup => ({
  name: 'api',
  change: 'None',
  place: 0,
  stop: 0,
  inPlace: 0,
  destructive: 0,
  migrate: 0,
  canary: 0,
  ignore: 2,
  fields: [],
  failure: null,
  ...overrides,
});

describe('a plan', () => {
  it('tells HCL from JSON by the first character', () => {
    expect(specFormat('  {"Job": {}}')).toBe('json');
    expect(specFormat('job "web" {}')).toBe('hcl2');
  });

  it('lists only the updates that happen', () => {
    expect(groupUpdates(group({ place: 1, destructive: 2 }))).toEqual([
      { key: 'place', count: 1 },
      { key: 'destructive', count: 2 },
    ]);
    expect(groupChanges(group({}))).toBe(false);
    expect(
      groupChanges(group({ fields: [{ path: 'Count', change: 'Edited', old: '1', new: '2' }] })),
    ).toBe(true);
  });
});
