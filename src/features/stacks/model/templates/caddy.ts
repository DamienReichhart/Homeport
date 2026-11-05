import type { StackTemplate } from './template';

export const caddy: StackTemplate = {
  id: 'caddy',
  title: 'stacks.templates.caddy.title',
  description: 'stacks.templates.caddy.description',
  name: 'proxy',
  compose: `services:
  caddy:
    image: caddy:2.8-alpine
    command: ["caddy", "reverse-proxy", "--from", "\${SITE_ADDRESS}", "--to", "\${UPSTREAM}"]
    ports:
      - "80:80"
      - "443:443"
      - "443:443/udp"
    volumes:
      # Certificates Caddy obtains, kept across redeploys.
      - caddy-data:/data
      - caddy-config:/config
    restart: unless-stopped

volumes:
  caddy-data:
  caddy-config:
`,
  env: `# What Caddy serves: a domain pointing at this host, such as example.com,
# for automatic HTTPS, or :80 for plain HTTP.
SITE_ADDRESS=:80
# Where requests go, such as 10.0.0.5:8080 or a container on a shared network.
UPSTREAM=
`,
};
