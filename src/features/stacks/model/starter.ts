/** The definition a new Stack starts from. */
export const STARTER = `services:
  web:
    image: nginx:alpine
    ports:
      - "8080:80"
    restart: unless-stopped
`;
