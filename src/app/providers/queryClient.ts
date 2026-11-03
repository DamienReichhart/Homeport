import { MutationCache, QueryClient } from '@tanstack/react-query';
import { toastError } from '@/shared/ui';

export function createQueryClient() {
  return new QueryClient({
    // A failed mutation with no handler of its own still tells the user.
    // Mutations that show their error in place opt out with `meta: { silent: true }`.
    mutationCache: new MutationCache({
      onError: (error, _variables, _context, mutation) => {
        if (mutation.options.onError || mutation.meta?.silent) return;
        toastError(error);
      },
    }),
    defaultOptions: {
      // Engine data is refreshed by pushed events, so don't refetch on focus.
      queries: { refetchOnWindowFocus: false, retry: 1, staleTime: 10_000 },
    },
  });
}
