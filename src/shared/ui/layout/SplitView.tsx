import { createContext, useContext, useSyncExternalStore, type ReactNode } from 'react';
import { useOutlet, useParams } from 'react-router-dom';

/** From this window width, a list and the item opened from it share the screen. */
export const SPLIT_QUERY = '(min-width: 1280px)';

interface SplitPane {
  /** The list sits beside an open item, so it shows only what identifies a row. */
  compact: boolean;
  /** The key of the open item, as the route names it, to mark its row. */
  selected: string | null;
}

const SplitPaneContext = createContext<SplitPane>({ compact: false, selected: null });

/** Whether the list being drawn sits in a split view's narrow pane, and which row is open. */
export const useSplitPane = () => useContext(SplitPaneContext);

function subscribe(onChange: () => void) {
  const query = window.matchMedia?.(SPLIT_QUERY);
  query?.addEventListener('change', onChange);
  return () => query?.removeEventListener('change', onChange);
}

/** Whether the window is wide enough for a split view. */
export function useWideWindow(): boolean {
  return useSyncExternalStore(
    subscribe,
    () => window.matchMedia?.(SPLIT_QUERY).matches ?? false,
    () => false,
  );
}

/**
 * A list page whose items open beside it on a wide window: with no item open
 * the list fills the page; with one open, the list narrows to a pane on the
 * start side and the item takes the rest, so going through several is one
 * click (or one arrow key) each. On a narrow window the item takes the page,
 * as before. The URLs are the same either way.
 */
export function SplitView({
  list,
  listParams = [],
}: {
  list: ReactNode;
  /** The parameters of the list's own path, which aren't part of an item's key. */
  listParams?: string[];
}) {
  const outlet = useOutlet();
  const params = useParams();
  const wide = useWideWindow();
  // The open item's key: its route's parameters but the tab, joined like the
  // lists' row ids (`namespace/name`).
  const parts = Object.entries(params)
    .filter(
      ([name, value]) => value && name !== 'tab' && name !== '*' && !listParams.includes(name),
    )
    .map(([, value]) => value as string);
  const selected = parts.length ? parts.join('/') : null;
  if (!outlet || selected === null) {
    return (
      <SplitPaneContext.Provider value={{ compact: false, selected: null }}>
        {list}
      </SplitPaneContext.Provider>
    );
  }
  if (!wide) return <>{outlet}</>;
  return (
    <div className="-mx-6 -my-6 flex h-[calc(100%+3rem)] min-h-0">
      <aside className="bg-background w-[22rem] shrink-0 overflow-y-auto border-e py-6 ps-6 pe-4">
        <SplitPaneContext.Provider value={{ compact: true, selected }}>
          {list}
        </SplitPaneContext.Provider>
      </aside>
      <div className="min-w-0 flex-1 overflow-y-auto px-6 py-6">{outlet}</div>
    </div>
  );
}
