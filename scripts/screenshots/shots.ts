/**
 * The website's screenshots: each a page of the real UI on the demo data, in
 * a theme, after some steps. `capture.ts` photographs them; the app's
 * accessibility test (`src/app/a11y.test.tsx`) checks every one of these pages.
 */
import { PROD, SWARM_ID, hexId, swarmServices } from './data';

/**
 * A step is `click:<label>`, `fill:<placeholder>=<value>` or `wait:<ms>`. A
 * label written `@key` is that catalogue key's text in the shot's language, so
 * the same steps work in every language.
 */
export interface Shot {
  name: string;
  route: string;
  theme?: 'light' | 'dark';
  steps?: string[];
  hold?: number;
  fastClock?: boolean;
  /**
   * Demo data beyond the Hosts: `clusters` selects the demo EKS Cluster,
   * `nomad` the demo Nomad Cluster.
   */
  demo?: 'clusters' | 'nomad';
}

const container = (name: string) => hexId(`${PROD}/${name}`);

export const SHOTS: Shot[] = [
  { name: 'overview-light', route: '/' },
  { name: 'overview-dark', route: '/', theme: 'dark' },
  { name: 'board-light', route: '/board' },
  { name: 'board-dark', route: '/board', theme: 'dark' },
  { name: 'containers-light', route: '/containers' },
  { name: 'containers-dark', route: '/containers', theme: 'dark' },
  { name: 'container-logs', route: `/containers/${container('shop-api-1')}/logs`, theme: 'dark' },
  { name: 'container-logs-light', route: `/containers/${container('shop-api-1')}/logs` },
  {
    name: 'container-terminal',
    route: `/containers/${container('shop-db-1')}/exec`,
    theme: 'dark',
  },
  {
    name: 'container-stats',
    route: `/containers/${container('shop-api-1')}/stats`,
    hold: 8500,
    fastClock: true,
  },
  { name: 'stacks', route: '/stacks' },
  {
    name: 'stack-editor',
    route: '/stacks/shop',
    steps: ['click:@stacks.tab.definition'],
    hold: 3500,
  },
  {
    name: 'stack-editor-dark',
    route: '/stacks/shop',
    theme: 'dark',
    steps: ['click:@stacks.tab.definition'],
    hold: 3500,
  },
  {
    name: 'stack-logs',
    route: '/stacks/shop',
    theme: 'dark',
    steps: ['click:@stacks.tab.logs'],
    hold: 3500,
  },
  { name: 'images', route: '/images' },
  { name: 'volumes', route: '/volumes' },
  { name: 'networks', route: '/networks' },
  { name: 'events', route: '/events' },
  {
    name: 'add-host-ssh',
    route: '/hosts/new',
    steps: [
      'fill:production=backup-eu-2',
      'fill:server.example.com=backup-eu-2.example.com',
      'fill:deploy=deploy',
    ],
    hold: 3500,
  },
  {
    name: 'add-host-ssh-dark',
    route: '/hosts/new',
    theme: 'dark',
    steps: [
      'fill:production=backup-eu-2',
      'fill:server.example.com=backup-eu-2.example.com',
      'fill:deploy=deploy',
    ],
    hold: 3500,
  },
  { name: 'host-ssh', route: `/hosts/${PROD}` },
  { name: 'swarm-services', route: `/swarms/${SWARM_ID}/services` },
  {
    name: 'swarm-summary',
    route: `/swarms/${SWARM_ID}`,
    steps: ['click:@swarm.join.reveal'],
    hold: 3500,
  },
  { name: 'swarm-nodes', route: `/swarms/${SWARM_ID}/nodes` },
  { name: 'swarm-tasks', route: `/swarms/${SWARM_ID}/tasks` },
  { name: 'swarm-stacks', route: `/swarms/${SWARM_ID}/stacks` },
  { name: 'swarm-secrets', route: `/swarms/${SWARM_ID}/secrets` },
  { name: 'swarm-service', route: `/swarms/${SWARM_ID}/services/${swarmServices[1]!.id}` },
  { name: 'cluster-overview', route: '/', demo: 'clusters' },
  { name: 'cluster-overview-dark', route: '/', theme: 'dark', demo: 'clusters' },
  { name: 'add-cluster', route: '/clusters/new', demo: 'clusters' },
  { name: 'cluster-workloads', route: '/workloads', demo: 'clusters' },
  { name: 'cluster-workload', route: '/workloads/deployments/shop/api', demo: 'clusters' },
  { name: 'cluster-pods', route: '/pods', demo: 'clusters' },
  { name: 'cluster-pod', route: '/pods/shop/api-6b8f4d7c5-xr4np', demo: 'clusters' },
  { name: 'cluster-services', route: '/services', demo: 'clusters' },
  { name: 'cluster-ingress', route: '/ingresses/shop/storefront/rules', demo: 'clusters' },
  {
    name: 'cluster-secret',
    route: '/secrets/shop/db-credentials',
    demo: 'clusters',
    steps: ['click:@config.reveal'],
    hold: 3500,
  },
  { name: 'cluster-nodes', route: '/nodes', demo: 'clusters' },
  { name: 'cluster-events', route: '/cluster-events', demo: 'clusters' },
  { name: 'nomad-jobs', route: '/nomad/jobs', demo: 'nomad' },
  {
    name: 'nomad-run-plan',
    route: '/nomad/jobs/shop/shop-api',
    demo: 'nomad',
    steps: ['click:@nomad.job.editAndRun', 'wait:800', 'click:@nomad.run.plan'],
    hold: 3500,
  },
];
