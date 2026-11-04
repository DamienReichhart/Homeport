import type { Release } from '@/core/release';

/** A prerelease with installers that is newer than the latest release, offered for testing. */
export function newerPrerelease(
  releases: Release[],
  latest: Release | undefined,
): Release | undefined {
  return releases.find(
    (r) =>
      r.prerelease && r.installers.length > 0 && (!latest || r.publishedAt > latest.publishedAt),
  );
}
