import type { LucideIcon } from 'lucide-react';
import type { ReactNode } from 'react';

export function EmptyState({
  icon: Icon,
  title,
  description,
  action,
}: {
  icon: LucideIcon;
  title: string;
  description?: ReactNode;
  action?: ReactNode;
}) {
  return (
    <div className="flex flex-col items-center justify-center gap-2 rounded-lg border border-dashed p-10 text-center">
      <Icon className="text-muted-foreground size-8" />
      <p className="font-medium">{title}</p>
      {description && <div className="text-muted-foreground max-w-md text-sm">{description}</div>}
      {action && <div className="mt-2">{action}</div>}
    </div>
  );
}
