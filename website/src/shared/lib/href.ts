/** Prefixes a site path with the configured base, e.g. `/docs/` → `/Homeport/docs/`. */
export function href(path: string): string {
  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  return `${base}${path.startsWith('/') ? path : `/${path}`}`;
}
