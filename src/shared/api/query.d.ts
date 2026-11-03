import '@tanstack/react-query';

declare module '@tanstack/react-query' {
  interface Register {
    mutationMeta: {
      /** The mutation shows its own error, so the global error toast stays quiet. */
      silent?: boolean;
    };
  }
}
