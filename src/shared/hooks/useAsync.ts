import { useCallback, useEffect, useRef, useState } from 'react';

export type AsyncState<T> =
  | { status: 'loading'; data?: undefined; error?: undefined }
  | { status: 'success'; data: T; error?: undefined }
  | { status: 'error'; data?: undefined; error: Error };

/** Minimal async loader. Swap for TanStack Query when caching/invalidation is needed. */
export function useAsync<T>(fn: () => Promise<T>): AsyncState<T> & { reload: () => void } {
  const [state, setState] = useState<AsyncState<T>>({ status: 'loading' });
  const [tick, setTick] = useState(0);
  const fnRef = useRef(fn);

  useEffect(() => {
    fnRef.current = fn;
  });

  useEffect(() => {
    let cancelled = false;
    fnRef
      .current()
      .then((data) => !cancelled && setState({ status: 'success', data }))
      .catch(
        (error: unknown) =>
          !cancelled &&
          setState({
            status: 'error',
            error: error instanceof Error ? error : new Error(String(error)),
          }),
      );
    return () => {
      cancelled = true;
    };
  }, [tick]);

  const reload = useCallback(() => {
    setState({ status: 'loading' });
    setTick((t) => t + 1);
  }, []);

  return { ...state, reload };
}
