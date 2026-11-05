import type { StackTemplate } from './template';

export const wordpress: StackTemplate = {
  id: 'wordpress',
  title: 'stacks.templates.wordpress.title',
  description: 'stacks.templates.wordpress.description',
  name: 'wordpress',
  compose: `services:
  wordpress:
    image: wordpress:6.6-apache
    ports:
      - "\${HTTP_PORT}:80"
    environment:
      WORDPRESS_DB_HOST: db
      WORDPRESS_DB_USER: wordpress
      WORDPRESS_DB_PASSWORD: \${DB_PASSWORD}
      WORDPRESS_DB_NAME: wordpress
    volumes:
      - wordpress-data:/var/www/html
    depends_on:
      - db
    restart: unless-stopped

  db:
    image: mariadb:11.4
    environment:
      MARIADB_DATABASE: wordpress
      MARIADB_USER: wordpress
      MARIADB_PASSWORD: \${DB_PASSWORD}
      MARIADB_ROOT_PASSWORD: \${DB_ROOT_PASSWORD}
    volumes:
      - db-data:/var/lib/mysql
    restart: unless-stopped

volumes:
  wordpress-data:
  db-data:
`,
  env: `# The port on the host for the site.
HTTP_PORT=8080
# The database passwords: choose two long, random ones before deploying.
DB_PASSWORD=
DB_ROOT_PASSWORD=
`,
};
