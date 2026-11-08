import type { QuadletUnit, QuadletUnits } from '@/shared/api';
import {
  actBlocked,
  baseName,
  canSaveUnit,
  emptyUnitForm,
  filesBlocked,
  findUnit,
  isActive,
  kindOf,
  template,
  toInstall,
  unitFormErrors,
  unitActions,
  unitStatusLabel,
  unitTone,
} from './units';

const unit = (patch: Partial<QuadletUnit> = {}): QuadletUnit => ({
  name: 'web.container',
  unitName: 'web.service',
  path: '/home/me/.config/containers/systemd/web.container',
  status: 'active/running',
  app: null,
  pod: null,
  scope: 'user',
  containers: [],
  ...patch,
});

const units = (patch: Partial<QuadletUnits> = {}): QuadletUnits => ({
  units: [unit()],
  source: 'api',
  podmanVersion: '5.8.1',
  managesFiles: true,
  runsSystemctl: true,
  ...patch,
});

describe('Quadlet units', () => {
  it('are found by their file or their service', () => {
    const list = [unit(), unit({ name: null, unitName: 'old.service' })];
    expect(findUnit(list, 'web.container')).toBe(list[0]);
    expect(findUnit(list, 'old.service')).toBe(list[1]);
    expect(findUnit(list, 'nope')).toBeNull();
    expect(kindOf(list[0]!)).toBe('container');
    expect(kindOf(list[1]!)).toBe('service');
  });

  it('read systemd’s and Podman’s statuses', () => {
    expect(isActive('active/running')).toBe(true);
    expect(isActive('running')).toBe(true);
    expect(isActive('inactive/dead')).toBe(false);
    expect(unitTone('failed/failed')).toBe('danger');
    expect(unitTone('Not loaded')).toBe('warning');
    expect(unitTone('activating/start')).toBe('info');
    expect(unitStatusLabel('active/exited')).toBe('quadlets.status.running');
    expect(unitStatusLabel('inactive/dead')).toBe('quadlets.status.stopped');
    expect(unitStatusLabel('loaded template')).toBe('quadlets.status.template');
  });

  it('offer the actions that fit their status', () => {
    expect(unitActions('active/running', true)).toEqual(['stop']);
    expect(unitActions('inactive/dead', true)).toEqual(['start']);
    expect(unitActions('active/running', false)).toEqual(['stop', 'restart']);
    expect(unitActions('failed/failed', false)).toEqual(['start']);
  });

  it('say why they can’t be started or edited from here', () => {
    expect(actBlocked(units(), unit())).toBeNull();
    expect(actBlocked(units({ runsSystemctl: false }), unit())).toBe('quadlets.blocked.noCommands');
    expect(actBlocked(units(), unit({ unitName: null }))).toBe('quadlets.blocked.notLoaded');
    expect(filesBlocked(units())).toBeNull();
    expect(filesBlocked(units({ managesFiles: false }))).toBe('quadlets.blocked.podmanTooOld');
  });
});

describe('unit form', () => {
  it('starts from a working container unit', () => {
    expect(canSaveUnit(emptyUnitForm())).toBe(true);
    expect(template('pod')).toContain('[Pod]');
    expect(template('volume')).toBe('[Volume]\n');
  });

  it('checks the name, the section and the extra files', () => {
    const form = emptyUnitForm();
    expect(unitFormErrors({ ...form, name: 'web.service' }).name).toBe('quadlets.form.nameInvalid');
    expect(unitFormErrors({ ...form, name: 'web.pod' }).content).toBe(
      'quadlets.form.sectionMissing',
    );
    const extra = (name: string) => ({ ...form, extra: [{ name, content: '' }] });
    expect(unitFormErrors(extra('a/b')).extra).toBe('quadlets.form.extraInvalid');
    expect(unitFormErrors(extra('web.container')).extra).toBe('quadlets.form.extraInvalid');
    expect(unitFormErrors(extra('Containerfile'))).toEqual({});
  });

  it('builds the install request into the unit’s application', () => {
    const form = {
      name: ' web.container ',
      content: '[Container]\n',
      extra: [{ name: 'web.env', content: 'A=1' }],
    };
    expect(toInstall(form, 'shop')).toEqual({
      unit: { name: 'web.container', content: '[Container]\n' },
      extra: [{ name: 'web.env', content: 'A=1' }],
      app: 'shop',
    });
    expect(baseName('/home/me/app/Containerfile')).toBe('Containerfile');
    expect(baseName('C:\\Users\\me\\kube.yaml')).toBe('kube.yaml');
  });
});
