import { describe, expect, test } from 'bun:test';
import baseLinks from './base-links.mjs';

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
    expect(link('/Homeport', '/docs/installation/')).toBe('/Homeport/docs/installation/');
    expect(card('/Homeport/', '/docs/hosts/')).toBe('/Homeport/docs/hosts/');
  });

  test('leaves other links alone', () => {
    for (const href of [
      'https://example.com/',
      '//cdn.example.com/x',
      '#ssh',
      'relative/',
      '/Homeport/docs/',
    ]) {
      expect(link('/Homeport', href)).toBe(href);
    }
    expect(card('/Homeport', 'https://example.com/')).toBeUndefined();
  });

  test('does nothing on a custom domain', () => {
    expect(link('/', '/docs/')).toBe('/docs/');
  });
});
