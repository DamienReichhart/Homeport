/**
 * The steps of backing up, restoring or cloning a volume, each through a
 * helper container (ADR 0015): check which running containers use it, get
 * the helper's image, then run, stopping those containers meanwhile when
 * the user chose to.
 */
export type FlowStep =
  | { kind: 'checking' }
  /** Running containers use the volume; the user chooses what to do. */
  | { kind: 'running'; users: string[] }
  /** The helper's image is being checked for, or pulled. */
  | { kind: 'image'; stop: string[] }
  | { kind: 'transfer'; stop: string[]; pulledImage: boolean; replace: boolean }
  /** A restore found files in the volume; the user confirms replacing them. */
  | { kind: 'replace'; stop: string[]; pulledImage: boolean };

export type FlowEvent =
  | { type: 'checked'; users: string[] }
  /** Go on with `stop` stopped for the duration; empty to leave them running. */
  | { type: 'proceed'; stop: string[] }
  | { type: 'imageReady'; pulledImage: boolean }
  | { type: 'notEmpty' }
  | { type: 'replace' };

export const startFlow: FlowStep = { kind: 'checking' };

export function reduceFlow(step: FlowStep, event: FlowEvent): FlowStep {
  switch (event.type) {
    case 'checked':
      return step.kind === 'checking'
        ? event.users.length > 0
          ? { kind: 'running', users: event.users }
          : { kind: 'image', stop: [] }
        : step;
    case 'proceed':
      return step.kind === 'running' ? { kind: 'image', stop: event.stop } : step;
    case 'imageReady':
      return step.kind === 'image'
        ? { kind: 'transfer', stop: step.stop, pulledImage: event.pulledImage, replace: false }
        : step;
    case 'notEmpty':
      return step.kind === 'transfer' && !step.replace
        ? { kind: 'replace', stop: step.stop, pulledImage: step.pulledImage }
        : step;
    case 'replace':
      return step.kind === 'replace'
        ? { kind: 'transfer', stop: step.stop, pulledImage: step.pulledImage, replace: true }
        : step;
  }
}

/** A suggested volume name from a backup's file name: `db-2026.tar` → `db-2026`. */
export function volumeNameFromFile(path: string): string {
  const file = path.split(/[\\/]/).pop() ?? '';
  return file
    .replace(/\.tar$/i, '')
    .replace(/[^A-Za-z0-9_.-]/g, '-')
    .replace(/^[^A-Za-z0-9]+/, '');
}
