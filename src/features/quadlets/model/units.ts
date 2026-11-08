import type { QuadletInstall, QuadletUnit, QuadletUnits, UnitAction } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

/** How a unit is named in links: its file, or its service when only that is known. */
export const unitKey = (unit: QuadletUnit) => unit.name ?? unit.unitName ?? '';

/** The unit whose file or service is `key`. */
export const findUnit = (units: QuadletUnit[], key: string) =>
  units.find((u) => u.name === key || u.unitName === key) ?? null;

/** A unit file's kind, from its extension: `container`, `pod`, `volume`… */
export const kindOf = (unit: QuadletUnit) => unit.name?.split('.').pop() ?? 'service';

/** Whether the unit's service runs: systemd's `active/…`, or a running container. */
export const isActive = (status: string) =>
  status.startsWith('active') || status.startsWith('activating') || status === 'running';

/** The actions that apply to a unit in `status`: one in a row, all on its page. */
export function unitActions(status: string, compact: boolean): UnitAction[] {
  if (compact) return [isActive(status) ? 'stop' : 'start'];
  return isActive(status) ? ['stop', 'restart'] : ['start'];
}

/** A unit's status as a colour. */
export function unitTone(status: string): Tone {
  if (status.startsWith('activating') || status.startsWith('deactivating')) return 'info';
  if (isActive(status)) return 'success';
  if (status.startsWith('failed')) return 'danger';
  if (status === 'Not loaded') return 'warning';
  return 'muted';
}

/** A unit's status in words, from systemd's or Podman's. */
export function unitStatusLabel(status: string): MessageKey {
  if (status.startsWith('activating')) return 'quadlets.status.starting';
  if (status.startsWith('deactivating')) return 'quadlets.status.stopping';
  if (isActive(status)) return 'quadlets.status.running';
  if (status.startsWith('failed')) return 'quadlets.status.failed';
  if (status === 'Not loaded') return 'quadlets.status.notLoaded';
  if (status === 'loaded template') return 'quadlets.status.template';
  return 'quadlets.status.stopped';
}

/** Why an action on units can't be done on this Host, or `null` when it can. */
export type Blocked = MessageKey | null;

/** Why the unit's service can't be started, stopped or restarted from here. */
export function actBlocked(units: QuadletUnits, unit: QuadletUnit): Blocked {
  if (!units.runsSystemctl) return 'quadlets.blocked.noCommands';
  if (!unit.unitName) return 'quadlets.blocked.notLoaded';
  return null;
}

/** Why unit files can't be read, installed, edited or removed on this Host. */
export const filesBlocked = (units: QuadletUnits): Blocked =>
  units.managesFiles ? null : 'quadlets.blocked.podmanTooOld';

/** A file beside a unit, as the dialog edits it. */
export interface ExtraFile {
  name: string;
  content: string;
}

/** The **New unit** and **Edit** dialog's fields. */
export interface UnitForm {
  name: string;
  content: string;
  extra: ExtraFile[];
}

/** The unit kinds a new unit can be, each with the section it needs. */
export const UNIT_KINDS = [
  { extension: 'container', section: 'Container' },
  { extension: 'pod', section: 'Pod' },
  { extension: 'volume', section: 'Volume' },
  { extension: 'network', section: 'Network' },
  { extension: 'kube', section: 'Kube' },
  { extension: 'image', section: 'Image' },
  { extension: 'build', section: 'Build' },
  { extension: 'artifact', section: 'Artifact' },
] as const;

const sectionOf = (name: string) =>
  UNIT_KINDS.find((k) => name.endsWith(`.${k.extension}`))?.section ?? null;

/** A starting point for a new unit of `extension`, ready to edit. */
export function template(extension: string): string {
  switch (extension) {
    case 'container':
      return [
        '[Unit]',
        'Description=Web server',
        '',
        '[Container]',
        'Image=docker.io/library/nginx:latest',
        'PublishPort=8080:80',
        '',
        '[Service]',
        'Restart=always',
        '',
        '[Install]',
        'WantedBy=default.target',
        '',
      ].join('\n');
    case 'pod':
      return ['[Pod]', 'PublishPort=8080:80', '', '[Install]', 'WantedBy=default.target', ''].join(
        '\n',
      );
    default: {
      const section = sectionOf(`x.${extension}`) ?? 'Container';
      return `[${section}]\n`;
    }
  }
}

export const emptyUnitForm = (): UnitForm => ({
  name: 'web.container',
  content: template('container'),
  extra: [],
});

/** What's wrong with the form, by field; empty when it can be sent. */
export interface UnitFormErrors {
  name?: MessageKey;
  content?: MessageKey;
  extra?: MessageKey;
}

const NAME = /^[A-Za-z0-9][A-Za-z0-9_.@-]*\.[a-z]+$/;
const EXTRA = /^[^/\\-][^/\\]*$/;

export function unitFormErrors(form: UnitForm): UnitFormErrors {
  const errors: UnitFormErrors = {};
  const name = form.name.trim();
  const section = sectionOf(name);
  if (!NAME.test(name) || !section) errors.name = 'quadlets.form.nameInvalid';
  else if (!form.content.split('\n').some((l) => l.trim() === `[${section}]`))
    errors.content = 'quadlets.form.sectionMissing';
  const names = form.extra.map((f) => f.name.trim());
  if (
    names.some((n) => !EXTRA.test(n) || n === '.' || n === '..' || n === name) ||
    new Set(names).size !== names.length
  )
    errors.extra = 'quadlets.form.extraInvalid';
  return errors;
}

/** The section a unit called `name` needs, for the error message. */
export const neededSection = (name: string) => `[${sectionOf(name.trim()) ?? 'Container'}]`;

export const canSaveUnit = (form: UnitForm) => Object.keys(unitFormErrors(form)).length === 0;

/** The install request for the form, into `app` when the unit has one. */
export function toInstall(form: UnitForm, app: string | null = null): QuadletInstall {
  return {
    unit: { name: form.name.trim(), content: form.content },
    extra: form.extra.map((f) => ({ name: f.name.trim(), content: f.content })),
    app,
  };
}

/** The name of a file picked on this computer, without its folders. */
export const baseName = (path: string) => path.split(/[\\/]/).pop() ?? path;
