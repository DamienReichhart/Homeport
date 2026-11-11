/**
 * The checks that need a browser and the built site (`e2e/`):
 *
 *   bun run build && bun run test:images
 *
 * They run against `astro preview`, which serves `dist/` under the base path
 * the build used, so SITE_URL and BASE_PATH must be what the build had. The
 * browser is Playwright's Chromium (`bunx playwright install chromium`); set
 * PLAYWRIGHT_CHROMIUM_EXECUTABLE to use another Chromium or Chrome binary.
 */
import { defineConfig } from '@playwright/test';

const PORT = 4329;
/** The base path astro.config.mjs builds for, without a trailing slash. */
const base = (process.env.SITE_URL ? process.env.BASE_PATH || '/' : '/Homeport').replace(/\/$/, '');

export default defineConfig({
  testDir: 'e2e',
  testMatch: '**/*.e2e.ts',
  outputDir: 'node_modules/.playwright',
  fullyParallel: true,
  forbidOnly: Boolean(process.env.CI),
  reporter: process.env.CI ? [['github'], ['list']] : 'list',
  use: {
    baseURL: `http://127.0.0.1:${PORT}${base}/`,
    browserName: 'chromium',
    launchOptions: { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE || undefined },
  },
  webServer: {
    // --ignore-lock keeps the server in the foreground, where Playwright can stop
    // it: Astro moves `preview` to the background when it thinks an agent runs it.
    command: `bun run preview --host 127.0.0.1 --port ${PORT} --ignore-lock`,
    url: `http://127.0.0.1:${PORT}${base}/`,
    reuseExistingServer: !process.env.CI,
  },
});
