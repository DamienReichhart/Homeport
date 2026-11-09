/** English text of the `clusters` namespace; every key starts with `clusters.`. */
export default {
  'clusters.title': 'Clusters',
  'clusters.summary': '{count, plural, one {# cluster} other {# clusters}}',
  'clusters.add': 'Add a cluster',
  'clusters.notFound': 'This cluster no longer exists.',
  'clusters.empty.title': 'No clusters yet',
  'clusters.empty.description':
    'Add the Kubernetes clusters your kubeconfig reaches. Homeport installs nothing on them.',
  'clusters.column.name': 'Name',
  'clusters.column.status': 'Status',
  'clusters.column.kind': 'Kind',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'Signs in with',
  'clusters.palette.add': 'Add a cluster',
  'clusters.palette.manage': 'Manage clusters',

  'clusters.auth.none': 'No credentials',
  'clusters.auth.clientCertificate': 'Client certificate',
  'clusters.auth.token': 'Token',
  'clusters.auth.basic': 'User name and password',
  'clusters.auth.exec': 'Exec plugin ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'ACL token',

  'clusters.addPage.description':
    'Each Kubernetes context you pick, or each Nomad agent you name, becomes a cluster. Its credentials are stored encrypted, never in clear on disk.',
  'clusters.addPage.fromMachine': 'From this machine’s kubeconfig',
  'clusters.addPage.paste': 'Paste a kubeconfig',
  'clusters.addPage.nomad': 'Nomad',
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
  'clusters.connection.address': 'Address',
  'clusters.connection.region': 'Region',
  'clusters.connection.customCa': 'Its own CA certificate',
  'clusters.connection.clientCertificate': 'Client certificate',
  'clusters.connection.checkedAs': 'Certificate checked as {name}',
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

  'clusters.nomad.description':
    'Homeport talks to the HTTP API of a Nomad agent, server or client, from this computer. Nothing is installed on the cluster, and no nomad CLI is needed here.',
  'clusters.nomad.name': 'Name',
  'clusters.nomad.address': 'Address',
  'clusters.nomad.addressHelp':
    'The agent’s HTTP API, as NOMAD_ADDR gives it to the nomad CLI. Port 4646 when none is given.',
  'clusters.nomad.region': 'Region',
  'clusters.nomad.regionHelp': 'Leave it empty to use the agent’s own region.',
  'clusters.nomad.token': 'ACL token',
  'clusters.nomad.tokenHelp':
    'The token’s secret ID, when the cluster has ACLs enabled. Homeport’s docs give a policy for it.',
  'clusters.nomad.tls': 'TLS settings',
  'clusters.nomad.tlsHelp':
    'Only for an agent that serves HTTPS with its own CA, or asks for a client certificate. Paste the PEM files’ contents.',
  'clusters.nomad.ca': 'CA certificate',
  'clusters.nomad.caHelp':
    'The CA that signed the agent’s certificate, as NOMAD_CACERT names it. This computer’s CAs are trusted without one.',
  'clusters.nomad.clientCertificate': 'Client certificate',
  'clusters.nomad.clientKey': 'Client key',
  'clusters.nomad.tlsServerName': 'TLS server name',
  'clusters.nomad.tlsServerNameHelp':
    'The name the agent’s certificate is checked against, when not the address’s, as NOMAD_TLS_SERVER_NAME sets it.',
  'clusters.nomad.keep': 'Saved; leave empty to keep it',
  'clusters.nomad.willBeRemoved': 'Will be removed when you save',
  'clusters.nomad.remove': 'Remove',
  'clusters.nomad.keepIt': 'Keep it',
  'clusters.nomad.test': 'Test connection',
  'clusters.nomad.testing': 'Testing…',
  'clusters.nomad.testedVersion': 'Connected to Nomad {version}, region {region}.',
  'clusters.nomad.testedNoVersion':
    'Connected to region {region}. The token can’t read the agent, so its version isn’t known.',
  'clusters.nomad.add': 'Add cluster',
  'clusters.nomad.added': 'Added {name}',
  'clusters.nomad.settings': 'Settings',
  'clusters.nomad.settingsDescription':
    'Saving reconnects. The token and TLS key are never shown again: leave them empty to keep them.',
  'clusters.nomad.save': 'Save changes',
  'clusters.nomad.saved': 'Cluster saved',

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
