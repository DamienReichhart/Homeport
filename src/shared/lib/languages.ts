/**
 * The OS's preferred languages, as the webview reports them, keeping only
 * valid BCP 47 tags.
 *
 * On Linux, WebKitGTK reports a `C` or `POSIX` locale (minimal installs,
 * servers, `LANG=C.UTF-8`) as the language `"C"`, which isn't a tag: `Intl`
 * throws on it. uPlot builds a number format from `navigator.language` as
 * soon as it loads, so one bad tag left the whole window empty.
 */
export function validLanguages(tags: readonly string[]): string[] {
  return tags.filter((tag) => {
    try {
      return Intl.getCanonicalLocales(tag).length === 1;
    } catch {
      return false;
    }
  });
}

/** What `navigator` reports when the OS's languages hold no valid tag. */
export const FALLBACK_LANGUAGE = 'en-US';

/**
 * Makes `nav.language` and `nav.languages` report only valid tags, falling
 * back to English, so no library that reads them can throw. Leaves a
 * navigator that reports valid tags untouched.
 */
export function keepValidLanguages(nav: Navigator = navigator): void {
  const reported = nav.languages?.length ? [...nav.languages] : [nav.language];
  const valid = validLanguages(reported);
  if (valid.length === reported.length) return;
  const languages = Object.freeze(valid.length ? valid : [FALLBACK_LANGUAGE]);
  Object.defineProperty(nav, 'languages', { configurable: true, get: () => languages });
  Object.defineProperty(nav, 'language', { configurable: true, get: () => languages[0] });
}
