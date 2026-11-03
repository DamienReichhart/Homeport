import { useState, type ReactNode } from 'react';
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from './alert-dialog';
import { Button } from './button';
import { cn } from '@/shared/lib/utils';

export interface ConfirmOptions {
  title: string;
  /** What will happen, naming exactly what is lost for destructive actions. */
  description: ReactNode;
  confirmLabel: string;
  /** Red emphasis for actions that destroy data. */
  destructive?: boolean;
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
  return (
    <AlertDialog open={options !== null} onOpenChange={(open) => !open && !busy && onClose()}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle className={cn(options?.destructive && 'text-destructive')}>
            {options?.title}
          </AlertDialogTitle>
          <AlertDialogDescription asChild>
            <div>{options?.description}</div>
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={busy}>Cancel</AlertDialogCancel>
          <Button
            variant={options?.destructive ? 'destructive' : 'default'}
            className={cn(
              options?.destructive && 'bg-destructive text-white hover:bg-destructive/90',
            )}
            disabled={busy}
            onClick={async () => {
              if (!options) return;
              setBusy(true);
              try {
                await options.onConfirm();
                onClose();
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
