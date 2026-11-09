/**
 * Captures the website's screenshots of the real Homeport UI.
 *
 *   bun run screenshots [name…]
 *
 * Builds `scripts/screenshots/` (the app's frontend with mocked IPC and demo
 * data), serves it, and photographs each shot with headless Firefox at
 * 1440×900 into `website/src/assets/screenshots/<name>.png`. Pass shot names
 * to capture only those. Set FIREFOX to use another Firefox binary, or CHROMIUM
 * to a Chrome or Chromium binary to use that instead.
 */
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import { build } from 'vite';
import { SHOTS } from './shots';

const HERE = import.meta.dir;
const ROOT = join(HERE, '..', '..');
const OUT = join(ROOT, 'website', 'src', 'assets', 'screenshots');
const FIREFOX = process.env.FIREFOX ?? 'firefox';
const CHROMIUM = process.env.CHROMIUM;

/** The command that photographs `url` into `png` at 1440×900, with reduced motion. */
function screenshotCommand(profile: string, png: string, url: string): string[] {
  if (CHROMIUM) {
    return [
      CHROMIUM,
      '--headless',
      '--no-sandbox',
      '--hide-scrollbars',
      '--force-prefers-reduced-motion',
      `--screenshot=${png}`,
      '--window-size=1440,900',
      url,
    ];
  }
  return [
    FIREFOX,
    '--headless',
    '--no-remote',
    '-profile',
    profile,
    '--screenshot',
    png,
    '--window-size=1440,900',
    url,
  ];
}

/** Snap Firefox can only reach non-hidden paths in the home directory. */
function workDir(): string {
  const snap = join(homedir(), 'snap', 'firefox', 'common');
  const base = existsSync(snap) ? snap : tmpdir();
  return mkdtempSync(join(base, 'homeport-screenshots-'));
}

const GIF = Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7', 'base64');

async function main() {
  const only = process.argv.slice(2);
  const shots = only.length ? SHOTS.filter((s) => only.includes(s.name)) : SHOTS;
  if (shots.length === 0) throw new Error(`No shot named ${only.join(', ')}`);

  const work = workDir();
  const site = join(work, 'site');
  try {
    await build({
      configFile: join(ROOT, 'vite.config.ts'),
      root: ROOT,
      logLevel: 'warn',
      build: {
        outDir: site,
        emptyOutDir: true,
        rollupOptions: { input: join(HERE, 'index.html') },
      },
    });

    const server = Bun.serve({
      port: 0,
      async fetch(request) {
        const url = new URL(request.url);
        if (url.pathname === '/__hold') {
          await Bun.sleep(Number(url.searchParams.get('ms') ?? 0));
          return new Response(GIF, { headers: { 'content-type': 'image/gif' } });
        }
        const file = Bun.file(join(site, decodeURIComponent(url.pathname)));
        return (await file.exists())
          ? new Response(file)
          : new Response('Not found', { status: 404 });
      },
    });

    const profile = join(work, 'profile');
    mkdirSync(profile);
    writeFileSync(
      join(profile, 'user.js'),
      [
        'user_pref("browser.shell.checkDefaultBrowser", false);',
        'user_pref("datareporting.policy.dataSubmissionEnabled", false);',
        'user_pref("toolkit.telemetry.reportingpolicy.firstRun", false);',
        'user_pref("ui.prefersReducedMotion", 1);',
      ].join('\n'),
    );
    mkdirSync(OUT, { recursive: true });

    for (const shot of shots) {
      const query = new URLSearchParams({
        route: shot.route,
        theme: shot.theme ?? 'light',
        hold: String(shot.hold ?? 2500),
      });
      if (shot.fastClock) query.set('clock', 'fast');
      if (shot.demo) query.set('demo', shot.demo);
      for (const step of shot.steps ?? []) query.append('step', step);
      const url = `http://127.0.0.1:${server.port}/scripts/screenshots/index.html?${query}`;
      const png = join(work, `${shot.name}.png`);
      const proc = Bun.spawn(screenshotCommand(profile, png, url), {
        stdout: 'ignore',
        stderr: process.env.DEBUG_SHOTS ? 'inherit' : 'ignore',
      });
      const timer = setTimeout(() => proc.kill(), 90_000);
      await proc.exited;
      clearTimeout(timer);
      if (!existsSync(png)) throw new Error(`The browser produced no screenshot for ${shot.name}`);
      copyFileSync(png, join(OUT, `${shot.name}.png`));
      console.log(`✓ ${shot.name}`);
    }
    await server.stop(true);
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

await main();
