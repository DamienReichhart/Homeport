import type { ReactNode } from 'react';
import { createIntl } from 'react-intl';
import { en, errorMessages } from './en';

/** The arguments a message names, given a sample value each. */
type Sample = string | number | ((chunks: ReactNode[]) => ReactNode);

function samples(message: string): Record<string, Sample> {
  const values: Record<string, Sample> = {};
  for (const [, name, kind] of message.matchAll(/\{\s*(\w+)\s*(?:,\s*(\w+))?/g)) {
    if (name) values[name] = kind === 'plural' || kind === 'selectordinal' ? 2 : 'x';
  }
  for (const [, tag] of message.matchAll(/<(\w+)>/g)) if (tag) values[tag] = (c) => c;
  return values;
}

describe('the English catalogue', () => {
  it('holds valid ICU messages that format with their arguments', () => {
    const errors: string[] = [];
    const intl = createIntl({ locale: 'en', messages: en, onError: (e) => errors.push(e.message) });
    for (const [key, message] of Object.entries(en)) {
      const text = intl.formatMessage({ id: key }, samples(message));
      if (typeof text === 'string' && text.includes('{')) errors.push(`${key} left an argument`);
    }
    expect(errors).toEqual([]);
  });

  it('keeps error keys in the errors namespace', () => {
    expect(Object.keys(errorMessages).filter((k) => !k.startsWith('errors.'))).toEqual([]);
  });
});
