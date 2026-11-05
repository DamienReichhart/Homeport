import type { StackTemplate } from './template';

export const mariadb: StackTemplate = {
  id: 'mariadb',
  title: 'stacks.templates.mariadb.title',
  description: 'stacks.templates.mariadb.description',
  name: 'mariadb',
  compose: `services:
  db:
    image: mariadb:11.4
    environment:
      MARIADB_ROOT_PASSWORD: \${MARIADB_ROOT_PASSWORD}
      MARIADB_DATABASE: \${MARIADB_DATABASE}
      MARIADB_USER: \${MARIADB_USER}
      MARIADB_PASSWORD: \${MARIADB_PASSWORD}
    ports:
      - "\${MARIADB_PORT}:3306"
    volumes:
      - db-data:/var/lib/mysql
    restart: unless-stopped

volumes:
  db-data:
`,
  env: `# The root password: choose a long, random one before deploying.
MARIADB_ROOT_PASSWORD=
MARIADB_DATABASE=app
MARIADB_USER=app
# The app user's password: choose another long, random one.
MARIADB_PASSWORD=
# The port on the host; remove the ports: section to keep it private.
MARIADB_PORT=3306
`,
};
