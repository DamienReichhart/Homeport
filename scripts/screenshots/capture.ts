/**
 * Captures the website's screenshots of the real Homeport UI.
 *
 *   bun run screenshots [--locale <tag>…] [name…]
 *
 * Builds `scripts/screenshots/` (the app's frontend with mocked IPC and demo
 * data), serves it, and photographs each shot with headless Firefox in a
 * 1440×900 window at twice the pixel density (`SCALE`), so each PNG is
 * 2880×1800 and stays sharp on HiDPI screens. They go into
 * `website/src/assets/screenshots/<name>.png` (English) and
 * `website/src/assets/screenshots/<tag>/<name>.png` for every other language,
 * the tag in lower case (`fr`, `pt-br`). Every language with an app catalogue
 * is captured, so each page of the website shows the app in its own language;
 * `--locale` (one per flag) captures only those, for a quick retake. Pass shot
 * names to capture only those. `website/src/features/docs/screenshots.test.ts`
 * fails when a language lacks a shot English has.
 *
 * Each PNG is then compressed losslessly with oxipng when it's installed
 * (`cargo install oxipng`, or OXIPNG for a binary that isn't on PATH); without
 * it the shots are left as the browser wrote them, with a notice.
 *
 * Set FIREFOX to use another Firefox binary, or CHROMIUM to a Chrome or
 * Chromium binary to use that instead.
 */
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import { build } from 'vite';
import { SHOTS } from './shots';

const HERE = import.meta.dir;
const ROOT = join(HERE, '..', '..');
const OUT = join(ROOT, 'website', 'src', 'assets', 'screenshots');
const FIREFOX = process.env.FIREFOX ?? 'firefox';
const CHROMIUM = process.env.CHROMIUM;

/** The window the app is laid out in, in CSS pixels. */
const WIDTH = 1440;
const HEIGHT = 900;
/** Device pixels per CSS pixel: the PNGs are WIDTH × SCALE by HEIGHT × SCALE. */
const SCALE = 2;

/**
 * The command that photographs `url` into `png`, laid out at WIDTH×HEIGHT and
 * drawn at SCALE, with reduced motion. Firefox takes the scale from its profile.
 */
function screenshotCommand(profile: string, png: string, url: string): string[] {
  if (CHROMIUM) {
    return [
      CHROMIUM,
      '--headless',
      '--no-sandbox',
      '--hide-scrollbars',
      '--force-prefers-reduced-motion',
      `--screenshot=${png}`,
      `--window-size=${WIDTH},${HEIGHT}`,
      `--force-device-scale-factor=${SCALE}`,
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
    `--window-size=${WIDTH},${HEIGHT}`,
    url,
  ];
}

/** Snap Firefox can only reach non-hidden paths in the home directory. */
function workDir(): string {
  const snap = join(homedir(), 'snap', 'firefox', 'common');
  const base = existsSync(snap) ? snap : tmpdir();
  return mkdtempSync(join(base, 'homeport-screenshots-'));
}

/** The size of a PNG, from its header. */
function pngSize(path: string): { width: number; height: number } {
  const header = readFileSync(path).subarray(0, 24);
  return { width: header.readUInt32BE(16), height: header.readUInt32BE(20) };
}

/** oxipng's path: OXIPNG, or the one on PATH; undefined when there is none. */
const OXIPNG = process.env.OXIPNG ?? Bun.which('oxipng') ?? undefined;

/** Compresses the PNGs in place without changing a pixel. */
async function optimise(pngs: string[]): Promise<void> {
  if (pngs.length === 0) return;
  if (!OXIPNG) {
    console.warn(
      'oxipng not found: the screenshots are not compressed. Install it (`cargo install oxipng`) and capture again before committing them.',
    );
    return;
  }
  const size = () => pngs.reduce((sum, png) => sum + statSync(png).size, 0);
  const before = size();
  const proc = Bun.spawn([OXIPNG, '-o', '4', '--strip', 'safe', '--quiet', ...pngs], {
    stdout: 'ignore',
    stderr: 'inherit',
  });
  if ((await proc.exited) !== 0) throw new Error('oxipng failed');
  const mb = (bytes: number) => `${(bytes / 1e6).toFixed(1)} MB`;
  console.log(`✓ oxipng: ${pngs.length} screenshots, ${mb(before)} → ${mb(size())}`);
}

const GIF = Buffer.from('R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7', 'base64');

/** The app's languages: every catalogue folder with a meta.ts. */
const CATALOGUES = readdirSync(join(ROOT, 'src', 'shared', 'i18n')).filter((d) =>
  existsSync(join(ROOT, 'src', 'shared', 'i18n', d, 'meta.ts')),
);

/** Where a language's shots go: English at the top, others in their folder. */
const outDir = (tag: string) => (tag === 'en' ? OUT : join(OUT, tag.toLowerCase()));

/** The languages and shot names asked for on the command line. */
function parseArgs(args: string[]): { locales: string[]; names: string[] } {
  const asked: string[] = [];
  const names: string[] = [];
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--locale') asked.push(args[++i] ?? '');
    else names.push(args[i] ?? '');
  }
  const locales = asked.length === 0 || asked.includes('all') ? CATALOGUES : asked;
  for (const tag of locales)
    if (!CATALOGUES.includes(tag)) throw new Error(`No catalogue for ${tag} in src/shared/i18n`);
  return { locales, names };
}

async function main() {
  const { locales, names: only } = parseArgs(process.argv.slice(2));
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
        `user_pref("layout.css.devPixelsPerPx", "${SCALE}");`,
      ].join('\n'),
    );
    const written: string[] = [];
    for (const locale of locales) {
      const out = outDir(locale);
      mkdirSync(out, { recursive: true });
      for (const shot of shots) {
        const query = new URLSearchParams({
          route: shot.route,
          theme: shot.theme ?? 'light',
          hold: String(shot.hold ?? 2500),
          lang: locale,
        });
        if (shot.fastClock) query.set('clock', 'fast');
        if (shot.demo) query.set('demo', shot.demo);
        for (const step of shot.steps ?? []) query.append('step', step);
        const url = `http://127.0.0.1:${server.port}/scripts/screenshots/index.html?${query}`;
        const png = join(work, `${locale}-${shot.name}.png`);
        const proc = Bun.spawn(screenshotCommand(profile, png, url), {
          stdout: 'ignore',
          stderr: process.env.DEBUG_SHOTS ? 'inherit' : 'ignore',
        });
        const timer = setTimeout(() => proc.kill(), 90_000);
        await proc.exited;
        clearTimeout(timer);
        if (!existsSync(png))
          throw new Error(`The browser produced no screenshot for ${shot.name}`);
        const { width, height } = pngSize(png);
        if (width !== WIDTH * SCALE || height !== HEIGHT * SCALE)
          throw new Error(
            `${shot.name} is ${width}×${height}, not ${WIDTH * SCALE}×${HEIGHT * SCALE}: the browser ignored the window size or the scale`,
          );
        const target = join(out, `${shot.name}.png`);
        copyFileSync(png, target);
        written.push(target);
        console.log(`✓ ${locale} ${shot.name}`);
      }
    }
    await server.stop(true);
    await optimise(written);
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

await main();
