import { afterEach, describe, expect, test } from 'bun:test';
import { detectPlatform } from './platform';

const original = Object.getOwnPropertyDescriptor(globalThis, 'navigator');

function browser(nav: {
  userAgent: string;
  platform?: string;
  maxTouchPoints?: number;
  userAgentData?: { platform: string; architecture?: string; fails?: boolean };
}) {
  const hints = nav.userAgentData && {
    platform: nav.userAgentData.platform,
    getHighEntropyValues: () =>
      nav.userAgentData!.fails
        ? Promise.reject(new Error('blocked'))
        : Promise.resolve({ architecture: nav.userAgentData!.architecture }),
  };
  Object.defineProperty(globalThis, 'navigator', {
    configurable: true,
    value: { platform: '', maxTouchPoints: 0, ...nav, userAgentData: hints },
  });
}

afterEach(() => {
  if (original) Object.defineProperty(globalThis, 'navigator', original);
});

const WINDOWS = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/130.0';
const MAC_SAFARI =
  'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 Safari/605.1';
const LINUX_ARM = 'Mozilla/5.0 (X11; Linux aarch64; rv:131.0) Gecko/20100101 Firefox/131.0';
const ANDROID = 'Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 Chrome/130.0 Mobile';

describe('detectPlatform', () => {
  test('takes the architecture from client hints', async () => {
    browser({ userAgent: MAC_SAFARI, userAgentData: { platform: 'macOS', architecture: 'arm' } });
    expect(await detectPlatform()).toEqual({ os: 'macos', arch: 'arm64' });
  });

  test('falls back on the user agent when the hints fail', async () => {
    browser({ userAgent: WINDOWS, userAgentData: { platform: 'Windows', fails: true } });
    expect(await detectPlatform()).toEqual({ os: 'windows', arch: 'x64' });
  });

  test('reads an ARM Linux from the user agent', async () => {
    browser({ userAgent: LINUX_ARM, platform: 'Linux aarch64' });
    expect(await detectPlatform()).toEqual({ os: 'linux', arch: 'arm64' });
  });

  test('leaves a Mac without hints undecided, since Safari always says Intel', async () => {
    browser({ userAgent: MAC_SAFARI, platform: 'MacIntel' });
    expect(await detectPlatform()).toEqual({ os: 'macos', arch: undefined });
  });

  test('treats an iPad, which says it is a Mac, as another device', async () => {
    browser({ userAgent: MAC_SAFARI, platform: 'MacIntel', maxTouchPoints: 5 });
    expect((await detectPlatform()).os).toBe('other');
  });

  test('treats phones as another device', async () => {
    browser({ userAgent: ANDROID, platform: 'Linux armv8l' });
    expect((await detectPlatform()).os).toBe('other');
  });
});
