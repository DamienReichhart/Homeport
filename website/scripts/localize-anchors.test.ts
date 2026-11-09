import { describe, expect, test } from 'bun:test';
import { anchorMap, headingIds, rewrite } from './localize-anchors';

describe('localize-anchors', () => {
  test('reads a page’s heading ids in order, without Starlight’s', () => {
    const html =
      '<h2 id="starlight__on-this-page">x</h2><h2 id="how-it-works">a</h2><h3 id="bastions">b</h3>';
    expect(headingIds(html)).toEqual(['how-it-works', 'bastions']);
  });

  test('maps by position, and not at all when the headings differ', () => {
    expect(anchorMap(['a', 'b'], ['x', 'y']).get('b')).toBe('y');
    expect(anchorMap(['a', 'b'], ['x']).size).toBe(0);
  });

  test('rewrites links to other pages and to the same page', () => {
    const maps = new Map([
      ['docs/hosts/ssh', new Map([['bastions', 'bastiones']])],
      ['docs/faq', new Map([['top', 'arriba']])],
    ]);
    const text =
      'See [x](/docs/hosts/ssh/#bastions), <a href="/docs/hosts/ssh/#bastions">y</a>, [z](#top) and [w](/docs/other/#keep).';
    expect(rewrite(text, 'docs/faq', maps)).toBe(
      'See [x](/docs/hosts/ssh/#bastiones), <a href="/docs/hosts/ssh/#bastiones">y</a>, [z](#arriba) and [w](/docs/other/#keep).',
    );
  });
});
