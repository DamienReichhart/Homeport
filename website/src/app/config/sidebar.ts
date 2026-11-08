import type { StarlightUserConfig } from '@astrojs/starlight/types';

/** The docs' sidebar, in reading order. A new docs page gets its entry here. */
export const sidebar: NonNullable<StarlightUserConfig['sidebar']> = [
  {
    label: 'Getting started',
    items: [
      { label: 'Introduction', slug: 'docs' },
      { label: 'Installation', slug: 'docs/installation' },
      { label: 'Quick start', slug: 'docs/quick-start' },
      { label: 'System requirements', slug: 'docs/requirements' },
    ],
  },
  {
    label: 'Hosts',
    items: [
      { label: 'Hosts and connections', slug: 'docs/hosts' },
      { label: 'Local engines', slug: 'docs/hosts/local' },
      { label: 'SSH', slug: 'docs/hosts/ssh' },
      { label: 'TLS and TCP', slug: 'docs/hosts/tls-tcp' },
    ],
  },
  {
    label: 'Clusters',
    items: [
      { label: 'Clusters and kubeconfigs', slug: 'docs/clusters' },
      { label: 'Local clusters', slug: 'docs/clusters/local' },
      { label: 'Exec plugins', slug: 'docs/clusters/exec-plugins' },
      { label: 'Workloads and Pods', slug: 'docs/clusters/workloads' },
      { label: 'Services and configuration', slug: 'docs/clusters/networking-config' },
      { label: 'Nodes and Events', slug: 'docs/clusters/nodes-events' },
      { label: 'Logs and terminals', slug: 'docs/clusters/logs-exec' },
      { label: 'Port forwarding', slug: 'docs/clusters/port-forward' },
      { label: 'Scaling, restarting and deleting', slug: 'docs/clusters/actions' },
      { label: 'Applying and editing YAML', slug: 'docs/clusters/yaml' },
    ],
  },
  {
    label: 'Using Homeport',
    items: [
      { label: 'Overview and events', slug: 'docs/guides/overview' },
      { label: 'Containers', slug: 'docs/guides/containers' },
      { label: 'Images', slug: 'docs/guides/images' },
      { label: 'Volumes and networks', slug: 'docs/guides/volumes-networks' },
      { label: 'Stacks', slug: 'docs/guides/stacks' },
      { label: 'Synced Stacks', slug: 'docs/guides/synced-stacks' },
      { label: 'Swarms', slug: 'docs/guides/swarm' },
      { label: 'Settings', slug: 'docs/guides/settings' },
      { label: 'Keyboard', slug: 'docs/guides/keyboard' },
    ],
  },
  {
    label: 'Reference',
    items: [
      { label: 'Security and your data', slug: 'docs/reference/security' },
      { label: 'Verifying downloads', slug: 'docs/reference/verifying' },
      { label: 'Updating and uninstalling', slug: 'docs/reference/updating' },
      { label: 'Troubleshooting', slug: 'docs/reference/troubleshooting' },
      { label: 'FAQ', slug: 'docs/reference/faq' },
      { label: 'Glossary', slug: 'docs/reference/glossary' },
    ],
  },
  {
    label: 'Project',
    items: [
      { label: 'Building from source', slug: 'docs/project/building' },
      { label: 'Contributing', slug: 'docs/project/contributing' },
      { label: 'Changelog', link: '/changelog/' },
    ],
  },
];
