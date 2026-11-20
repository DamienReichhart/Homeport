import type { ReactNode } from 'react';
import { useSplitPane } from './SplitView';

/**
 * A page's title, a one-line description and its actions: at most one solid
 * button among them (DESIGN.md). In a split view's narrow list pane it shows
 * the title only, since the open item's page carries the actions.
 */
export function PageHeader({
  title,
  description,
  actions,
}: {
  title: ReactNode;
  description?: ReactNode;
  actions?: ReactNode;
}) {
  const { compact } = useSplitPane();
  if (compact) {
    return (
      <header className="mb-3">
        <h1 className="truncate text-base font-semibold">{title}</h1>
      </header>
    );
  }
  return (
    <header className="mb-5 flex flex-wrap items-start justify-between gap-x-6 gap-y-3">
      <div className="min-w-0 flex-1">
        <h1 className="truncate text-xl font-semibold tracking-[-0.01em]">{title}</h1>
        {description && (
          <div className="text-muted-foreground mt-1 max-w-prose text-sm">{description}</div>
        )}
      </div>
      {actions && <div className="flex flex-wrap items-center gap-2">{actions}</div>}
    </header>
  );
}
