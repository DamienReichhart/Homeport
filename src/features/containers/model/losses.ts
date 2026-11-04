import type { Loss } from '@/shared/api';

/** One line of Edit's confirmation per thing the recreated container won't have. */
export function lossText(loss: Loss): string {
  switch (loss.kind) {
    case 'anonymousVolume':
      return `The anonymous volume at ${loss.path}: the new container starts with an empty one. The old one is kept as an unused volume.`;
    case 'filesystemChanges':
      return `${loss.count === 1 ? '1 file' : `${loss.count} files`} changed inside the container since it was created. Commit it to an image first to keep them.`;
    case 'downtime':
      return 'It stops while the two are swapped, usually for a few seconds.';
    case 'stackRedeploy':
      return `It belongs to the ${loss.stack} Stack: the Stack's next deploy recreates it from its compose file, without this edit.`;
  }
}
