import type { StackTemplate } from './template';

export const redis: StackTemplate = {
  id: 'redis',
  title: 'stacks.templates.redis.title',
  description: 'stacks.templates.redis.description',
  name: 'redis',
  compose: `services:
  redis:
    image: redis:7.4-alpine
    command: ["redis-server", "--appendonly", "yes", "--requirepass", "\${REDIS_PASSWORD}"]
    ports:
      - "\${REDIS_PORT}:6379"
    volumes:
      - redis-data:/data
    restart: unless-stopped

volumes:
  redis-data:
`,
  env: `# The password clients authenticate with: choose a long, random one.
REDIS_PASSWORD=
# The port on the host; remove the ports: section to keep it private.
REDIS_PORT=6379
`,
};
