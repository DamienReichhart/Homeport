import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './app/App';
import { logFailure, toAppError } from './shared/api';
import './shared/styles/global.css';

// Anything that escapes React and the command wrapper still reaches the log.
window.addEventListener('error', (e) => logFailure(toAppError(e.error ?? e.message), 'error'));
window.addEventListener('unhandledrejection', (e) => logFailure(toAppError(e.reason), 'error'));

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
