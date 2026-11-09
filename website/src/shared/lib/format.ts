/** A file size in megabytes, in the language `lang` (a BCP 47 tag): `5.3 MB`, `5,3 Mo`. */
export function formatSize(bytes: number, lang: string): string {
  const mb = bytes / 1024 / 1024;
  const digits = mb >= 10 ? 0 : 1;
  return new Intl.NumberFormat(lang, {
    style: 'unit',
    unit: 'megabyte',
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(mb);
}

/** A long date in UTC, in the language `lang`: `3 November 2026`, `3 novembre 2026`. */
export function formatDate(iso: string, lang: string): string {
  if (!iso) return '';
  return new Intl.DateTimeFormat(lang, { dateStyle: 'long', timeZone: 'UTC' }).format(
    new Date(iso),
  );
}
