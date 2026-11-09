import { openUrl } from '@tauri-apps/plugin-opener';
import { BookOpenIcon, ExternalLinkIcon } from 'lucide-react';
import type { ComponentType } from 'react';
import { NavLink } from 'react-router-dom';
import { ContainerdNamespacePicker, IncusProjectPicker } from '@/core/host';
import { HostSwitcher } from '@/features/hosts';
import { SwarmNav } from '@/features/swarm';
import { cn, DOCS_URL, REPO_URL } from '@/shared/lib';
import { GitHubIcon, Logo, toastError, WithShortcut } from '@/shared/ui';
import { useNavEntries } from '../shortcuts/navShortcuts';
import { direction, t } from '@/shared/i18n';

export function Sidebar() {
  const { entries, shortcuts } = useNavEntries();
  return (
    <nav
      aria-label={t('app.nav.main')}
      className="bg-sidebar text-sidebar-foreground border-sidebar-border flex w-56 shrink-0 flex-col border-e p-3"
    >
      <Logo className="mb-3 px-1.5 pt-1" />
      <div className="mb-3 grid gap-1.5">
        <HostSwitcher />
        <ContainerdNamespacePicker />
        <IncusProjectPicker />
      </div>
      {/* The pages scroll when the window is short; the links below stay put.
          The padding leaves room for the focus ring inside the scroll box. */}
      <div className="-m-1 flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto p-1">
        {entries.map(({ path, label, icon: Icon }) => {
          const link = (
            <NavLink
              key={path}
              to={path}
              end={path === '/'}
              className={({ isActive }) =>
                cn(
                  'group flex items-center gap-2.5 rounded-md px-3 py-1.5 text-sm transition-colors',
                  isActive
                    ? 'bg-sidebar-accent text-sidebar-accent-foreground font-medium'
                    : 'text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground',
                )
              }
            >
              <Icon className="text-muted-foreground group-aria-[current=page]:text-sidebar-primary size-4" />
              {t(label)}
            </NavLink>
          );
          const keys = shortcuts.find((s) => s.entry.path === path)?.keys;
          return keys ? (
            <WithShortcut
              key={path}
              keys={keys}
              label={t(label)}
              side={direction() === 'rtl' ? 'left' : 'right'}
            >
              {link}
            </WithShortcut>
          ) : (
            link
          );
        })}
        <SwarmNav />
      </div>
      <div className="border-sidebar-border mt-3 grid shrink-0 gap-0.5 border-t pt-3">
        <ExternalLink url={DOCS_URL} label={t('app.nav.docs')} icon={BookOpenIcon} />
        <ExternalLink url={REPO_URL} label={t('app.nav.github')} icon={GitHubIcon} />
      </div>
    </nav>
  );
}

/**
 * A link out of the app, styled as a page link but never current: it opens in
 * the system browser, because following it here would replace the app.
 */
function ExternalLink({
  url,
  label,
  icon: Icon,
}: {
  url: string;
  label: string;
  icon: ComponentType<{ className?: string }>;
}) {
  return (
    <a
      href={url}
      aria-label={t('app.nav.opensInBrowser', { label })}
      className="text-sidebar-foreground/80 hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground flex items-center gap-2.5 rounded-md px-3 py-1.5 text-sm transition-colors"
      onClick={(event) => {
        event.preventDefault();
        openUrl(url).catch(toastError);
      }}
      // A middle click would open the page in a new webview window.
      onAuxClick={(event) => event.preventDefault()}
    >
      <Icon className="text-muted-foreground size-4" />
      {label}
      <ExternalLinkIcon className="text-muted-foreground ms-auto size-3.5" aria-hidden />
    </a>
  );
}
