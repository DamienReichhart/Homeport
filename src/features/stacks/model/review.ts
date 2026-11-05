import type { DeployDiff, EnvChange, FileChange } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** What the review step shows. */
export type ReviewState = 'unknown' | 'unchanged' | 'changes';

export function reviewState(diff: DeployDiff): ReviewState {
  if (!diff.known) return 'unknown';
  return diff.unchanged ? 'unchanged' : 'changes';
}

export type FileChangeKind = 'added' | 'removed' | 'changed' | 'unchanged';

export function fileChange(file: FileChange): FileChangeKind {
  if (file.deployed === null) return 'added';
  if (file.candidate === null) return 'removed';
  return file.deployed === file.candidate ? 'unchanged' : 'changed';
}

export const FILE_CHANGE_LABEL: Record<FileChangeKind, MessageKey> = {
  added: 'stacks.review.added',
  removed: 'stacks.review.removed',
  changed: 'stacks.review.changed',
  unchanged: 'stacks.review.unchanged',
};

export const ENV_CHANGE_LABEL: Record<EnvChange, MessageKey> = {
  added: 'stacks.review.added',
  removed: 'stacks.review.removed',
  changed: 'stacks.review.changed',
  unchanged: 'stacks.review.unchanged',
};
