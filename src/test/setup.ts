import '@testing-library/jest-dom/vitest';
import { clearMocks } from '@tauri-apps/api/mocks';
import { cleanup } from '@testing-library/react';
import { toast } from 'sonner';

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

// jsdom has no ResizeObserver; the terminal and charts observe their container.
globalThis.ResizeObserver ??= class {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// jsdom has no pointer capture; sonner's swipe handling calls it on toast clicks.
Element.prototype.setPointerCapture ??= () => {};
Element.prototype.releasePointerCapture ??= () => {};
Element.prototype.hasPointerCapture ??= () => false;
// jsdom has no scrollIntoView; Radix selects scroll the chosen option into view.
Element.prototype.scrollIntoView ??= () => {};

afterEach(async () => {
  // Toasts live in sonner's module state, so they would outlive the test.
  toast.dismiss();
  cleanup();
  // Event listeners unregister asynchronously on unmount; let them finish first.
  await new Promise((resolve) => setTimeout(resolve, 0));
  clearMocks();
});
