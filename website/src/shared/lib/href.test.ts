import { afterEach, expect, test } from 'bun:test';
import { href, unbased } from './href';

const base = import.meta.env.BASE_URL;

afterEach(() => {
  import.meta.env.BASE_URL = base;
});

test('prefixes the base path of a project site', () => {
  import.meta.env.BASE_URL = '/Homeport/';
  expect(href('/docs/')).toBe('/Homeport/docs/');
  expect(href('download/')).toBe('/Homeport/download/');
});

test('leaves paths alone on a custom domain', () => {
  import.meta.env.BASE_URL = '/';
  expect(href('/docs/')).toBe('/docs/');
  expect(unbased('/docs/')).toBe('/docs/');
});

test('takes the base path off again', () => {
  import.meta.env.BASE_URL = '/Homeport/';
  expect(unbased('/Homeport/fr/download/')).toBe('/fr/download/');
  expect(unbased('/Homeport/')).toBe('/');
  expect(unbased('/Homeport')).toBe('/');
  expect(unbased('/Homeportal/')).toBe('/Homeportal/');
});
