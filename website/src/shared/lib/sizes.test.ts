import { expect, test } from 'bun:test';
import { scaleSizes } from './sizes';

test('leaves the list alone at a factor of one', () => {
  expect(scaleSizes('(min-width: 64rem) 657px, 100vw', 1)).toBe('(min-width: 64rem) 657px, 100vw');
});

test('multiplies every length, keeping its media condition', () => {
  expect(scaleSizes('(min-width: 64rem) 657px, 100vw', 1.5)).toBe(
    '(min-width: 64rem) calc((657px) * 1.5), calc((100vw) * 1.5)',
  );
});

test('multiplies a calc() length whole, commas and spaces included', () => {
  expect(
    scaleSizes('(min-width: 64rem) calc((100vw - 7.5rem) * 7 / 11), min(100vw, 40rem)', 2),
  ).toBe(
    '(min-width: 64rem) calc((calc((100vw - 7.5rem) * 7 / 11)) * 2), calc((min(100vw, 40rem)) * 2)',
  );
});

test('keeps a condition joined with and', () => {
  expect(scaleSizes('(min-width: 48rem) and (max-width: 64rem) 50vw', 2)).toBe(
    '(min-width: 48rem) and (max-width: 64rem) calc((50vw) * 2)',
  );
});

test('rounds the factor to four decimals', () => {
  expect(scaleSizes('100vw', 1.3666666)).toBe('calc((100vw) * 1.3667)');
});
