import type { StackTemplate } from './template';

export const uptimeKuma: StackTemplate = {
  id: 'uptime-kuma',
  title: 'stacks.templates.uptimeKuma.title',
  description: 'stacks.templates.uptimeKuma.description',
  name: 'uptime-kuma',
  compose: `services:
  uptime-kuma:
    image: louislam/uptime-kuma:1.23.16
    ports:
      - "\${HTTP_PORT}:3001"
    volumes:
      - uptime-kuma-data:/app/data
    restart: unless-stopped

volumes:
  uptime-kuma-data:
`,
  env: `# The port on the host for its web interface; create the admin account there.
HTTP_PORT=3001
`,
};
