import '@testing-library/jest-dom/vitest';
import { clearMocks } from '@tauri-apps/api/mocks';
import { cleanup } from '@testing-library/react';

// jsdom has no matchMedia; uPlot and the theme store read it at import time.
window.matchMedia ??= (query: string) =>
  ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: () => {},
    removeEventListener: () => {},
    addListener: () => {},
    removeListener: () => {},
    dispatchEvent: () => false,
  }) as MediaQueryList;

afterEach(async () => {
  cleanup();
  // Event listeners unregister asynchronously on unmount; let them finish first.
  await new Promise((resolve) => setTimeout(resolve, 0));
  clearMocks();
});
