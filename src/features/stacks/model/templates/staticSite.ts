import type { StackTemplate } from './template';

export const staticSite: StackTemplate = {
  id: 'static-site',
  title: 'stacks.templates.staticSite.title',
  description: 'stacks.templates.staticSite.description',
  name: 'site',
  compose: `services:
  web:
    image: nginx:1.27-alpine
    ports:
      - "\${HTTP_PORT}:80"
    volumes:
      # Starts with nginx's welcome page; put your site's files here.
      - site:/usr/share/nginx/html
    restart: unless-stopped

volumes:
  site:
`,
  env: `# The port on the host that serves the site.
HTTP_PORT=8080
`,
};
