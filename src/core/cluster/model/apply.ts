import type { ApplyResult } from '@/shared/api';

/** What a preview says about applying: whether it can, and what it would do. */
export function previewSummary(result: ApplyResult) {
  const docs = result.documents;
  const failed = docs.filter((d) => d.action === 'failed');
  const conflicted = failed.length > 0 && failed.every((d) => d.conflicts.length > 0);
  const changes = docs.filter((d) => d.action === 'created' || d.action === 'changed').length;
  return {
    /** Every document can be applied as it is. */
    ready: failed.length === 0 && changes > 0,
    /** Applying would only fail on fields other managers own: it can be forced. */
    forceable: conflicted,
    nothingToDo: failed.length === 0 && changes === 0,
    changes,
    failed: failed.length,
  };
}
