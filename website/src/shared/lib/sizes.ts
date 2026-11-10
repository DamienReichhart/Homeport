/**
 * An `<img sizes>` list with every length multiplied by `factor`:
 * `(min-width: 64rem) 657px, 100vw` × 1.5 is
 * `(min-width: 64rem) calc((657px) * 1.5), calc((100vw) * 1.5)`.
 *
 * For an image drawn larger than its box (`object-fit: cover` on a crop), so the
 * browser picks a file wide enough for what is on screen.
 */
export function scaleSizes(sizes: string, factor: number): string {
  if (factor === 1) return sizes;
  return splitTopLevel(sizes)
    .map((entry) => {
      const at = lengthStart(entry);
      return `${entry.slice(0, at)}calc((${entry.slice(at)}) * ${Number(factor.toFixed(4))})`;
    })
    .join(', ');
}

/** The entries of a comma-separated list, leaving the commas inside brackets alone. */
function splitTopLevel(list: string): string[] {
  const entries: string[] = [];
  let depth = 0;
  let from = 0;
  for (let i = 0; i < list.length; i++) {
    if (list[i] === '(') depth++;
    else if (list[i] === ')') depth--;
    else if (list[i] === ',' && depth === 0) {
      entries.push(list.slice(from, i).trim());
      from = i + 1;
    }
  }
  entries.push(list.slice(from).trim());
  return entries.filter(Boolean);
}

/** Where an entry's length starts: after its media condition, when it has one. */
function lengthStart(entry: string): number {
  let depth = 0;
  let start = 0;
  for (let i = 0; i < entry.length; i++) {
    if (entry[i] === '(') depth++;
    else if (entry[i] === ')') depth--;
    else if (entry[i] === ' ' && depth === 0) start = i + 1;
  }
  return start;
}
