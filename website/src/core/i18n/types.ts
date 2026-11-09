import type en from './strings/en';

/** A key of the marketing pages' strings, such as `home.hero.title`. */
export type StringKey = keyof typeof en;

/** A language's strings: every key English has. */
export type Strings = { [K in StringKey]: string };

/** One area file of a language (`fr/home.ts`), typed from the English file it translates. */
export type Translation<English> = { [K in keyof English]: string };
