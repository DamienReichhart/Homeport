import { useId } from 'react';
import type { StackTemplate } from '../../model/templates';
import { t } from '@/shared/i18n';

/**
 * The templates a new Managed Stack can start from, as cards. Each card is
 * a button named by its title and described by its one line, so the
 * gallery works with the keyboard and a screen reader.
 */
export function TemplateGallery({
  templates,
  onPick,
}: {
  templates: StackTemplate[];
  onPick: (template: StackTemplate) => void;
}) {
  const id = useId();
  return (
    <section className="grid gap-2" aria-labelledby={`${id}-title`}>
      <h2 id={`${id}-title`} className="text-sm font-medium">
        {t('stacks.templates.title')}
      </h2>
      <ul className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
        {templates.map((template) => (
          <li key={template.id} className="grid">
            <button
              type="button"
              aria-describedby={`${id}-${template.id}`}
              onClick={() => onPick(template)}
              className="hover:bg-muted focus-visible:ring-ring/50 grid content-start gap-1 rounded-lg border p-3 text-start outline-none focus-visible:ring-[3px]"
            >
              <span className="font-medium">{t(template.title)}</span>
              <span id={`${id}-${template.id}`} className="text-muted-foreground text-sm">
                {t(template.description)}
              </span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
