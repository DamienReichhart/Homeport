/** One editable row of a label list. */
export interface LabelRow {
  key: string;
  value: string;
}

/** A label map as rows, sorted by key, for editing. */
export function labelRows(labels: Partial<Record<string, string>>): LabelRow[] {
  return Object.entries(labels)
    .map(([key, value]) => ({ key, value: value ?? '' }))
    .sort((a, b) => a.key.localeCompare(b.key));
}

/** The rows back as a map: blank rows dropped, keys trimmed, the last of a repeated key kept. */
export function labelMap(rows: LabelRow[]): Record<string, string> {
  const map: Record<string, string> = {};
  for (const row of rows) {
    const key = row.key.trim();
    if (key === '' && row.value.trim() === '') continue;
    map[key] = row.value;
  }
  return map;
}
