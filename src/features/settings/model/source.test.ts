import { sourceUrl } from './source';

it('points to the source of the running version', () => {
  expect(sourceUrl('0.12.0')).toBe('https://github.com/DamienReichhart/Homeport/tree/v0.12.0');
});
