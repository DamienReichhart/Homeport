/**
 * Generates the site's raster brand images into public/:
 *
 *   og.png                 1200×630 social card (rendered by headless Firefox, so it
 *                          uses Fira Sans): the headline and a small line map of
 *                          Hosts, as on the home page
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

/*
 * The card is the home page's line map, small: this machine at the terminus,
 * four lines out to their stations, and one red station where something needs you.
 */
const LINES = [
  {
    colour: '#0b7a4b',
    d: 'M640 300 H780 L880 200 H1128',
    stations: [
      [960, 200, 'prod-eu-1'],
      [1128, 200, 'nas'],
    ],
  },
  {
    colour: '#1e4bb8',
    d: 'M640 318 H800 L842 276 H1128',
    stations: [
      [960, 276, 'staging'],
      [1128, 276, 'vps-fra-1', true],
    ],
  },
  { colour: '#a8650a', d: 'M640 336 H1040', stations: [[1040, 336, 'lan-box']] },
  {
    colour: '#0f7a80',
    d: 'M640 354 H780 L850 424 H1128',
    stations: [
      [960, 424, 'k8s-prod'],
      [1128, 424, 'jobs'],
    ],
  },
] as const;

const MAP = LINES.map(
  (line) =>
    `<path d="${line.d}" fill="none" stroke="${line.colour}" stroke-width="7" stroke-linecap="round" stroke-linejoin="round"/>` +
    line.stations
      .map(([x, y, name, signal]) => {
        const above = y < 330;
        return (
          `<circle cx="${x}" cy="${y}" r="${signal ? 12 : 10}" fill="${signal ? '#c8102e' : '#fff'}" stroke="${signal ? '#c8102e' : '#16181b'}" stroke-width="4.5"/>` +
          `<text x="${x}" y="${above ? y - 22 : y + 38}" text-anchor="${x > 1100 ? 'end' : 'middle'}" class="${signal ? 'signal' : ''}">${name}</text>`
        );
      })
      .join(''),
).join('');

const CARD = /* html */ `<!doctype html>
<html>
<head>
<meta charset="utf-8" />
<style>
  @font-face { font-family: 'Fira Sans'; src: url(fira-sans-400.woff2) format('woff2'); font-weight: 400; }
  @font-face { font-family: 'Fira Sans'; src: url(fira-sans-600.woff2) format('woff2'); font-weight: 600; }
  * { margin: 0; box-sizing: border-box; }
  html, body { width: 1200px; height: 630px; overflow: hidden; }
  body { position: relative; font-family: 'Fira Sans', sans-serif; color: #16181b; background: #fbfbf8; }
  .lockup { position: absolute; left: 72px; top: 72px; height: 34px; }
  h1 { position: absolute; left: 72px; top: 168px; width: 520px; font-size: 70px; font-weight: 600; line-height: 1; letter-spacing: -0.03em; }
  p { position: absolute; left: 72px; top: 418px; width: 470px; font-size: 25px; line-height: 1.35; color: #4b5058; }
  svg { position: absolute; inset: 0; }
  text { font-family: 'Fira Sans', sans-serif; font-size: 20px; font-weight: 600; fill: #16181b; }
  text.signal { fill: #c8102e; }
</style>
</head>
<body>
  <img class="lockup" src="lockup-light.svg" alt="" />
  <h1>One app for every container Engine.</h1>
  <p>Runs on your machine. Installs nothing on your servers.</p>
  <svg width="1200" height="630" viewBox="0 0 1200 630">
    <rect x="612" y="288" width="30" height="78" rx="3" fill="#16181b"/>
    ${MAP}
  </svg>
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
    for (const weight of [400, 600]) {
      copyFileSync(
        join(fonts, `fira-sans/files/fira-sans-latin-${weight}-normal.woff2`),
        join(work, `fira-sans-${weight}.woff2`),
      );
    }
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
