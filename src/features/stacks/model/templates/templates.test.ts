import { parse } from 'yaml';
import { t } from '@/shared/i18n';
import { TEMPLATES } from '.';

interface Service {
  image?: unknown;
  build?: unknown;
  volumes?: unknown[];
}

/** The keys a `.env` text defines. */
const envKeys = (env: string) =>
  env
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l && !l.startsWith('#'))
    .map((l) => l.split('=')[0]);

describe.each(TEMPLATES)('the $id template', (template) => {
  const doc = parse(template.compose) as { services?: Record<string, Service> | null };
  const services = Object.entries(doc.services ?? {});

  it('parses, with a services: section', () => {
    expect(doc).toHaveProperty('services');
  });

  it('pins every image to a tag other than latest, and builds nothing', () => {
    for (const [name, service] of services) {
      expect(service.build, name).toBeUndefined();
      expect(typeof service.image, name).toBe('string');
      const tag = /:([^/:@]+)$/.exec(service.image as string)?.[1];
      expect(tag, `${name} has a tag`).toBeDefined();
      expect(tag, name).not.toBe('latest');
    }
  });

  it('uses only named volumes or absolute paths', () => {
    for (const [name, service] of services) {
      for (const volume of service.volumes ?? []) {
        const source = typeof volume === 'string' ? (volume.split(':')[0] ?? '') : '';
        expect(source.startsWith('.') || source.startsWith('~'), `${name}: ${source}`).toBe(false);
      }
    }
  });

  it('defines every variable it references in its environment', () => {
    const keys = envKeys(template.env);
    for (const [, name] of template.compose.matchAll(/\$\{(\w+)/g))
      expect(keys, `${name} is in the environment`).toContain(name);
  });

  it('has a title and a description', () => {
    expect(t(template.title)).not.toBe(template.title);
    expect(t(template.description)).not.toBe(template.description);
  });
});

it('ends with the empty template, a services: skeleton', () => {
  expect(TEMPLATES.at(-1)?.id).toBe('empty');
  expect(TEMPLATES.at(-1)?.compose.trim()).toBe('services:');
});
