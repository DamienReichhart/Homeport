/**
 * A language's text for one namespace: the English file's keys, each with a
 * string. Written as `export default { … } satisfies Translation<typeof en>`,
 * so a missing or extra key fails the type check (ADR 0011).
 */
export type Translation<T> = { readonly [K in keyof T]: string };

/** What the Settings picker and the document need to know about a language. */
export interface LocaleMeta {
  /** The language's own name for itself: `Français`, `日本語`. */
  name: string;
  /** Its writing direction; `rtl` mirrors the layout. */
  dir: 'ltr' | 'rtl';
}
