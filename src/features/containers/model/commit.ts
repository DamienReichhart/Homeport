import type { CommitSpec, Image } from '@/shared/api';

/** The form of the Commit dialog. */
export interface CommitForm {
  reference: string;
  comment: string;
  author: string;
  /** One Dockerfile instruction per line. */
  changes: string;
  pause: boolean;
}

/** `web:2026-11-04` for container `web`, lower-cased as image names must be. */
export function commitName(container: string, today = new Date()): string {
  const repo = container.toLowerCase().replace(/[^a-z0-9._/-]/g, '-');
  return `${repo}:${today.toISOString().slice(0, 10)}`;
}

export const emptyCommitForm = (container: string): CommitForm => ({
  reference: commitName(container),
  comment: '',
  author: '',
  changes: '',
  pause: true,
});

export function toCommitSpec(form: CommitForm): CommitSpec {
  const blank = (s: string) => (s.trim() === '' ? null : s.trim());
  return {
    reference: form.reference.trim(),
    comment: blank(form.comment),
    author: blank(form.author),
    pause: form.pause,
    changes: form.changes
      .split('\n')
      .map((l) => l.trim())
      .filter(Boolean),
  };
}

/** The image that already has this name, whose tag the commit moves. */
export function imageNamed(reference: string, images: readonly Image[]): Image | undefined {
  const name = reference.trim();
  const lastPart = name.slice(name.lastIndexOf('/') + 1);
  const full = lastPart.includes(':') ? name : `${name}:latest`;
  return images.find((i) => i.tags.includes(full));
}
