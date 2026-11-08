/**
 * Whether containerd accepts `name` as a namespace: letters and digits,
 * separated by single dots, dashes or underscores, up to 76 characters.
 */
export function namespaceNameValid(name: string): boolean {
  const n = name.trim();
  return n.length > 0 && n.length <= 76 && /^[A-Za-z0-9]+(?:[._-][A-Za-z0-9]+)*$/.test(n);
}
