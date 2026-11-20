/** The containers table's optional columns, shown as its width allows. */
export interface Columns {
  image: boolean;
  ports: boolean;
  created: boolean;
  /** CPU and memory. */
  usage: boolean;
}

/**
 * Which optional columns fit a table `width` pixels wide, least useful first
 * to go: Created (the status says how long it's been up), then Ports, Image,
 * and CPU and memory. Name, status and the row's actions always stay. An
 * unknown width (0, before the first layout) shows them all.
 */
export function columnsFor(width: number): Columns {
  if (width <= 0) return { image: true, ports: true, created: true, usage: true };
  return {
    usage: width >= 576,
    image: width >= 704,
    ports: width >= 896,
    created: width >= 1088,
  };
}

/** How many columns a row has with `columns`: selection, name, status and actions, plus those shown. */
export const columnCount = (columns: Columns) =>
  4 +
  Number(columns.image) +
  Number(columns.ports) +
  Number(columns.created) +
  2 * Number(columns.usage);
