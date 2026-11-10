/**
 * How wide a Showcase's media column is, for its screenshot's `sizes`. It
 * follows Showcase.astro's grid and `.container`: beside the text from 64rem,
 * 7 of 11 columns of the container (72rem at most) less its padding (2 × 2rem)
 * and the gap (3.5rem); below that the container's full width, whose padding
 * is 2rem from 48rem and 1.25rem under it.
 */
export const SHOWCASE_SIZES = [
  '(min-width: 72rem) 657px',
  '(min-width: 64rem) calc((100vw - 7.5rem) * 7 / 11)',
  '(min-width: 48rem) calc(100vw - 4rem)',
  'calc(100vw - 2.5rem)',
].join(', ');
