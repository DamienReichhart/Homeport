import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { EditorView } from '@codemirror/view';
import { tags as t } from '@lezer/highlight';
import type { Extension } from '@codemirror/state';
import type { ResolvedTheme } from '@/shared/lib/theme';

// Syntax colours (docs/BRAND.md): keys in the text colour, so the structure reads
// as plain type; values in two quiet hues; comments in Fragment Mono's italic.
// Red, amber and green are kept for status and never mean syntax.
const SYNTAX = {
  light: {
    key: '#141414',
    string: '#1f5a96',
    number: '#8a3a8f',
    atom: '#8a3a8f',
    comment: '#6e6e68',
    punctuation: '#6e6e68',
  },
  dark: {
    key: '#f7f7f4',
    string: '#9dc0ee',
    number: '#e0aee6',
    atom: '#e0aee6',
    comment: '#8c8c86',
    punctuation: '#8c8c86',
  },
};

function build(theme: ResolvedTheme): Extension {
  const c = SYNTAX[theme];
  return [
    EditorView.theme(
      {
        '&': { backgroundColor: 'var(--card)', color: 'var(--card-foreground)' },
        '.cm-scroller': { fontFamily: 'var(--font-mono)', fontSize: '0.75rem', lineHeight: '1.6' },
        '.cm-content': { caretColor: 'var(--foreground)' },
        '.cm-cursor, .cm-dropCursor': {
          borderLeftColor: 'var(--foreground)',
          borderLeftWidth: '2px',
        },
        '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
          backgroundColor: 'color-mix(in oklch, var(--primary) 22%, transparent) !important',
        },
        '.cm-gutters': {
          backgroundColor: 'var(--muted)',
          color: 'var(--muted-foreground)',
          borderRight: '1px solid var(--border)',
        },
        '.cm-activeLine': {
          backgroundColor: 'color-mix(in oklch, var(--primary) 6%, transparent)',
        },
        '.cm-activeLineGutter': {
          backgroundColor: 'color-mix(in oklch, var(--primary) 12%, transparent)',
          color: 'var(--foreground)',
        },
        '.cm-matchingBracket, &.cm-focused .cm-matchingBracket': {
          backgroundColor: 'color-mix(in oklch, var(--foreground) 16%, transparent)',
        },
        '.cm-foldPlaceholder': {
          backgroundColor: 'var(--muted)',
          border: '1px solid var(--border)',
          color: 'var(--muted-foreground)',
        },
        '.cm-tooltip': {
          backgroundColor: 'var(--popover)',
          color: 'var(--popover-foreground)',
          border: '1px solid var(--border)',
        },
      },
      { dark: theme === 'dark' },
    ),
    syntaxHighlighting(
      HighlightStyle.define([
        { tag: [t.propertyName, t.definition(t.propertyName)], color: c.key },
        { tag: [t.string, t.special(t.string), t.content], color: c.string },
        { tag: [t.number, t.integer, t.float], color: c.number },
        { tag: [t.bool, t.null, t.atom, t.keyword], color: c.atom },
        { tag: [t.typeName, t.labelName, t.meta], color: c.atom },
        { tag: t.comment, color: c.comment, fontStyle: 'italic' },
        { tag: [t.punctuation, t.separator, t.squareBracket, t.brace], color: c.punctuation },
      ]),
    ),
  ];
}

const THEMES: Record<ResolvedTheme, Extension> = { light: build('light'), dark: build('dark') };

/** A CodeMirror theme in Homeport's colours, for the resolved app theme. */
export const editorTheme = (theme: ResolvedTheme): Extension => THEMES[theme];
