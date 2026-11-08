/** The picked rows still in `rows`: those deleted meanwhile drop out. */
export function stillListed<T>(picked: T[], rows: T[] | undefined, id: (row: T) => string): T[] {
  const ids = new Set((rows ?? []).map(id));
  return picked.filter((row) => ids.has(id(row)));
}
