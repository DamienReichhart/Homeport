import { useState, type ReactNode } from 'react';
import { useShortcut, useShortcutList, type Shortcut } from '@/shared/hooks';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  Keys,
} from '@/shared/ui';
import { ShortcutsHelpContext } from './helpContext';
import { t } from '@/shared/i18n';

/** The help sheet listing every active shortcut, opened with `?`. */
export function ShortcutsHelpProvider({ children }: { children: ReactNode }) {
  const [open, setOpen] = useState(false);
  useShortcut({
    keys: '?',
    label: t('app.shortcuts.help'),
    group: t('app.shortcuts.general'),
    handler: () => setOpen(true),
  });
  return (
    <ShortcutsHelpContext.Provider value={() => setOpen(true)}>
      {children}
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="max-h-[80vh] overflow-y-auto sm:max-w-lg">
          <DialogHeader>
            <DialogTitle>{t('app.shortcuts.title')}</DialogTitle>
            <DialogDescription>{t('app.shortcuts.description')}</DialogDescription>
          </DialogHeader>
          <ShortcutGroups />
        </DialogContent>
      </Dialog>
    </ShortcutsHelpContext.Provider>
  );
}

function ShortcutGroups() {
  const shortcuts = useShortcutList();
  const groups = new Map<string, Shortcut[]>();
  for (const shortcut of shortcuts) {
    groups.set(shortcut.group, [...(groups.get(shortcut.group) ?? []), shortcut]);
  }
  return (
    <div className="grid gap-4">
      {[...groups].map(([group, list]) => (
        <section key={group} aria-label={group}>
          <h3 className="text-muted-foreground mb-1.5 text-xs font-medium tracking-wide uppercase">
            {group}
          </h3>
          <ul className="grid gap-1">
            {list.map((s) => (
              <li
                key={`${s.keys} ${s.label}`}
                className="flex items-center justify-between gap-4 text-sm"
              >
                <span>{s.label}</span>
                <Keys keys={s.keys} />
              </li>
            ))}
          </ul>
        </section>
      ))}
    </div>
  );
}
