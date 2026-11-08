/** English text of the `apply` namespace (applying YAML to a Cluster); every key starts with `apply.`. */
export default {
  'apply.nav': 'Apply YAML',
  'apply.title': 'Apply YAML',
  'apply.description':
    'Paste or open manifests, see what they would change on {cluster}, then apply them, like kubectl apply.',
  'apply.open': 'Open file…',
  'apply.openTitle': 'Open manifests',
  'apply.openFailed': 'Homeport couldn’t read that file',
  'apply.manifests': 'Kubernetes manifests',
  'apply.editor': 'Manifests to apply',
  'apply.defaultNamespace': 'Documents without a namespace go to {namespace}.',
  'apply.defaultNamespaceCluster':
    'Documents without a namespace go to the cluster’s default namespace.',
} as const;
