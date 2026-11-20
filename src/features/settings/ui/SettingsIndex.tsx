import { useEffect, useState } from 'react';
import { cn } from '@/shared/lib/utils';
import { goToSection, sectionId, type SettingsSection } from './sections';
import { t } from '@/shared/i18n';

/** The section at the top of the page while it scrolls, for the index to mark. */
function useCurrentSection(sections: SettingsSection[], enabled: boolean): string | undefined {
  const [current, setCurrent] = useState(sections[0]?.id);
  useEffect(() => {
    if (!enabled || typeof IntersectionObserver === 'undefined') return;
    const visible = new Set<string>();
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          const id = entry.target.id.replace(/^settings-/, '');
          if (entry.isIntersecting) visible.add(id);
          else visible.delete(id);
        }
        const first = sections.find((s) => visible.has(s.id));
        if (first) setCurrent(first.id);
      },
      // A section counts once it reaches the top third of the window.
      { rootMargin: '0px 0px -66% 0px' },
    );
    for (const { id } of sections) {
      const section = document.getElementById(sectionId(id));
      if (section) observer.observe(section);
    }
    return () => observer.disconnect();
  }, [sections, enabled]);
  return current;
}

/**
 * Links to every section of the Settings page, so any setting is one click
 * away. `aside` is a list that stays in view beside the sections on a wide
 * window, marking the one being read; `row` is a wrapped row of links above
 * them on a narrower one.
 */
export function SettingsIndex({
  sections,
  variant,
}: {
  sections: SettingsSection[];
  variant: 'aside' | 'row';
}) {
  const aside = variant === 'aside';
  const current = useCurrentSection(sections, aside);
  return (
    <nav
      aria-label={t('settings.index.label')}
      className={aside ? 'sticky top-0 hidden w-44 shrink-0 self-start xl:block' : 'xl:hidden'}
    >
      <ul className={aside ? 'grid text-sm' : 'flex flex-wrap gap-x-4 gap-y-1 text-sm'}>
        {sections.map(({ id, label }) => (
          <li key={id}>
            <a
              href={`#${sectionId(id)}`}
              aria-current={aside && current === id ? 'location' : undefined}
              onClick={(e) => {
                e.preventDefault();
                goToSection(id);
              }}
              className={cn(
                'text-muted-foreground hover:text-foreground focus-visible:ring-ring/50 rounded-sm outline-none focus-visible:ring-[3px]',
                aside
                  ? 'aria-[current=location]:text-foreground aria-[current=location]:border-foreground block border-s-2 border-transparent py-1 ps-3 aria-[current=location]:font-medium'
                  : 'underline-offset-4 hover:underline',
              )}
            >
              {t(label)}
            </a>
          </li>
        ))}
      </ul>
    </nav>
  );
}
