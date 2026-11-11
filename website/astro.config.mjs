// @ts-check
import { satteri } from '@astrojs/markdown-satteri';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';
import { defineConfig, sharpImageService } from 'astro/config';
import starlightLinksValidator from 'starlight-links-validator';
import { localizedBaseLinks } from './src/app/markdown/base-links.mjs';
import { sidebar } from './src/app/config/sidebar.ts';
import { REPO_URL } from './src/core/site/index.ts';
import { LOCALES, PREFIXED_LOCALES } from './src/core/site/locales.ts';

/*
 * The site is served from GitHub Pages. The Pages workflow passes the origin and
 * base path that `actions/configure-pages` reports, so a custom domain (empty
 * base path) works without editing this file. Elsewhere — local builds and pull
 * requests — it builds for the project site.
 */
const site = process.env.SITE_URL || 'https://damienreichhart.github.io';
const base = process.env.SITE_URL ? process.env.BASE_PATH || '/' : '/Homeport';
/** The base without a trailing slash, for building paths: `/Homeport` or ``. */
const root = base.replace(/\/$/, '');
const codes = PREFIXED_LOCALES.map((l) => l.code);

/** Starlight's locales: English at the root, every other language under its prefix. */
const locales = Object.fromEntries(
  LOCALES.map((l) => [l.code || 'root', { label: l.label, lang: l.lang, dir: l.dir }]),
);

export default defineConfig({
  site,
  base,
  trailingSlash: 'always',
  markdown: { processor: satteri({ hastPlugins: [localizedBaseLinks(base, codes)] }) },
  // Every screenshot becomes AVIF at five widths, which is most of a build without
  // the image cache. At effort 3, not sharp's 4, the encoder takes about a third of
  // the time for files up to 5% larger. Their quality is set in shared/ui/Screenshot.astro.
  image: { service: sharpImageService({ avif: { effort: 3 } }) },
  // The brand assets live at the repo root (assets/brand); let Vite read them.
  vite: { server: { fs: { allow: ['..'] } } },
  integrations: [
    starlight({
      title: 'Homeport',
      description:
        'A desktop app for managing Docker Engines anywhere: this machine’s, or remote ones over SSH, TLS or TCP.',
      logo: {
        light: '../assets/brand/lockup-light.svg',
        dark: '../assets/brand/lockup-dark.svg',
        replacesTitle: true,
      },
      favicon: '/favicon.svg',
      // English URLs don't change; untranslated pages show English with a notice.
      defaultLocale: 'root',
      locales,
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      editLink: { baseUrl: `${REPO_URL}/edit/main/website/` },
      lastUpdated: true,
      credits: false,
      customCss: [
        '@fontsource-variable/host-grotesk',
        '@fontsource/fragment-mono/400.css',
        '@fontsource/fragment-mono/400-italic.css',
        './src/app/styles/theme.css',
        './src/app/styles/docs.css',
      ],
      components: {
        Header: './src/app/starlight/Header.astro',
        Footer: './src/app/starlight/Footer.astro',
      },
      head: [
        { tag: 'meta', attrs: { property: 'og:image', content: `${site}${root}/og.png` } },
        { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
        { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
        { tag: 'meta', attrs: { name: 'twitter:card', content: 'summary_large_image' } },
        { tag: 'link', attrs: { rel: 'manifest', href: `${root}/site.webmanifest` } },
        { tag: 'link', attrs: { rel: 'apple-touch-icon', href: `${root}/apple-touch-icon.png` } },
      ],
      sidebar,
      plugins: [
        // The marketing pages aren't Starlight pages, so the validator can't check links to
        // them; src/app/i18n's route test checks that each exists in every language.
        starlightLinksValidator({
          exclude: [
            ...['', ...codes.map((c) => `/${c}`)].flatMap((prefix) => [
              `${root}${prefix}/`,
              `${root}${prefix}/download/**`,
              `${root}${prefix}/changelog/**`,
            ]),
            `${root}/legal-notice/**`,
            `${root}/privacy/**`,
            `${root}/terms/**`,
            `${root}/cookies/**`,
          ],
        }),
      ],
    }),
    // Each page with its other languages as xhtml:link alternates.
    sitemap({
      i18n: {
        defaultLocale: 'en',
        locales: Object.fromEntries(LOCALES.map((l) => [l.code || 'en', l.lang])),
      },
    }),
  ],
});
