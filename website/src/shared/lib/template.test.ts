import { expect, test } from 'bun:test';
import { escapeHtml, fill, fillHtml } from './template';

test('fills placeholders and leaves unknown ones', () => {
  expect(fill('Homeport {version} is out', { version: '0.4.0' })).toBe('Homeport 0.4.0 is out');
  expect(fill('{count} Stacks', { count: 3 })).toBe('3 Stacks');
  expect(fill('Keep {this}', {})).toBe('Keep {this}');
});

test('escapes the template but not the HTML values', () => {
  expect(escapeHtml(`<a & "b">'`)).toBe('&lt;a &amp; &quot;b&quot;&gt;&#39;');
  expect(fillHtml('Tom & {code}', { code: '<code>git</code>' })).toBe('Tom &amp; <code>git</code>');
});
