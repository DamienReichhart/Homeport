/**
 * Captures the website's screenshots of the real Homeport UI.
 *
 *   bun run screenshots [name…]
 *
 * Builds `scripts/screenshots/` (the app's frontend with mocked IPC and demo
 * data), serves it, and photographs each shot with headless Firefox at
 * 1440×900 into `website/src/assets/screenshots/<name>.png`. Pass shot names
 * to capture only those. Set FIREFOX to use another Firefox binary.
 */
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import { build } from 'vite';
import { PROD, hexId } from './data';

const HERE = import.meta.dir;
const ROOT = join(HERE, '..', '..');
const OUT = join(ROOT, 'website', 'src', 'assets', 'screenshots');
const FIREFOX = process.env.FIREFOX ?? 'firefox';

interface Shot {
  name: string;
  route: string;
  theme?: 'light' | 'dark';
  steps?: string[];
  hold?: number;
  fastClock?: boolean;
}

const container = (name: string) => hexId(`${PROD}/${name}`);

const SHOTS: Shot[] = [
  { name: 'overview-light', route: '/' },
  { name: 'overview-dark', route: '/', theme: 'dark' },
  { name: 'containers-light', route: '/containers' },
  { name: 'containers-dark', route: '/containers', theme: 'dark' },
  { name: 'container-logs', route: `/containers/${container('shop-api-1')}/logs`, theme: 'dark' },
  { name: 'container-logs-light', route: `/containers/${container('shop-api-1')}/logs` },
  {
    name: 'container-terminal',
    route: `/containers/${container('shop-db-1')}/exec`,
    theme: 'dark',
  },
  {
    name: 'container-stats',
    route: `/containers/${container('shop-api-1')}/stats`,
    hold: 8500,
    fastClock: true,
  },
  { name: 'stacks', route: '/stacks' },
  { name: 'stack-editor', route: '/stacks/shop', steps: ['click:Definition'], hold: 3500 },
  {
    name: 'stack-editor-dark',
    route: '/stacks/shop',
    theme: 'dark',
    steps: ['click:Definition'],
    hold: 3500,
  },
  { name: 'stack-logs', route: '/stacks/shop', theme: 'dark', steps: ['click:Logs'], hold: 3500 },
  { name: 'images', route: '/images' },
  { name: 'volumes', route: '/volumes' },
  { name: 'networks', route: '/networks' },
  { name: 'events', route: '/events' },
  {
    name: 'add-host-ssh',
    route: '/hosts/new',
    steps: [
      'fill:production=backup-eu-2',
      'fill:server.example.com=backup-eu-2.example.com',
      'fill:deploy=deploy',
    ],
    hold: 3500,
  },
  {
    name: 'add-host-ssh-dark',
    route: '/hosts/new',
    theme: 'dark',
    steps: [
      'fill:production=backup-eu-2',
      'fill:server.example.com=backup-eu-2.example.com',
      'fill:deploy=deploy',
    ],
    hold: 3500,
  },
  { name: 'host-ssh', route: `/hosts/${PROD}` },
];

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
      for (const step of shot.steps ?? []) query.append('step', step);
      const url = `http://localhost:${server.port}/scripts/screenshots/index.html?${query}`;
      const png = join(work, `${shot.name}.png`);
      const proc = Bun.spawn(
        [
          FIREFOX,
          '--headless',
          '--no-remote',
          '-profile',
          profile,
          '--screenshot',
          png,
          '--window-size=1440,900',
          url,
        ],
        { stdout: 'ignore', stderr: 'ignore' },
      );
      const timer = setTimeout(() => proc.kill(), 90_000);
      await proc.exited;
      clearTimeout(timer);
      if (!existsSync(png)) throw new Error(`Firefox produced no screenshot for ${shot.name}`);
      copyFileSync(png, join(OUT, `${shot.name}.png`));
      console.log(`✓ ${shot.name}`);
    }
    await server.stop(true);
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

await main();
