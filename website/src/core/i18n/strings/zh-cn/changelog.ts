import type { Translation } from '../../types';
import type en from '../en/changelog';

/** The changelog page, in Simplified Chinese. Still in English: translate each value of ../en/changelog.ts. */
export default {
  'changelog.title': 'Changelog — Homeport',
  'changelog.description':
    "What's new in each Homeport release: features, fixes and other changes.",
  'changelog.eyebrow': 'Changelog',
  'changelog.heading': "What's new in Homeport",
  'changelog.lead':
    "Every release, newest first. Notes are generated from the project's commits; the files are on the {link}.",
  'changelog.downloadPage': 'download page',
  'changelog.notesInEnglish': 'The release notes below are in English, as published on GitHub.',
  'changelog.empty': 'The release history is on {link}.',
  'changelog.github': 'GitHub',
  'changelog.latest': 'Latest',
  'changelog.prerelease': 'Prerelease',
  'changelog.onGitHub': 'Release on GitHub',
  'changelog.download': 'Download',
} satisfies Translation<typeof en>;
