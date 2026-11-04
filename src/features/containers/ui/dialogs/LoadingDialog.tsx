import { useQuery, type QueryKey, type UseQueryOptions } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * Loads what a dialog needs first, in a dialog of its own: a spinner, or the
 * failure with Retry, then `children` with the data.
 */
export function LoadingDialog<T, K extends QueryKey>({
  title,
  query,
  onClose,
  children,
}: {
  title: string;
  query: UseQueryOptions<T, Error, T, K>;
  onClose: () => void;
  children: (data: T) => ReactNode;
}) {
  const result = useQuery(query);
  if (result.data !== undefined) return children(result.data);
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        {result.error ? (
          <ErrorMessage error={result.error} onRetry={() => void result.refetch()} />
        ) : (
          <Spinner />
        )}
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
