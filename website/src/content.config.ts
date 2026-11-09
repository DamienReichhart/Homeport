import { defineCollection } from 'astro:content';
import { docsLoader, i18nLoader } from '@astrojs/starlight/loaders';
import { docsSchema, i18nSchema } from '@astrojs/starlight/schema';

export const collections = {
  docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }),
  // Where Starlight's own interface text (built in for every language) needs Homeport's words.
  i18n: defineCollection({ loader: i18nLoader(), schema: i18nSchema() }),
};
