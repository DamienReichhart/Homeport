import { expect, test } from 'bun:test';
import { type RecommendationText, recommendation } from './recommendation';

const english: RecommendationText = {
  sentence: "You're on {platform}. We've put its downloads first.",
  onArch: '{os} on {arch}',
  appleSilicon: 'Apple silicon',
  intel: 'Intel',
  arm64: 'ARM64',
  x64: 'x64',
};

test.each([
  ['macos', 'arm64', "You're on macOS on Apple silicon. We've put its downloads first."],
  ['macos', 'x64', "You're on macOS on Intel. We've put its downloads first."],
  ['macos', undefined, "You're on macOS. We've put its downloads first."],
  ['windows', 'arm64', "You're on Windows on ARM64. We've put its downloads first."],
  ['linux', 'x64', "You're on Linux on x64. We've put its downloads first."],
  ['linux', undefined, "You're on Linux. We've put its downloads first."],
] as const)('%s %s', (os, arch, text) => {
  expect(recommendation(os, arch, english)).toBe(text);
});

test('follows the order of another language', () => {
  const text = { ...english, sentence: 'Vous êtes sur {platform}.', onArch: '{os} ({arch})' };
  expect(recommendation('macos', 'arm64', text)).toBe('Vous êtes sur macOS (Apple silicon).');
});
