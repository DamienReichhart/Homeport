import { cn } from '@/shared/lib';

/** The classes of a sidebar link or button, current or not. */
export const navItem = (current: boolean) =>
  cn(
    'group flex h-8 items-center gap-2.5 rounded-md px-2.5 text-sm transition-colors outline-none focus-visible:ring-2 focus-visible:ring-sidebar-ring/50',
    current
      ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
      : 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground',
  );
