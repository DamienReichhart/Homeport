import type { Release } from '@/core/release';
import { DESCRIPTION, REPO_URL } from '@/core/site';
import { href } from '@/shared/lib/href';

/** The home page's JSON-LD: Homeport as a SoftwareApplication, for search engines. */
export function softwareApplication(
  site: URL | undefined,
  release: Release | undefined,
): Record<string, unknown> {
  return {
    '@context': 'https://schema.org',
    '@type': 'SoftwareApplication',
    name: 'Homeport',
    description: DESCRIPTION,
    applicationCategory: 'DeveloperApplication',
    operatingSystem: 'Windows, macOS, Linux',
    license: 'https://opensource.org/licenses/MIT',
    url: new URL(href('/'), site).href,
    downloadUrl: new URL(href('/download/'), site).href,
    codeRepository: REPO_URL,
    ...(release && { softwareVersion: release.version }),
    offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
    author: { '@type': 'Person', name: 'Damien Reichhart' },
  };
}
