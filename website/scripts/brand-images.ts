/**
 * Generates the site's raster brand images into public/:
 *
 *   og.png                 1200×630 social card (rendered by headless Firefox, so it
 *                          uses Geist and the real app screenshot)
 *   apple-touch-icon.png   180×180, and icon-192.png / icon-512.png for the manifest
 *
 *   bun scripts/brand-images.ts        (from website/)
 *
 * Run it again after changing the mark or the overview screenshot. Set FIREFOX
 * to use a Firefox other than the one on PATH.
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

const CARD = /* html */ `<!doctype html>
<html>
<head>
<meta charset="utf-8" />
<style>
  @font-face { font-family: Geist; src: url(geist.woff2) format('woff2'); font-weight: 100 900; }
  * { margin: 0; box-sizing: border-box; }
  html, body { width: 1200px; height: 630px; overflow: hidden; }
  body {
    position: relative;
    font-family: Geist, sans-serif;
    color: #edf3f7;
    background:
      radial-gradient(700px 420px at 15% -10%, rgb(0 106 165 / 0.45), transparent 70%),
      radial-gradient(500px 300px at 100% 100%, rgb(248 132 45 / 0.14), transparent 70%),
      linear-gradient(180deg, #0d1c2e, #0b121b);
  }
  .grid {
    position: absolute; inset: 0;
    background-image:
      linear-gradient(rgb(190 214 235 / 0.06) 1px, transparent 1px),
      linear-gradient(90deg, rgb(190 214 235 / 0.06) 1px, transparent 1px);
    background-size: 48px 48px;
    mask-image: linear-gradient(90deg, #000 30%, transparent 75%);
  }
  .text { position: absolute; left: 72px; top: 72px; width: 560px; }
  .lockup { height: 56px; display: block; }
  h1 { margin-top: 64px; font-size: 60px; font-weight: 650; line-height: 1.04; letter-spacing: -0.035em; }
  h1 span { color: #6cc1ea; }
  p { margin-top: 28px; font-size: 24px; line-height: 1.4; color: #a3b3c2; }
  .meta { position: absolute; left: 72px; bottom: 64px; font-size: 20px; color: #73879a; letter-spacing: 0.01em; }
  .meta b { color: #edf3f7; font-weight: 600; }
  .shot {
    position: absolute; left: 690px; top: 96px; width: 900px;
    border-radius: 18px; border: 1px solid rgb(190 214 235 / 0.18);
    box-shadow: 0 40px 100px -30px rgb(0 0 0 / 0.8), 0 0 0 8px rgb(190 214 235 / 0.04);
  }
</style>
</head>
<body>
  <div class="grid"></div>
  <div class="text">
    <img class="lockup" src="lockup-dark.svg" alt="" />
    <h1>Every Docker Engine, <span>docked in one place.</span></h1>
    <p>A desktop app for your local and remote Docker Engines — over SSH, TLS or TCP.</p>
  </div>
  <div class="meta"><b>Free &amp; open source</b> · Windows · macOS · Linux</div>
  <img class="shot" src="overview-dark.png" alt="" />
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
    copyFileSync(
      join(SITE, 'node_modules/@fontsource-variable/geist/files/geist-latin-wght-normal.woff2'),
      join(work, 'geist.woff2'),
    );
    copyFileSync(join(REPO, 'assets/brand/lockup-dark.svg'), join(work, 'lockup-dark.svg'));
    copyFileSync(
      join(SITE, 'src/assets/screenshots/overview-dark.png'),
      join(work, 'overview-dark.png'),
    );
    writeFileSync(join(work, 'card.html'), CARD);
    const profile = join(work, 'profile');
    mkdirSync(profile);

    const out = join(work, 'og.png');
    const proc = Bun.spawn(
      [
        FIREFOX,
        '--headless',
        '--no-remote',
        '-profile',
        profile,
        '--screenshot',
        out,
        '--window-size=1200,630',
        `file://${join(work, 'card.html')}`,
      ],
      { stdout: 'ignore', stderr: 'ignore' },
    );
    await proc.exited;
    if (!existsSync(out)) throw new Error('Firefox produced no screenshot');
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
