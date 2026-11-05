import type { StackTemplate } from './template';

export const postgres: StackTemplate = {
  id: 'postgres',
  title: 'stacks.templates.postgres.title',
  description: 'stacks.templates.postgres.description',
  name: 'postgres',
  compose: `services:
  db:
    image: postgres:17-alpine
    environment:
      POSTGRES_USER: \${POSTGRES_USER}
      POSTGRES_PASSWORD: \${POSTGRES_PASSWORD}
      POSTGRES_DB: \${POSTGRES_DB}
    volumes:
      - db-data:/var/lib/postgresql/data
    restart: unless-stopped

  adminer:
    image: adminer:4.8.1
    ports:
      - "\${ADMINER_PORT}:8080"
    depends_on:
      - db
    restart: unless-stopped

volumes:
  db-data:
`,
  env: `POSTGRES_USER=app
# The database password: choose a long, random one before deploying.
POSTGRES_PASSWORD=
POSTGRES_DB=app
# The port on the host for Adminer; sign in to the server "db".
ADMINER_PORT=8081
`,
};
