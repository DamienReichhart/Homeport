import { afterEach, beforeEach, describe, expect, mock, spyOn, test } from 'bun:test';
import { loadReleases } from './api';

const sha = (c: string) => c.repeat(64);
const dmg = 'Homeport_0.4.0_aarch64.dmg';

const releases = [
  {
    tag_name: 'v0.5.0-beta.1',
    name: null,
    html_url: 'https://github.com/x/beta',
    published_at: '2026-11-04T00:00:00Z',
    prerelease: true,
    draft: false,
    body: null,
    assets: [{ name: 'Homeport_0.5.0_aarch64.dmg', size: 1, browser_download_url: 'beta.dmg' }],
  },
  {
    tag_name: 'v0.6.0',
    name: null,
    html_url: 'https://github.com/x/draft',
    published_at: null,
    prerelease: false,
    draft: true,
    body: null,
    assets: [],
  },
  {
    tag_name: 'v0.4.0',
    name: null,
    html_url: 'https://github.com/x/v0.4.0',
    published_at: '2026-11-03T00:00:00Z',
    prerelease: false,
    draft: false,
    body: null,
    assets: [
      { name: dmg, size: 1, browser_download_url: 'stable.dmg' },
      { name: 'SHA256SUMS.txt', size: 1, browser_download_url: 'https://example.com/sums' },
    ],
  },
];

function respond(routes: Record<string, Response>) {
  return mock((url: string | URL | Request) => {
    const response = routes[String(url)];
    return Promise.resolve(response ?? new Response('missing', { status: 404 }));
  });
}

const api = 'https://api.github.com/repos/DamienReichhart/Homeport/releases?per_page=50';
const env = { ...process.env };

beforeEach(() => {
  delete process.env.RELEASES_REQUIRED;
  delete process.env.GITHUB_TOKEN;
});

afterEach(() => {
  mock.restore();
  process.env = { ...env };
});

describe('loadReleases', () => {
  test('drops drafts and gives the latest stable release its checksums', async () => {
    spyOn(globalThis, 'fetch').mockImplementation(
      respond({
        [api]: Response.json(releases),
        'https://example.com/sums': new Response(`${sha('a')}  ${dmg}\n${sha('b')}  other.dmg\n`),
      }) as unknown as typeof fetch,
    );

    const result = await loadReleases();

    expect(result.map((r) => r.tag)).toEqual(['v0.5.0-beta.1', 'v0.4.0']);
    expect(result[1]!.installers[0]!.sha256).toBe(sha('a'));
    expect(result[0]!.installers[0]!.sha256).toBeUndefined();
  });

  test('sends the token to the GitHub API only', async () => {
    process.env.GITHUB_TOKEN = 'secret';
    const fetch = spyOn(globalThis, 'fetch').mockImplementation(
      respond({
        [api]: Response.json(releases),
        'https://example.com/sums': new Response(''),
      }) as unknown as typeof globalThis.fetch,
    );

    await loadReleases();

    const headers = fetch.mock.calls.map(([url, init]): [string, string | undefined] => [
      String(url),
      (init?.headers as Record<string, string>).authorization,
    ]);
    expect(headers).toEqual([
      [api, 'Bearer secret'],
      ['https://example.com/sums', undefined],
    ]);
  });

  test('builds without releases when GitHub fails', async () => {
    spyOn(globalThis, 'fetch').mockImplementation(respond({}) as unknown as typeof fetch);
    const warn = spyOn(console, 'warn').mockImplementation(() => {});

    expect(await loadReleases()).toEqual([]);
    expect(warn).toHaveBeenCalledTimes(1);
  });

  test('fails the build when releases are required', async () => {
    process.env.RELEASES_REQUIRED = '1';
    spyOn(globalThis, 'fetch').mockImplementation(respond({}) as unknown as typeof fetch);

    expect(loadReleases()).rejects.toThrow('404');
  });
});
