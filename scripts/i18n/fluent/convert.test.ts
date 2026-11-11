import { describe, expect, it } from 'vitest';
import { fluentFile, fluentId, fluentPattern } from './convert';

describe('fluentId', () => {
  it('replaces the dots, which Fluent ids cannot hold', () => {
    expect(fluentId('containers.empty.title')).toBe('containers-empty-title');
  });

  it('refuses a key that would collide or not parse', () => {
    expect(() => fluentId('a-b.c')).toThrow();
    expect(() => fluentId('1a')).toThrow();
  });
});

describe('fluentPattern', () => {
  it('turns arguments into variables', () => {
    expect(fluentPattern('{running} running of {count}')).toBe(
      '{ $running } running of { $count }',
    );
  });

  it('turns a plural into a select, exact numbers first, # as the count', () => {
    expect(fluentPattern('{count, plural, one {# host} other {# hosts} =0 {Import}}')).toBe(
      [
        '{ $count ->',
        '        [0] Import',
        '        [one] { NUM($count) } host',
        '       *[other] { NUM($count) } hosts',
        '    }',
      ].join('\n'),
    );
  });

  it('keeps text around a select, and nests', () => {
    const pattern = fluentPattern(
      'A {kind, select, x {{n, plural, one {#} other {#s}}} other {}} B',
    );
    expect(pattern).toContain('A { $kind ->');
    expect(pattern).toContain('[x] { $n ->');
    expect(pattern).toContain('*[other] {""}');
    expect(pattern.endsWith('} B')).toBe(true);
  });

  it('keeps tags, quoted braces, edge spaces and line breaks', () => {
    expect(fluentPattern('Use <mono>{name}</mono>')).toBe('Use <mono>{ $name }</mono>');
    expect(fluentPattern("'{'x'}'")).toBe('{"{"}x{"}"}');
    expect(fluentPattern(' a ')).toBe('{" "}a{" "}');
    expect(fluentPattern('a\nb')).toBe('a{"\\u000A"}b');
  });

  it('refuses what it does not convert', () => {
    expect(() => fluentPattern('{n, number}')).toThrow();
    expect(() => fluentPattern('{n, selectordinal, other {#}}')).toThrow();
  });
});

describe('fluentFile', () => {
  it('names the message that fails', () => {
    expect(() => fluentFile('x', { 'a.b': '{n, date}' })).toThrow(/^a\.b:/);
  });

  it('writes a header and one message per key', () => {
    expect(fluentFile('Header', { 'a.b': 'Text' })).toBe('### Header\n\na-b = Text\n');
  });
});
