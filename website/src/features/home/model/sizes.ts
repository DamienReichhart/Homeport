/**
 * How wide the home page draws its screenshots, for their `sizes`. They follow
 * Questions.astro's grid and `.container`: the container is 76rem at most, less
 * its gutters (2 × 2rem from 48rem, 2 × 1.25rem under it); the stops sit 3.5rem
 * in from the line; from 64rem a stop's screenshot takes 8 of 12 columns after
 * a 4rem gap.
 */
export const STOP_SIZES = [
  '(min-width: 76rem) 688px',
  '(min-width: 64rem) calc((100vw - 11.5rem) * 8 / 12)',
  '(min-width: 48rem) calc(100vw - 7.5rem)',
  'calc(100vw - 6rem)',
].join(', ');
