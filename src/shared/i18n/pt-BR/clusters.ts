/** Brazilian Portuguese text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': 'Clusters',
  'clusters.summary': '{count, plural, one {# cluster} other {# clusters}}',
  'clusters.add': 'Adicionar um cluster',
  'clusters.notFound': 'Este cluster não existe mais.',
  'clusters.empty.title': 'Nenhum cluster ainda',
  'clusters.empty.description':
    'Adicione os clusters Kubernetes que o seu kubeconfig alcança. O Homeport não instala nada neles.',
  'clusters.column.name': 'Nome',
  'clusters.column.status': 'Status',
  'clusters.column.kind': 'Tipo',
  'clusters.column.server': 'API',
  'clusters.column.auth': 'Autentica com',
  'clusters.palette.add': 'Adicionar um cluster',
  'clusters.palette.manage': 'Gerenciar clusters',

  'clusters.auth.none': 'Sem credenciais',
  'clusters.auth.clientCertificate': 'Certificado de cliente',
  'clusters.auth.token': 'Token',
  'clusters.auth.basic': 'Nome de usuário e senha',
  'clusters.auth.exec': 'Plugin exec ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'Token ACL',

  'clusters.addPage.description':
    'Cada contexto Kubernetes que você escolher, ou cada agente Nomad que você indicar, vira um cluster. As credenciais dele são armazenadas criptografadas, nunca em texto puro no disco.',
  'clusters.addPage.fromMachine': 'Do kubeconfig desta máquina',
  'clusters.addPage.paste': 'Colar um kubeconfig',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'O kubeconfig é lido na memória e nunca é salvo como arquivo. Os arquivos aos quais ele se refere precisam ter caminhos completos.',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': 'Ler contextos',

  'clusters.import.readFrom': 'Lido de {paths}',
  'clusters.import.noContexts': 'Este kubeconfig não tem contextos',
  'clusters.import.pick': 'Importar {context}',
  'clusters.import.name': 'Nome do cluster criado a partir de {context}',
  'clusters.import.current': 'atual',
  'clusters.import.added': 'Já adicionado como {name}.',
  'clusters.import.refresh': 'Atualizá-lo',
  'clusters.import.submit':
    '{count, plural, =0 {Importar} one {Importar # cluster} other {Importar # clusters}}',
  'clusters.import.done':
    '{count, plural, one {# cluster adicionado} other {# clusters adicionados}}',
  'clusters.import.problem.empty': 'Dê um nome a cada cluster.',
  'clusters.import.problem.duplicate': 'Dois clusters ficariam com o nome “{name}”.',
  'clusters.import.problem.taken': 'Já existe um cluster chamado “{name}”.',

  'clusters.detail.enabled': 'Ativado',
  'clusters.detail.reconnect': 'Reconectar',
  'clusters.detail.name': 'Nome',
  'clusters.detail.rename': 'Renomear',
  'clusters.detail.renamed': 'Cluster renomeado',
  'clusters.connection.title': 'Conexão',
  'clusters.connection.server': 'Servidor da API',
  'clusters.connection.address': 'Endereço',
  'clusters.connection.region': 'Região',
  'clusters.connection.customCa': 'Certificado de CA próprio',
  'clusters.connection.clientCertificate': 'Certificado de cliente',
  'clusters.connection.checkedAs': 'Certificado verificado como {name}',
  'clusters.connection.version': 'Versão',
  'clusters.connection.platform': 'Plataforma',
  'clusters.connection.namespace': 'Namespace padrão',
  'clusters.connection.auth': 'Autentica com',
  'clusters.connection.tlsServerName': 'Nome do certificado',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure':
    'O certificado do servidor não é verificado (insecure-skip-tls-verify)',
  'clusters.connection.proxy': 'Proxy',
  'clusters.connection.source': 'Importado de',
  'clusters.source.machine': 'Contexto {context} do kubeconfig desta máquina',
  'clusters.source.pasted': 'Contexto {context} de um kubeconfig colado',

  'clusters.nomad.description':
    'O Homeport se comunica com a API HTTP de um agente Nomad, servidor ou cliente, a partir deste computador. Nada é instalado no cluster, e nenhuma CLI nomad é necessária aqui.',
  'clusters.nomad.name': 'Nome',
  'clusters.nomad.address': 'Endereço',
  'clusters.nomad.addressHelp':
    'A API HTTP do agente, como NOMAD_ADDR a passa para a CLI nomad. Porta 4646 quando nenhuma é informada.',
  'clusters.nomad.region': 'Região',
  'clusters.nomad.regionHelp': 'Deixe em branco para usar a região do próprio agente.',
  'clusters.nomad.token': 'Token ACL',
  'clusters.nomad.tokenHelp':
    'O ID secreto do token, quando o cluster tem ACLs ativadas. A documentação do Homeport fornece uma política para ele.',
  'clusters.nomad.tls': 'Configurações de TLS',
  'clusters.nomad.tlsHelp':
    'Somente para um agente que serve HTTPS com a própria CA ou que pede um certificado de cliente. Cole o conteúdo dos arquivos PEM.',
  'clusters.nomad.ca': 'Certificado de CA',
  'clusters.nomad.caHelp':
    'A CA que assinou o certificado do agente, como NOMAD_CACERT a indica. As CAs deste computador são confiáveis sem ela.',
  'clusters.nomad.clientCertificate': 'Certificado de cliente',
  'clusters.nomad.clientKey': 'Chave de cliente',
  'clusters.nomad.tlsServerName': 'Nome do servidor TLS',
  'clusters.nomad.tlsServerNameHelp':
    'O nome com o qual o certificado do agente é verificado, quando não é o do endereço, como NOMAD_TLS_SERVER_NAME o define.',
  'clusters.nomad.keep': 'Salvo; deixe em branco para mantê-lo',
  'clusters.nomad.willBeRemoved': 'Será removido quando você salvar',
  'clusters.nomad.remove': 'Remover',
  'clusters.nomad.keepIt': 'Manter',
  'clusters.nomad.test': 'Testar conexão',
  'clusters.nomad.testing': 'Testando…',
  'clusters.nomad.testedVersion': 'Conectado ao Nomad {version}, região {region}.',
  'clusters.nomad.testedNoVersion':
    'Conectado à região {region}. O token não consegue ler o agente, então a versão dele não é conhecida.',
  'clusters.nomad.add': 'Adicionar cluster',
  'clusters.nomad.added': '{name} adicionado',
  'clusters.nomad.settings': 'Configurações',
  'clusters.nomad.settingsDescription':
    'Salvar reconecta. O token e a chave TLS nunca mais são mostrados: deixe-os em branco para mantê-los.',
  'clusters.nomad.save': 'Salvar alterações',
  'clusters.nomad.saved': 'Cluster salvo',

  'clusters.refresh.title': 'Atualizar a partir do kubeconfig',
  'clusters.refresh.machine':
    'Lê de novo o contexto {context} do kubeconfig desta máquina, para quando o cluster foi recriado ou as credenciais dele foram renovadas.',
  'clusters.refresh.pasted':
    'Cole o kubeconfig de novo para ler o contexto {context} outra vez, para quando o cluster foi recriado ou as credenciais dele foram renovadas.',
  'clusters.refresh.submit': 'Atualizar',
  'clusters.refresh.done': 'Cluster atualizado a partir do kubeconfig',

  'clusters.remove.title': 'Remover cluster',
  'clusters.remove.description':
    'O Homeport esquece o cluster e as credenciais armazenadas dele. Nada muda no cluster nem no seu kubeconfig.',
  'clusters.remove.confirmTitle': 'Remover {name}?',
  'clusters.remove.confirmDescription': 'Você pode adicioná-lo de novo a partir do kubeconfig.',
} satisfies Translation<typeof en>;
