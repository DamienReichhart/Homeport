/**
 * Whether an SPDX expression can be satisfied with accepted licenses. Handles
 * `AND`, `OR`, `WITH`, parentheses and the legacy `/` separator.
 */
export function isAccepted(expression: string, accepted: readonly string[]): boolean {
  const tokens = expression.replaceAll('/', ' OR ').match(/\(|\)|[^\s()]+/g) ?? [];
  let pos = 0;
  const peek = () => tokens[pos]?.toUpperCase();

  const primary = (): boolean => {
    if (tokens[pos] === '(') {
      pos++;
      const ok = or();
      pos++; // ')'
      return ok;
    }
    let id = tokens[pos++] ?? '';
    if (peek() === 'WITH') {
      pos++;
      id = `${id} WITH ${tokens[pos++] ?? ''}`;
    }
    return accepted.includes(id);
  };
  const and = (): boolean => {
    let ok = primary();
    while (peek() === 'AND') {
      pos++;
      ok = primary() && ok;
    }
    return ok;
  };
  const or = (): boolean => {
    let ok = and();
    while (peek() === 'OR') {
      pos++;
      ok = and() || ok;
    }
    return ok;
  };
  return tokens.length > 0 && or() && pos === tokens.length;
}
