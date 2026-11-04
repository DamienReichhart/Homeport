import { useQuery } from '@tanstack/react-query';
import { FolderOpenIcon } from 'lucide-react';
import { useEffect, useState, useSyncExternalStore } from 'react';
import { appQueries, commands, errorReport, toAppError, type AppError } from '@/shared/api';
import { Button } from '../primitives/button';
import { CopyButton } from '../data/CopyButton';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../primitives/dialog';
import { HintView } from './HintView';

let shown: AppError | null = null;
const listeners = new Set<() => void>();

function setShown(error: AppError | null) {
  shown = error;
  listeners.forEach((l) => l());
}

/** Opens the Details dialog for `error`, from anywhere (toasts, alerts, status badges). */
export function showErrorDetails(error: AppError) {
  setShown(error);
}

/** Renders the Details dialog opened by {@link showErrorDetails}. Mount once, near the root. */
export function ErrorDetailsHost() {
  const error = useSyncExternalStore(
    (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
    () => shown,
  );
  // Close when the host goes away, so a remounted app starts without a stale dialog.
  useEffect(() => () => setShown(null), []);
  return error && <ErrorDetailsDialog error={error} onClose={() => setShown(null)} />;
}

function ErrorDetailsDialog({ error, onClose }: { error: AppError; onClose: () => void }) {
  const app = useQuery(appQueries.info());
  const [openLogError, setOpenLogError] = useState<AppError | null>(null);
  const facts: [string, string][] = [
    ['Code', error.code],
    ...(error.context.command ? [['Command', error.context.command] as [string, string]] : []),
    ['When', error.context.at.toLocaleString()],
    ...(app.data
      ? [
          ['App', `${app.data.name} ${app.data.version} · ${app.data.os} ${app.data.arch}`] as [
            string,
            string,
          ],
        ]
      : []),
  ];
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Error details</DialogTitle>
          <DialogDescription className="text-foreground">{error.message}</DialogDescription>
        </DialogHeader>
        <div className="flex min-w-0 flex-col gap-4">
          {error.hint && <HintView hint={error.hint} />}
          <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm">
            {facts.map(([label, value]) => (
              <div key={label} className="contents">
                <dt className="text-muted-foreground">{label}</dt>
                <dd className="font-mono text-xs leading-5 break-all">{value}</dd>
              </div>
            ))}
          </dl>
          {error.detail && (
            <div className="flex flex-col gap-1">
              <p className="text-muted-foreground text-sm">Raw detail</p>
              <pre className="bg-muted max-h-64 overflow-auto rounded-md p-3 font-mono text-xs whitespace-pre-wrap">
                {error.detail}
              </pre>
            </div>
          )}
          {openLogError && <p className="text-destructive text-sm">{openLogError.message}</p>}
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            onClick={() =>
              commands
                .openLogFolder()
                .then(() => setOpenLogError(null))
                .catch((e: unknown) => setOpenLogError(toAppError(e)))
            }
          >
            <FolderOpenIcon />
            Open log folder
          </Button>
          <CopyButton text={errorReport(error, app.data)} label="Copy report" />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
