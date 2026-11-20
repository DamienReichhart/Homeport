import { useState, type ReactNode } from 'react';
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '../primitives/alert-dialog';
import { Button } from '../primitives/button';
import { TypeToConfirm, typedMatches } from './TypeToConfirm';
import { cn } from '@/shared/lib/utils';
import { t } from '@/shared/i18n';

export interface ConfirmOptions {
  title: string;
  /** What will happen, naming exactly what is lost for destructive actions. */
  description: ReactNode;
  confirmLabel: string;
  /** Red emphasis for actions that destroy data. */
  destructive?: boolean;
  /**
   * For actions that lose data for good (a volume, an Instance's disk): the
   * name the user types before the button works, so it's never a reflex.
   */
  typeToConfirm?: string;
  /**
   * Runs the action. If it rejects, the dialog stays open so the user can try
   * again; reporting the failure is the action's job (a mutation toasts it).
   */
  onConfirm: () => Promise<unknown> | void;
}

/**
 * A confirmation step for actions that can't be undone. Render it once and
 * open it with the setter returned by {@link useConfirm}.
 */
export function ConfirmDialog({
  options,
  onClose,
}: {
  options: ConfirmOptions | null;
  onClose: () => void;
}) {
  const [busy, setBusy] = useState(false);
  const [typed, setTyped] = useState('');
  const expected = options?.typeToConfirm;
  const close = () => {
    setTyped('');
    onClose();
  };
  return (
    <AlertDialog open={options !== null} onOpenChange={(open) => !open && !busy && close()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle className={cn(options?.destructive && 'text-destructive')}>
            {options?.title}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div>{options?.description}</div>
          </AlertDialogDescription>
        </AlertDialogHeader>
        {expected && <TypeToConfirm name={expected} value={typed} onChange={setTyped} />}
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy}>{t('common.cancel')}</AlertDialogCancel>
          <Button
            variant={options?.destructive ? 'destructive' : 'default'}
            className={cn(
              options?.destructive && 'bg-destructive text-white hover:bg-destructive/90',
            )}
            disabled={busy || (!!expected && !typedMatches(typed, expected))}
            onClick={async () => {
              if (!options) return;
              setBusy(true);
              try {
                await options.onConfirm();
                close();
              } catch {
                // Already reported by the action; stay open for a retry.
              } finally {
                setBusy(false);
              }
            }}
          >
            {options?.confirmLabel}
          </Button>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}

/** `const [dialog, confirm] = useConfirm()`: render `dialog`, call `confirm({...})`. */
export function useConfirm(): [ReactNode, (options: ConfirmOptions) => void] {
  const [options, setOptions] = useState<ConfirmOptions | null>(null);
  return [
    <ConfirmDialog key="confirm" options={options} onClose={() => setOptions(null)} />,
    setOptions,
  ];
}
