import '@testing-library/jest-dom/vitest';
import { clearMocks } from '@tauri-apps/api/mocks';
import { cleanup } from '@testing-library/react';

afterEach(async () => {
  cleanup();
  // Event listeners unregister asynchronously on unmount; let them finish first.
  await new Promise((resolve) => setTimeout(resolve, 0));
  clearMocks();
});
