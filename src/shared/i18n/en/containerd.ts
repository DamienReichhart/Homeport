/** English text of the `containerd` namespace; every key starts with `containerd.`. */
export default {
  'containerd.namespaces.nav': 'Namespaces',
  'containerd.namespaces.title': 'Namespaces',
  'containerd.namespaces.summary':
    '{count, plural, one {# namespace} other {# namespaces}}. Every containerd object lives in one: nerdctl uses default, Kubernetes k8s.io, Docker moby.',
  'containerd.namespaces.new': 'New namespace',
  'containerd.namespaces.created': 'Namespace {name} created',
  'containerd.namespaces.removed': 'Namespace {name} removed',
  'containerd.namespaces.shown': 'Shown',
  'containerd.namespaces.show': 'Show',
  'containerd.namespaces.removeOne': 'Remove {name}',
  'containerd.namespaces.notEmpty':
    'Only empty namespaces can be removed. Remove its containers and images first.',
  'containerd.namespaces.remove.title': 'Remove the namespace {name}?',
  'containerd.namespaces.remove.description':
    'containerd refuses if it still holds snapshots or content that the counts don’t show.',
  'containerd.namespaces.column.name': 'Name',
  'containerd.namespaces.column.containers': 'Containers',
  'containerd.namespaces.column.images': 'Images',
  'containerd.namespaces.column.labels': 'Labels',
  'containerd.namespaces.empty': 'No namespaces',
  'containerd.namespaces.emptyDescription':
    'containerd makes default the first time something is created in it.',
  'containerd.namespaces.newDialog.title': 'New namespace',
  'containerd.namespaces.newDialog.description':
    'An empty namespace, to keep containers and images apart from the others.',
  'containerd.namespaces.newDialog.name': 'Name',
  'containerd.namespaces.newDialog.invalid':
    'Use letters and digits, separated by single dots, dashes or underscores.',
  'containerd.namespaces.newDialog.create': 'Create',
};
