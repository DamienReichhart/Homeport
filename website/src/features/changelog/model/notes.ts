import { marked } from 'marked';

/**
 * Release notes are written by scripts/release.ts from Conventional Commits. The
 * download table at the end is for GitHub; the download page covers it here.
 */
export function releaseNotes(body: string): string {
  const withoutDownloads = body.split(/^## Download\b/m)[0] ?? body;
  // Demote headings so each release's own title stays the page's h2.
  const demoted = withoutDownloads.replace(/^(#{1,4}) /gm, (_, hashes: string) => `${hashes}# `);
  return marked.parse(demoted, { async: false, gfm: true });
}
