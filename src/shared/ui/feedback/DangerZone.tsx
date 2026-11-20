import type { ReactNode } from 'react';

/**
 * Where an action that can't be undone lives, set apart from the everyday
 * ones at the foot of a page: what it is, what it does, and its button.
 */
export function DangerZone({
  title,
  description,
  action,
}: {
  title: string;
  description: ReactNode;
  action: ReactNode;
}) {
  return (
    <section
      aria-label={title}
      className="border-destructive/40 flex flex-wrap items-center justify-between gap-3 rounded-md border p-4"
    >
      <div className="grid max-w-prose gap-0.5">
        <h2 className="text-sm font-semibold">{title}</h2>
        <p className="text-muted-foreground text-sm">{description}</p>
      </div>
      {action}
    </section>
  );
}
