// @ts-check
/**
 * A Sätteri hast plugin that prefixes root-relative links in Markdown and MDX
 * with the site's base path, so docs can link to `/docs/installation/` and work
 * whether the site is served from `/Homeport/` (a GitHub Pages project site) or
 * from the root of a custom domain. Covers plain links and <LinkCard href>.
 *
 * In a translated page (`locale`, such as `fr`), links to the docs and to the
 * pages that exist in every language (home, download, changelog) also get the
 * language's prefix, so a French page links to French pages: translators keep
 * writing `/docs/hosts/`. Legal pages stay English.
 *
 * @param {string} base  Astro's `base`, e.g. `/Homeport`.
 * @param {string} [locale]  The page's language prefix; empty for English.
 * @returns {import('satteri').HastPluginDefinition}
 */
/** Paths that exist in every language: the docs, home, download and changelog. */
const LOCALIZED = /^\/(docs\/|download\/|changelog\/|$|#)/;

/**
 * The plugin for the page being compiled: its language comes from its path,
 * `src/content/docs/<code>/…`, when `<code>` is one of `codes`.
 *
 * @param {string} base
 * @param {readonly string[]} codes  The prefixed languages' codes.
 * @returns {(ctx: { fileURL: URL | undefined }) => import('satteri').HastPluginDefinition}
 */
export function localizedBaseLinks(base, codes) {
  return ({ fileURL }) => {
    const match = /\/content\/docs\/([^/]+)\//.exec(fileURL?.pathname ?? '');
    const code = match && codes.includes(match[1] ?? '') ? (match[1] ?? '') : '';
    return baseLinks(base, code);
  };
}

export default function baseLinks(base, locale = '') {
  const prefix = base.replace(/\/$/, '');

  /** @param {string} href */
  const localize = (href) =>
    locale && LOCALIZED.test(href) && !href.startsWith(`/${locale}/`) ? `/${locale}${href}` : href;

  /** @param {unknown} href */
  const rewrite = (href) => {
    if (typeof href !== 'string' || !href.startsWith('/') || href.startsWith('//'))
      return undefined;
    if (prefix && href.startsWith(`${prefix}/`)) return undefined;
    const out = `${prefix}${localize(href)}`;
    return out === href ? undefined : out;
  };

  return {
    name: 'homeport-base-links',
    element: {
      filter: ['a'],
      visit(node, ctx) {
        const href = rewrite(node.properties?.href);
        if (href) ctx.setProperty(node, 'href', href);
      },
    },
    mdxJsxFlowElement: {
      filter: ['LinkCard'],
      visit(node) {
        let changed = false;
        const attributes = node.attributes.map((attribute) => {
          if (attribute.type !== 'mdxJsxAttribute' || attribute.name !== 'href') return attribute;
          const href = rewrite(attribute.value);
          if (!href) return attribute;
          changed = true;
          return { ...attribute, value: href };
        });
        return changed ? { ...node, attributes } : undefined;
      },
    },
  };
}
