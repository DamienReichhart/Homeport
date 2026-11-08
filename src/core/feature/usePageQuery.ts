import { useState } from 'react';
import { useLocation } from 'react-router-dom';

/**
 * A list page's search text, starting from the `?q=` it was opened with,
 * as global search opens an image, volume or network. Opening the page
 * again with another `?q=` takes that one.
 */
export function usePageQuery(): [string, (query: string) => void] {
  const location = useLocation();
  const asked = () => new URLSearchParams(location.search).get('q') ?? '';
  const [state, setState] = useState(() => ({ key: location.key, query: asked() }));
  // Adjusting state while rendering, once per navigation: no effect needed.
  if (state.key !== location.key) setState({ key: location.key, query: asked() });
  const query = state.key === location.key ? state.query : asked();
  return [query, (next) => setState({ key: location.key, query: next })];
}
