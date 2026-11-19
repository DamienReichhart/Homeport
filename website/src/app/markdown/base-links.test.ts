import { describe, expect, test } from 'bun:test';
import baseLinks, { localizedBaseLinks } from './base-links.mjs';

/** The plugin's two visitors, as it defines them (the type also allows lists). */
interface Visitors {
  element: { visit(node: unknown, ctx: unknown): void };
  mdxJsxFlowElement: { visit(node: unknown): { attributes: { value: string }[] } | undefined };
}
const plugin = (base: string) => baseLinks(base) as unknown as Visitors;

/** Runs the plugin's link visitor on an <a href>, as Sätteri would. */
function link(base: string, href: string): string {
  const node = { properties: { href } };
  let result = href;
  plugin(base).element.visit(node, {
    setProperty: (_: unknown, _name: string, value: string) => (result = value),
  });
  return result;
}

/** Runs the plugin's MDX visitor on a <LinkCard href>. */
function card(base: string, href: string): string | undefined {
  const node = { attributes: [{ type: 'mdxJsxAttribute', name: 'href', value: href }] };
  return plugin(base).mdxJsxFlowElement.visit(node)?.attributes[0]!.value;
}

describe('baseLinks', () => {
  test('prefixes site paths with the base', () => {
    expect(link('/homeport', '/docs/installation/')).toBe('/homeport/docs/installation/');
    expect(card('/homeport/', '/docs/hosts/')).toBe('/homeport/docs/hosts/');
  });

  test('leaves other links alone', () => {
    for (const href of [
      'https://example.com/',
      '//cdn.example.com/x',
      '#ssh',
      'relative/',
      '/homeport/docs/',
    ]) {
      expect(link('/homeport', href)).toBe(href);
    }
    expect(card('/homeport', 'https://example.com/')).toBeUndefined();
  });

  test('does nothing on a custom domain', () => {
    expect(link('/', '/docs/')).toBe('/docs/');
  });
});

describe('baseLinks in a translated page', () => {
  const fr = (href: string) => {
    const node = { properties: { href } };
    let result = href;
    (baseLinks('/homeport', 'fr') as unknown as Visitors).element.visit(node, {
      setProperty: (_: unknown, _name: string, value: string) => (result = value),
    });
    return result;
  };

  test('keeps links to the docs and the translated pages in its language', () => {
    expect(fr('/docs/hosts/')).toBe('/homeport/fr/docs/hosts/');
    expect(fr('/docs/hosts/ssh/#bastions')).toBe('/homeport/fr/docs/hosts/ssh/#bastions');
    expect(fr('/download/')).toBe('/homeport/fr/download/');
    expect(fr('/')).toBe('/homeport/fr/');
    expect(fr('/fr/docs/')).toBe('/homeport/fr/docs/');
  });

  test('leaves the English-only pages and other links alone', () => {
    expect(fr('/privacy/')).toBe('/homeport/privacy/');
    expect(fr('https://example.com/docs/')).toBe('https://example.com/docs/');
  });

  test('finds the page’s language from its path', () => {
    const factory = localizedBaseLinks('', ['fr', 'pt-br']);
    const plugin = (path: string) =>
      factory({ fileURL: new URL(`file:///site/src/content/docs/${path}`) }) as unknown as Visitors;
    const visit = (p: Visitors) => {
      let out = '/docs/';
      p.element.visit(
        { properties: { href: '/docs/' } },
        { setProperty: (_: unknown, _n: string, v: string) => (out = v) },
      );
      return out;
    };
    expect(visit(plugin('pt-br/docs/index.mdx'))).toBe('/pt-br/docs/');
    expect(visit(plugin('docs/index.mdx'))).toBe('/docs/');
  });
});
