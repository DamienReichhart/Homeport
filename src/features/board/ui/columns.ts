/**
 * The optional columns' classes: the table keeps to the page's width, so
 * they go as it narrows, least useful first: Ports, Stack, Image, then CPU,
 * memory and the status (the name's dot keeps the state).
 */
export const SHOWN_FROM = {
  ports: 'hidden @[72rem]:table-cell',
  stack: 'hidden @[66rem]:table-cell',
  image: 'hidden @[60rem]:table-cell',
  basic: 'hidden @[36rem]:table-cell',
} as const;
