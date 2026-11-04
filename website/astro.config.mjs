// @ts-check
import { satteri } from '@astrojs/markdown-satteri';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import starlightLinksValidator from 'starlight-links-validator';
import baseLinks from './src/app/markdown/base-links.mjs';
import { sidebar } from './src/app/config/sidebar.ts';
import { REPO_URL } from './src/core/site/index.ts';

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

export default defineConfig({
  site,
  base,
  trailingSlash: 'always',
  markdown: { processor: satteri({ hastPlugins: [baseLinks(base)] }) },
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
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      editLink: { baseUrl: `${REPO_URL}/edit/main/website/` },
      lastUpdated: true,
      credits: false,
      customCss: [
        '@fontsource-variable/geist',
        '@fontsource-variable/geist-mono',
        './src/app/styles/theme.css',
        './src/app/styles/docs.css',
      ],
      components: {
        Header: './src/app/starlight/Header.astro',
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
        // The marketing pages aren't Starlight pages, so the validator can't check links to them.
        starlightLinksValidator({
          exclude: [`${root}/`, `${root}/download/**`, `${root}/changelog/**`],
        }),
      ],
    }),
    sitemap(),
  ],
});
