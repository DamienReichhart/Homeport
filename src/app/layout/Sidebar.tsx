import { openUrl } from '@tauri-apps/plugin-opener';
import {
  BookOpenIcon,
  ExternalLinkIcon,
  LayoutGridIcon,
  SearchIcon,
  SettingsIcon,
  type LucideIcon,
} from 'lucide-react';
import { Fragment, type ComponentType } from 'react';
import { NavLink, useMatch } from 'react-router-dom';
import type { NavEntry } from '@/core/feature';
import { cn, DOCS_URL, REPO_URL } from '@/shared/lib';
import { GitHubIcon, Keys, Logo, toastError, WithShortcut } from '@/shared/ui';
import { usePalette } from '../palette/paletteContext';
import { useNavEntries } from '../shortcuts/navShortcuts';
import { FleetNav } from './FleetNav';
import { navItem } from './navItem';
import { direction, t } from '@/shared/i18n';

/**
 * The sidebar: search, the Board, then the fleet (every Host, Swarm and
 * Cluster with its status, the selected one opened onto its pages), and the
 * app's own links at the foot.
 */
export function Sidebar() {
  const { entries, shortcuts } = useNavEntries();
  const keysOf = (entry: NavEntry) => shortcuts.find((s) => s.entry.path === entry.path)?.keys;
  const board = entries.find((e) => e.path === '/');
  const pages = entries.filter((e) => e.scope !== 'app' || e.path === '/overview');
  const settings = entries.find((e) => e.path === '/settings');
  return (
    <nav
      aria-label={t('app.nav.main')}
      className="bg-sidebar text-sidebar-foreground border-sidebar-border flex w-60 shrink-0 flex-col border-e"
    >
      <div className="grid gap-3 p-3 pb-2">
        <Logo className="px-1 pt-1" />
        <PaletteButton />
      </div>
      {/* The fleet scrolls when the window is short; search and the foot stay put.
          The padding leaves room for the focus ring inside the scroll box. */}
      <div className="flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto px-3 pb-3">
        {board && <PageLink entry={board} icon={LayoutGridIcon} keys={keysOf(board)} end />}
        <FleetNav
          pages={pages.map((entry, i) => (
            <Fragment key={entry.path}>
              {i > 0 && entry.group !== pages[i - 1]?.group && (
                <div role="separator" className="border-sidebar-border mx-2.5 my-1 border-t" />
              )}
              <PageLink entry={entry} keys={keysOf(entry)} nested />
            </Fragment>
          ))}
        />
      </div>
      <div className="border-sidebar-border grid shrink-0 gap-0.5 border-t p-3">
        {settings && <PageLink entry={settings} icon={SettingsIcon} keys={keysOf(settings)} />}
        <ExternalLink url={DOCS_URL} label={t('app.nav.docs')} icon={BookOpenIcon} />
        <ExternalLink url={REPO_URL} label={t('app.nav.github')} icon={GitHubIcon} />
      </div>
    </nav>
  );
}

/** Opens the command palette; it says how to open it from the keyboard too. */
function PaletteButton() {
  const { setOpen } = usePalette();
  return (
    <button
      type="button"
      onClick={() => setOpen(true)}
      className="border-sidebar-border bg-background text-muted-foreground hover:text-foreground focus-visible:ring-sidebar-ring/50 flex h-8 min-w-0 items-center gap-2 rounded-md border ps-2.5 pe-1.5 text-sm transition-colors outline-none focus-visible:ring-2"
    >
      <SearchIcon className="size-4 shrink-0" />
      <span className="min-w-0 flex-1 truncate text-start">{t('app.nav.search')}</span>
      <Keys keys="Mod+K" className="shrink-0" />
    </button>
  );
}

function PageLink({
  entry,
  icon,
  keys,
  end,
  nested,
}: {
  entry: NavEntry;
  icon?: LucideIcon;
  keys?: string;
  end?: boolean;
  nested?: boolean;
}) {
  const Icon = icon ?? entry.icon;
  // A class string, not NavLink's function: the tooltip's trigger merges classes as strings.
  const current = useMatch({ path: entry.path, end: end ?? false }) !== null;
  const link = (
    <NavLink to={entry.path} end={end} className={cn(navItem(current), nested && 'h-7 ps-3')}>
      <Icon className="text-muted-foreground group-aria-[current=page]:text-sidebar-primary size-4 shrink-0" />
      <span className="truncate">{t(entry.label)}</span>
    </NavLink>
  );
  return keys ? (
    <WithShortcut
      keys={keys}
      label={t(entry.label)}
      side={direction() === 'rtl' ? 'left' : 'right'}
    >
      {link}
    </WithShortcut>
  ) : (
    link
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
      className={navItem(false)}
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
