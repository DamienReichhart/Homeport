import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './app/App';
import { applyLanguage } from '@/core/settings';
import { commands, logFailure, toAppError } from '@/shared/api';
import './shared/styles/global.css';

// Anything that escapes React and the command wrapper still reaches the log.
window.addEventListener('error', (e) => logFailure(toAppError(e.error ?? e.message), 'error'));
window.addEventListener('unhandledrejection', (e) => logFailure(toAppError(e.reason), 'error'));

// The first frame is already in the user's language: no flash of English.
const language = await commands
  .getSettings()
  .then((s) => s.language)
  .catch(() => undefined);
await applyLanguage(language);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
