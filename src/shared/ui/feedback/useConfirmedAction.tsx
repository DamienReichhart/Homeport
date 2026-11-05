import type { ReactNode } from 'react';
import { useConfirm, type ConfirmOptions } from './ConfirmDialog';

/** What to ask before an action, and the action itself. */
export interface ConfirmedAction extends Omit<ConfirmOptions, 'onConfirm'> {
  /**
   * Runs once the user confirms. It reports its own failure (a mutation's
   * `onError` toast) and rejects, which keeps the dialog open for a retry.
   * Never swallow the rejection: the dialog would close as if it had worked.
   */
  action: () => Promise<unknown>;
}

/**
 * `const [dialog, run] = useConfirmedAction()`: render `dialog`, call
 * `run({ title, description, confirmLabel, destructive, action })` to ask
 * first and act on confirmation. The dialog closes only when the action
 * resolves.
 */
export function useConfirmedAction(): [ReactNode, (request: ConfirmedAction) => void] {
  const [dialog, confirm] = useConfirm();
  return [dialog, ({ action, ...options }) => confirm({ ...options, onConfirm: action })];
}
