import { useState } from 'react';
import { useLocation } from 'react-router-dom';

/** The navigation state that asks a page to start one of its actions. */
export interface ActionRequest {
  action: string;
}

/**
 * Calls `start` once each time the page is opened to start `action`, as
 * the command palette does (`ctx.open('/containers', 'run')`).
 */
export function useActionRequest(action: string, start: () => void) {
  const location = useLocation();
  const [handled, setHandled] = useState<string | null>(null);
  const state = location.state as Partial<ActionRequest> | null;
  // Adjusting state while rendering, once per navigation: no effect needed.
  if (state?.action === action && handled !== location.key) {
    setHandled(location.key);
    start();
  }
}
