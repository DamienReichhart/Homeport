import { t } from '@/core/i18n';
import type { Release } from '@/core/release';
import { type Locale, localePath, REPO_URL } from '@/core/site';
import { href } from '@/shared/lib/href';

/** The home page's JSON-LD: Homeport as a SoftwareApplication, for search engines, in `locale`. */
export function softwareApplication(
  site: URL | undefined,
  release: Release | undefined,
  locale: Locale,
): Record<string, unknown> {
  return {
    '@context': 'https://schema.org',
    '@type': 'SoftwareApplication',
    name: 'Homeport',
    description: t(locale, 'site.description'),
    applicationCategory: 'DeveloperApplication',
    operatingSystem: 'Windows, macOS, Linux',
    license: 'https://www.gnu.org/licenses/agpl-3.0.html',
    url: new URL(href(localePath('/', locale)), site).href,
    downloadUrl: new URL(href(localePath('/download/', locale)), site).href,
    codeRepository: REPO_URL,
    ...(release && { softwareVersion: release.version }),
    offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
    author: { '@type': 'Person', name: 'Damien Reichhart' },
  };
}
