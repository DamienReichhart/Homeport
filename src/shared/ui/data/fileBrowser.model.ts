/** `/a/b/c` → `[['/', '/'], ['a', '/a'], ['b', '/a/b'], ['c', '/a/b/c']]`. */
export function crumbs(path: string): [string, string][] {
  const parts = path.split('/').filter(Boolean);
  return [
    ['/', '/'],
    ...parts.map((p, i): [string, string] => [p, `/${parts.slice(0, i + 1).join('/')}`]),
  ];
}
