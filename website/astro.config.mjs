// @ts-check
import { satteri } from '@astrojs/markdown-satteri';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import starlightLinksValidator from 'starlight-links-validator';
import baseLinks from './src/lib/base-links.mjs';
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
        './src/styles/theme.css',
        './src/styles/docs.css',
      ],
      components: {
        Header: './src/components/starlight/Header.astro',
      },
      head: [
        { tag: 'meta', attrs: { property: 'og:image', content: `${site}${root}/og.png` } },
        { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
        { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
        { tag: 'meta', attrs: { name: 'twitter:card', content: 'summary_large_image' } },
        { tag: 'link', attrs: { rel: 'manifest', href: `${root}/site.webmanifest` } },
        { tag: 'link', attrs: { rel: 'apple-touch-icon', href: `${root}/apple-touch-icon.png` } },
      ],
      sidebar: [
        {
          label: 'Getting started',
          items: [
            { label: 'Introduction', slug: 'docs' },
            { label: 'Installation', slug: 'docs/installation' },
            { label: 'Quick start', slug: 'docs/quick-start' },
            { label: 'System requirements', slug: 'docs/requirements' },
          ],
        },
        {
          label: 'Hosts',
          items: [
            { label: 'Hosts and connections', slug: 'docs/hosts' },
            { label: 'Local engines', slug: 'docs/hosts/local' },
            { label: 'SSH', slug: 'docs/hosts/ssh' },
            { label: 'TLS and TCP', slug: 'docs/hosts/tls-tcp' },
          ],
        },
        {
          label: 'Using Homeport',
          items: [
            { label: 'Overview and events', slug: 'docs/guides/overview' },
            { label: 'Containers', slug: 'docs/guides/containers' },
            { label: 'Images', slug: 'docs/guides/images' },
            { label: 'Volumes and networks', slug: 'docs/guides/volumes-networks' },
            { label: 'Stacks', slug: 'docs/guides/stacks' },
            { label: 'Settings', slug: 'docs/guides/settings' },
          ],
        },
        {
          label: 'Reference',
          items: [
            { label: 'Security and your data', slug: 'docs/reference/security' },
            { label: 'Verifying downloads', slug: 'docs/reference/verifying' },
            { label: 'Updating and uninstalling', slug: 'docs/reference/updating' },
            { label: 'Troubleshooting', slug: 'docs/reference/troubleshooting' },
            { label: 'FAQ', slug: 'docs/reference/faq' },
            { label: 'Glossary', slug: 'docs/reference/glossary' },
          ],
        },
        {
          label: 'Project',
          items: [
            { label: 'Building from source', slug: 'docs/project/building' },
            { label: 'Contributing', slug: 'docs/project/contributing' },
            { label: 'Changelog', link: '/changelog/' },
          ],
        },
      ],
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
