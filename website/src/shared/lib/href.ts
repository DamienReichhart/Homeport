/** Prefixes a site path with the configured base, e.g. `/docs/` → `/Homeport/docs/`. */
export function href(path: string): string {
  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  return `${base}${path.startsWith('/') ? path : `/${path}`}`;
}

/** The reverse of `href`: a site path without the configured base, e.g. `/Homeport/docs/` → `/docs/`. */
export function unbased(path: string): string {
  const base = import.meta.env.BASE_URL.replace(/\/$/, '');
  if (!base || (path !== base && !path.startsWith(`${base}/`))) return path;
  return path.slice(base.length) || '/';
}
