import { expect, test } from 'bun:test';
import { recommendation } from './recommendation';

test.each([
  ['macos', 'arm64', "You're on macOS on Apple silicon. We've put its downloads first."],
  ['macos', 'x64', "You're on macOS on Intel. We've put its downloads first."],
  ['macos', undefined, "You're on macOS. We've put its downloads first."],
  ['windows', 'arm64', "You're on Windows on ARM64. We've put its downloads first."],
  ['linux', 'x64', "You're on Linux on x64. We've put its downloads first."],
  ['linux', undefined, "You're on Linux. We've put its downloads first."],
] as const)('%s %s', (os, arch, text) => {
  expect(recommendation(os, arch)).toBe(text);
});
