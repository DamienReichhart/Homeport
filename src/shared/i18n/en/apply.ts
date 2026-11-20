/** English text of the `apply` namespace (applying YAML to a Cluster); every key starts with `apply.`. */
export default {
  'apply.nav': 'Apply YAML',
  'apply.title': 'Apply YAML',
  'apply.description': 'Preview what manifests would change on {cluster}, then apply them.',
  'apply.open': 'Open file…',
  'apply.openTitle': 'Open manifests',
  'apply.openFailed': 'Homeport couldn’t read that file',
  'apply.manifests': 'Kubernetes manifests',
  'apply.editor': 'Manifests to apply',
  'apply.defaultNamespace': 'Documents without a Namespace go to {namespace}.',
  'apply.defaultNamespaceCluster':
    'Documents without a Namespace go to the Cluster’s default Namespace.',
} as const;
