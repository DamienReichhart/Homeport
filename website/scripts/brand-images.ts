/**
 * Generates the site's raster brand images into public/:
 *
 *   og.png                 1200×630 social card (rendered by headless Firefox, so it
 *                          uses Host Grotesk and Fragment Mono): the headline and a
 *                          departure board of Hosts, as on the home page
 *   apple-touch-icon.png   180×180, and icon-192.png / icon-512.png for the manifest
 *
 *   bun scripts/brand-images.ts        (from website/)
 *
 * Run it again after changing the mark or the tagline. Set FIREFOX to use a
 * Firefox other than the one on PATH, or CHROMIUM to use Chrome or Chromium.
 */
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { homedir, tmpdir } from 'node:os';
import { join } from 'node:path';
import sharp from 'sharp';

const SITE = join(import.meta.dir, '..');
const REPO = join(SITE, '..');
const PUBLIC = join(SITE, 'public');
const MARK = join(REPO, 'assets/brand/mark.svg');
const FIREFOX = process.env.FIREFOX ?? 'firefox';
const CHROMIUM = process.env.CHROMIUM;

async function icons() {
  for (const [file, size] of [
    ['apple-touch-icon.png', 180],
    ['icon-192.png', 192],
    ['icon-512.png', 512],
  ] as const) {
    await sharp(MARK, { density: 384 }).resize(size, size).png().toFile(join(PUBLIC, file));
    console.log(`✓ ${file}`);
  }
  copyFileSync(MARK, join(PUBLIC, 'favicon.svg'));
  console.log('✓ favicon.svg');
}

const BOARD = [
  ['laptop', 'socket', 'All running', false],
  ['homelab', 'ssh', 'All running', false],
  ['vps-fra-1', 'tls', '1 Stack needs you', true],
  ['client-prod', 'ssh', 'All running', false],
] as const;

const CARD = /* html */ `<!doctype html>
<html>
<head>
<meta charset="utf-8" />
<style>
  @font-face { font-family: 'Host Grotesk'; src: url(host-grotesk.woff2) format('woff2'); font-weight: 300 800; }
  @font-face { font-family: 'Fragment Mono'; src: url(fragment-mono.woff2) format('woff2'); }
  * { margin: 0; box-sizing: border-box; }
  html, body { width: 1200px; height: 630px; overflow: hidden; }
  body { position: relative; font-family: 'Host Grotesk', sans-serif; color: #141414; background: #f7f7f4; }
  .lockup { position: absolute; left: 72px; top: 72px; height: 34px; }
  h1 { position: absolute; left: 72px; top: 168px; width: 520px; font-size: 68px; font-weight: 700; line-height: 1; letter-spacing: -0.035em; }
  p { position: absolute; left: 72px; top: 410px; width: 480px; font-size: 24px; line-height: 1.35; color: #4a4a46; }
  .meta { position: absolute; left: 72px; bottom: 64px; font-family: 'Fragment Mono', monospace; font-size: 17px; color: #5c5c57; letter-spacing: 0.02em; }
  table { position: absolute; left: 640px; top: 150px; width: 488px; border-collapse: collapse; background: #141414; color: #f7f7f4; border-radius: 6px; overflow: hidden; font-size: 22px; }
  th, td { padding: 22px 24px; border-bottom: 1px solid rgb(247 247 244 / 0.12); text-align: left; white-space: nowrap; }
  tr:last-child td { border-bottom: 0; }
  th { padding-block: 16px; font-family: 'Fragment Mono', monospace; font-weight: 400; font-size: 13px; letter-spacing: 0.08em; text-transform: uppercase; color: #a6a6a0; }
  td.host { font-weight: 600; }
  td.via { font-family: 'Fragment Mono', monospace; font-size: 18px; color: #a6a6a0; }
  td.status span { display: inline-flex; align-items: center; gap: 12px; }
  td.status i { width: 11px; height: 11px; border-radius: 50%; background: currentColor; }
  tr.signal td.status { color: #ff5c5c; }
</style>
</head>
<body>
  <img class="lockup" src="lockup-light.svg" alt="" />
  <h1>Every Docker Engine, on one board.</h1>
  <p>Your local and remote Engines over SSH, TLS or TCP. No agent on the server.</p>
  <div class="meta">Free · Open source · Windows · macOS · Linux</div>
  <table>
    <tr><th>Host</th><th>Via</th><th>Status</th></tr>
    ${BOARD.map(
      ([host, via, status, signal]) =>
        `<tr class="${signal ? 'signal' : ''}"><td class="host">${host}</td><td class="via">${via}</td><td class="status"><span><i></i>${status}</span></td></tr>`,
    ).join('')}
  </table>
</body>
</html>`;

/** Snap Firefox can only read non-hidden folders in the home directory. */
function workDir(): string {
  const snap = join(homedir(), 'snap', 'firefox', 'common');
  return mkdtempSync(join(existsSync(snap) ? snap : tmpdir(), 'homeport-og-'));
}

async function socialCard() {
  const work = workDir();
  try {
    const fonts = join(SITE, 'node_modules/@fontsource');
    copyFileSync(
      join(fonts + '-variable', 'host-grotesk/files/host-grotesk-latin-wght-normal.woff2'),
      join(work, 'host-grotesk.woff2'),
    );
    copyFileSync(
      join(fonts, 'fragment-mono/files/fragment-mono-latin-400-normal.woff2'),
      join(work, 'fragment-mono.woff2'),
    );
    copyFileSync(join(REPO, 'assets/brand/lockup-light.svg'), join(work, 'lockup-light.svg'));
    writeFileSync(join(work, 'card.html'), CARD);
    const profile = join(work, 'profile');
    mkdirSync(profile);

    const out = join(work, 'og.png');
    const page = `file://${join(work, 'card.html')}`;
    const command = CHROMIUM
      ? [
          CHROMIUM,
          '--headless',
          '--no-sandbox',
          '--hide-scrollbars',
          `--screenshot=${out}`,
          '--window-size=1200,630',
          page,
        ]
      : [
          FIREFOX,
          '--headless',
          '--no-remote',
          '-profile',
          profile,
          '--screenshot',
          out,
          '--window-size=1200,630',
          page,
        ];
    const proc = Bun.spawn(command, { stdout: 'ignore', stderr: 'ignore' });
    await proc.exited;
    if (!existsSync(out)) throw new Error('The browser produced no screenshot');
    // Recompress: social cards are fetched by every link preview.
    await sharp(out).png({ compressionLevel: 9, palette: false }).toFile(join(PUBLIC, 'og.png'));
    console.log('✓ og.png');
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

mkdirSync(PUBLIC, { recursive: true });
await icons();
await socialCard();
