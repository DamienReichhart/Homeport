/**
 * Best-effort detection of the visitor's OS and CPU architecture, in the browser.
 * User-Agent Client Hints give the architecture in Chromium browsers; elsewhere
 * the architecture is only known when the user agent says so (Safari on Apple
 * silicon still reports "Intel", so a Mac without hints is left undecided).
 */
export type DetectedOs = 'windows' | 'macos' | 'linux' | 'other';
export type DetectedArch = 'x64' | 'arm64' | undefined;

export const OS_NAMES: Record<Exclude<DetectedOs, 'other'>, string> = {
  windows: 'Windows',
  macos: 'macOS',
  linux: 'Linux',
};

interface UADataValues {
  platform?: string;
  architecture?: string;
  bitness?: string;
}
interface UAData {
  platform: string;
  getHighEntropyValues(hints: string[]): Promise<UADataValues>;
}

export interface Platform {
  os: DetectedOs;
  arch: DetectedArch;
}

function osFrom(text: string): DetectedOs {
  const s = text.toLowerCase();
  if (/android|iphone|ipad|ipod|cros/.test(s)) return 'other';
  if (s.includes('win')) return 'windows';
  if (s.includes('mac')) return 'macos';
  if (s.includes('linux') || s.includes('x11')) return 'linux';
  return 'other';
}

export async function detectPlatform(): Promise<Platform> {
  const ua = navigator.userAgent;
  // iPadOS reports itself as a Mac; a touch screen gives it away.
  const isIpad = /Macintosh/.test(ua) && navigator.maxTouchPoints > 1;
  const uaData = (navigator as Navigator & { userAgentData?: UAData }).userAgentData;

  if (uaData) {
    try {
      const values = await uaData.getHighEntropyValues(['architecture', 'bitness', 'platform']);
      const os = osFrom(values.platform || uaData.platform);
      const arch =
        values.architecture === 'arm' ? 'arm64' : values.architecture === 'x86' ? 'x64' : undefined;
      return { os: isIpad ? 'other' : os, arch };
    } catch {
      // Fall through to the user agent string.
    }
  }

  const os = isIpad ? 'other' : osFrom(`${navigator.platform} ${ua}`);
  let arch: DetectedArch;
  if (/aarch64|arm64|armv8/i.test(ua)) arch = 'arm64';
  else if (/x86_64|x64|win64|wow64|amd64/i.test(ua) && os !== 'macos') arch = 'x64';
  return { os, arch };
}
