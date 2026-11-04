import { afterEach, expect, test } from 'bun:test';
import { href } from './href';

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
});
