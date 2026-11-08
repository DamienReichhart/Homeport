/** English text of the `clusters` namespace; every key starts with `clusters.`. */
export default {
  'clusters.title': 'Clusters',
  'clusters.summary': '{count, plural, one {# cluster} other {# clusters}}',
  'clusters.add': 'Add a cluster',
  'clusters.notFound': 'This cluster no longer exists.',
  'clusters.kind.kubernetes': 'Kubernetes',
  'clusters.empty.title': 'No clusters yet',
  'clusters.empty.description':
    'Add the Kubernetes clusters your kubeconfig reaches. Homeport installs nothing on them.',
  'clusters.column.name': 'Name',
  'clusters.column.status': 'Status',
  'clusters.column.server': 'API server',
  'clusters.column.auth': 'Signs in with',
  'clusters.palette.add': 'Add a cluster',
  'clusters.palette.manage': 'Manage clusters',

  'clusters.auth.none': 'No credentials',
  'clusters.auth.clientCertificate': 'Client certificate',
  'clusters.auth.token': 'Token',
  'clusters.auth.basic': 'User name and password',
  'clusters.auth.exec': 'Exec plugin ({command})',
  'clusters.auth.oidc': 'OpenID Connect',

  'clusters.addPage.description':
    'Each context you pick becomes a cluster. Its credentials are stored encrypted, never in clear on disk.',
  'clusters.addPage.fromMachine': 'From this machine’s kubeconfig',
  'clusters.addPage.paste': 'Paste a kubeconfig',
  'clusters.paste.description':
    'The kubeconfig is read in memory and never saved as a file. Files it refers to must be full paths.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': 'Read contexts',

  'clusters.import.readFrom': 'Read from {paths}',
  'clusters.import.noContexts': 'This kubeconfig has no contexts',
  'clusters.import.pick': 'Import {context}',
  'clusters.import.name': 'Name of the cluster made from {context}',
  'clusters.import.current': 'current',
  'clusters.import.added': 'Already added as {name}.',
  'clusters.import.refresh': 'Refresh it',
  'clusters.import.submit':
    '{count, plural, =0 {Import} one {Import # cluster} other {Import # clusters}}',
  'clusters.import.done': '{count, plural, one {Added # cluster} other {Added # clusters}}',
  'clusters.import.problem.empty': 'Give every cluster a name.',
  'clusters.import.problem.duplicate': 'Two clusters would be named “{name}”.',
  'clusters.import.problem.taken': 'A cluster named “{name}” already exists.',

  'clusters.detail.enabled': 'Enabled',
  'clusters.detail.reconnect': 'Reconnect',
  'clusters.detail.name': 'Name',
  'clusters.detail.rename': 'Rename',
  'clusters.detail.renamed': 'Cluster renamed',
  'clusters.connection.title': 'Connection',
  'clusters.connection.server': 'API server',
  'clusters.connection.version': 'Version',
  'clusters.connection.platform': 'Platform',
  'clusters.connection.namespace': 'Default namespace',
  'clusters.connection.auth': 'Signs in with',
  'clusters.connection.tlsServerName': 'Certificate name',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure':
    'The server’s certificate isn’t checked (insecure-skip-tls-verify)',
  'clusters.connection.proxy': 'Proxy',
  'clusters.connection.source': 'Imported from',
  'clusters.source.machine': 'Context {context} of this machine’s kubeconfig',
  'clusters.source.pasted': 'Context {context} of a pasted kubeconfig',

  'clusters.refresh.title': 'Refresh from kubeconfig',
  'clusters.refresh.machine':
    'Reads context {context} again from this machine’s kubeconfig, for when the cluster was recreated or its credentials renewed.',
  'clusters.refresh.pasted':
    'Paste the kubeconfig again to read context {context} afresh, for when the cluster was recreated or its credentials renewed.',
  'clusters.refresh.submit': 'Refresh',
  'clusters.refresh.done': 'Cluster refreshed from its kubeconfig',

  'clusters.remove.title': 'Remove cluster',
  'clusters.remove.description':
    'Homeport forgets the cluster and its stored credentials. Nothing changes on the cluster or in your kubeconfig.',
  'clusters.remove.confirmTitle': 'Remove {name}?',
  'clusters.remove.confirmDescription': 'You can add it again from its kubeconfig.',
} as const;
