/** Values for a template's `{name}` placeholders. */
export type TemplateValues = Record<string, string | number>;

/** Fills `{name}` placeholders: `fill('Version {v}', { v: '1.0' })` → `Version 1.0`. Unknown names stay as written. */
export function fill(template: string, values: TemplateValues = {}): string {
  return template.replace(/\{(\w+)\}/g, (match, name: string) =>
    name in values ? String(values[name]) : match,
  );
}

/** Escapes text for HTML. */
export function escapeHtml(text: string): string {
  return text
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

/**
 * Fills a template for `set:html`: the template's own text is escaped, and each
 * value is HTML the caller built (a link, a `<code>`), so a template never
 * carries markup itself.
 */
export function fillHtml(template: string, html: Record<string, string>): string {
  return fill(escapeHtml(template), html);
}
