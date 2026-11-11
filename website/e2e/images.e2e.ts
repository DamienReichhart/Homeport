/**
 * The site's images are sharp on a HiDPI screen and small on a phone.
 *
 * On the built site, in a 1440×900 window at a pixel density of 2, every
 * raster image on the home page, the download page and a docs page has at
 * least two pixels for each CSS pixel it is drawn at. Screenshots are captured
 * at 2880×1800 (`bun run screenshots`) and served at several widths
 * (shared/ui/Screenshot.astro), the browser choosing by `sizes`; this fails
 * when a shot is recaptured too small, a width is dropped, or `sizes` no longer
 * says how wide a shot is drawn. At a density of 1 and on a phone the browser
 * must keep to the small files.
 */
import { type Page, expect, test } from '@playwright/test';

/** The pages checked, each with whether it must show screenshots. */
const PAGES = [
  { name: 'the home page', path: '', screenshots: true },
  { name: 'the download page', path: 'download/', screenshots: false },
  { name: 'a docs page', path: 'docs/guides/containers/', screenshots: true },
  { name: 'a translated docs page', path: 'ja/docs/guides/containers/', screenshots: true },
];

interface Shown {
  /** The file the browser chose, without its folder. */
  file: string;
  /** Whether it is a screenshot of the app (Screenshot.astro's frame). */
  screenshot: boolean;
  /** The chosen file's width in pixels. */
  naturalWidth: number;
  /** The widest file offered, from `srcset`; the image's own width without one. */
  widest: number;
  /** How wide the whole image is drawn, in CSS pixels: wider than its box when cropped to it. */
  drawnWidth: number;
}

/** Every raster image the page shows, once each has loaded. */
async function shownImages(page: Page, path: string): Promise<Shown[]> {
  await page.goto(path);
  const images = page.locator('img');
  const shown: Shown[] = [];
  for (let i = 0; i < (await images.count()); i++) {
    const image = images.nth(i);
    // The other theme's variant is in the page but not displayed.
    if (!(await image.isVisible())) continue;
    await image.scrollIntoViewIfNeeded(); // screenshots load lazily
    await expect
      .poll(() => image.evaluate((img: HTMLImageElement) => img.complete && img.naturalWidth > 0), {
        message: `image ${i} on ${path || '/'} loads`,
      })
      .toBe(true);
    const facts = await image.evaluate(async (img: HTMLImageElement) => {
      // With a `w` srcset, naturalWidth is divided by the density the browser
      // assumed for the file; the file loaded on its own gives its real pixels.
      const file = new Image();
      file.src = img.currentSrc;
      await file.decode();
      const offered = [...(img.closest('picture')?.querySelectorAll('source') ?? []), img].flatMap(
        (el) =>
          [...(el.getAttribute('srcset') ?? '').matchAll(/\s(\d+)w/g)].map((m) => Number(m[1])),
      );
      const cover = getComputedStyle(img).objectFit === 'cover';
      const byHeight = (img.clientHeight * file.naturalWidth) / file.naturalHeight;
      return {
        file: img.currentSrc.split('/').pop() ?? '',
        screenshot: img.closest('.window') !== null,
        naturalWidth: file.naturalWidth,
        widest: Math.max(file.naturalWidth, ...offered),
        drawnWidth: cover ? Math.max(img.clientWidth, byHeight) : img.clientWidth,
      };
    });
    if (!facts.file.endsWith('.svg')) shown.push(facts);
  }
  return shown;
}

const screenshotsOf = (shown: Shown[]) => shown.filter((image) => image.screenshot);

test.describe('on a HiDPI desktop screen', () => {
  test.use({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 2 });

  for (const { name, path, screenshots } of PAGES) {
    test(`${name} shows every image with two pixels for each CSS pixel`, async ({ page }) => {
      const shown = await shownImages(page, path);
      if (screenshots) expect(screenshotsOf(shown).length).toBeGreaterThan(0);
      for (const image of shown) {
        const sharp =
          image.naturalWidth >= 2 * image.drawnWidth || image.naturalWidth === image.widest;
        expect(
          sharp,
          `${image.file} is ${image.naturalWidth} px wide, drawn ${image.drawnWidth} CSS px wide`,
        ).toBe(true);
      }
    });

    if (screenshots)
      test(`${name} shows its screenshots as AVIF, not from the full-size file`, async ({
        page,
      }) => {
        for (const image of screenshotsOf(await shownImages(page, path))) {
          expect(image.file).toMatch(/\.avif$/);
          expect(image.widest, `${image.file} comes from a 2880 px capture`).toBe(2880);
          expect(image.naturalWidth, image.file).toBeLessThanOrEqual(1920);
        }
      });
  }
});

/** The widest file a screenshot may come from, by screen. */
const SMALL = [
  { screen: 'a desktop screen at a density of 1', width: 1440, height: 900, density: 1, most: 960 },
  { screen: 'a phone at a density of 1', width: 390, height: 844, density: 1, most: 640 },
  { screen: 'a phone at a density of 2', width: 390, height: 844, density: 2, most: 960 },
  { screen: 'a phone at a density of 3', width: 390, height: 844, density: 3, most: 1600 },
];

for (const { screen, width, height, density, most } of SMALL) {
  test.describe(`on ${screen}`, () => {
    test.use({ viewport: { width, height }, deviceScaleFactor: density });

    for (const { name, path, screenshots } of PAGES) {
      if (!screenshots) continue;
      test(`${name} takes screenshots no wider than ${most} px`, async ({ page }) => {
        const shown = screenshotsOf(await shownImages(page, path));
        expect(shown.length).toBeGreaterThan(0);
        for (const image of shown) expect(image.naturalWidth, image.file).toBeLessThanOrEqual(most);
      });
    }
  });
}
