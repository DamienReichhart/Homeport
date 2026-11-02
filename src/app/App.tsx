import { useState } from 'react';
import { RouterProvider } from 'react-router-dom';
import { AppProviders } from './providers/AppProviders';
import { createAppRouter } from './routes/router';

export function App() {
  const [router] = useState(() => createAppRouter());
  return (
    <AppProviders>
      <RouterProvider router={router} />
    </AppProviders>
  );
}
