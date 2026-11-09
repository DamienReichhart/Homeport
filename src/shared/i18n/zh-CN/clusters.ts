/** Simplified Chinese text of the `clusters` namespace. */
import type en from '../en/clusters';
import type { Translation } from '../types';

export default {
  'clusters.title': '集群',
  'clusters.summary': '{count, plural, other {# 个集群}}',
  'clusters.add': '添加集群',
  'clusters.notFound': '此集群已不存在。',
  'clusters.empty.title': '暂无集群',
  'clusters.empty.description':
    '添加你的 kubeconfig 可访问的 Kubernetes 集群。Homeport 不会在集群上安装任何内容。',
  'clusters.column.name': '名称',
  'clusters.column.status': '状态',
  'clusters.column.kind': '类型',
  'clusters.column.server': 'API',
  'clusters.column.auth': '登录方式',
  'clusters.palette.add': '添加集群',
  'clusters.palette.manage': '管理集群',

  'clusters.auth.none': '无凭据',
  'clusters.auth.clientCertificate': '客户端证书',
  'clusters.auth.token': '令牌',
  'clusters.auth.basic': '用户名和密码',
  'clusters.auth.exec': 'Exec 插件 ({command})',
  'clusters.auth.oidc': 'OpenID Connect',
  'clusters.auth.aclToken': 'ACL 令牌',

  'clusters.addPage.description':
    '你选择的每个 Kubernetes 上下文或你指定的每个 Nomad agent 都会成为一个集群。其凭据加密保存，绝不会以明文写入磁盘。',
  'clusters.addPage.fromMachine': '来自本机的 kubeconfig',
  'clusters.addPage.paste': '粘贴 kubeconfig',
  'clusters.addPage.nomad': 'Nomad',
  'clusters.paste.description':
    'kubeconfig 只在内存中读取，绝不会保存为文件。它引用的文件必须使用完整路径。',
  'clusters.paste.label': 'Kubeconfig',
  'clusters.paste.read': '读取上下文',

  'clusters.import.readFrom': '读取自 {paths}',
  'clusters.import.noContexts': '此 kubeconfig 没有上下文',
  'clusters.import.pick': '导入 {context}',
  'clusters.import.name': '由 {context} 创建的集群的名称',
  'clusters.import.current': '当前',
  'clusters.import.added': '已添加为 {name}。',
  'clusters.import.refresh': '刷新它',
  'clusters.import.submit': '{count, plural, =0 {导入} other {导入 # 个集群}}',
  'clusters.import.done': '{count, plural, other {已添加 # 个集群}}',
  'clusters.import.problem.empty': '请为每个集群命名。',
  'clusters.import.problem.duplicate': '有两个集群将被命名为“{name}”。',
  'clusters.import.problem.taken': '名为“{name}”的集群已存在。',

  'clusters.detail.enabled': '已启用',
  'clusters.detail.reconnect': '重新连接',
  'clusters.detail.name': '名称',
  'clusters.detail.rename': '重命名',
  'clusters.detail.renamed': '集群已重命名',
  'clusters.connection.title': '连接',
  'clusters.connection.server': 'API 服务器',
  'clusters.connection.address': '地址',
  'clusters.connection.region': '区域',
  'clusters.connection.customCa': '自有 CA 证书',
  'clusters.connection.clientCertificate': '客户端证书',
  'clusters.connection.checkedAs': '证书按 {name} 校验',
  'clusters.connection.version': '版本',
  'clusters.connection.platform': '平台',
  'clusters.connection.namespace': '默认 Namespace',
  'clusters.connection.auth': '登录方式',
  'clusters.connection.tlsServerName': '证书名称',
  'clusters.connection.tls': 'TLS',
  'clusters.connection.insecure': '不校验服务器证书 (insecure-skip-tls-verify)',
  'clusters.connection.proxy': '代理',
  'clusters.connection.source': '导入自',
  'clusters.source.machine': '本机 kubeconfig 中的上下文 {context}',
  'clusters.source.pasted': '粘贴的 kubeconfig 中的上下文 {context}',

  'clusters.nomad.description':
    'Homeport 从本机与 Nomad agent（server 或 client）的 HTTP API 通信。集群上不会安装任何内容，本机也不需要 nomad CLI。',
  'clusters.nomad.name': '名称',
  'clusters.nomad.address': '地址',
  'clusters.nomad.addressHelp':
    'agent 的 HTTP API，与 NOMAD_ADDR 提供给 nomad CLI 的相同。未指定端口时使用 4646。',
  'clusters.nomad.region': '区域',
  'clusters.nomad.regionHelp': '留空则使用 agent 自身的区域。',
  'clusters.nomad.token': 'ACL 令牌',
  'clusters.nomad.tokenHelp':
    '集群启用 ACL 时所用令牌的 Secret ID。Homeport 的文档提供了适用的策略。',
  'clusters.nomad.tls': 'TLS 设置',
  'clusters.nomad.tlsHelp':
    '仅适用于使用自有 CA 提供 HTTPS 或要求客户端证书的 agent。请粘贴 PEM 文件的内容。',
  'clusters.nomad.ca': 'CA 证书',
  'clusters.nomad.caHelp':
    '签署 agent 证书的 CA，与 NOMAD_CACERT 指定的相同。不填时信任本机的 CA。',
  'clusters.nomad.clientCertificate': '客户端证书',
  'clusters.nomad.clientKey': '客户端密钥',
  'clusters.nomad.tlsServerName': 'TLS 服务器名称',
  'clusters.nomad.tlsServerNameHelp':
    '当 agent 证书的名称与地址不同时，用于校验证书的名称，与 NOMAD_TLS_SERVER_NAME 的设置相同。',
  'clusters.nomad.keep': '已保存；留空以保留',
  'clusters.nomad.willBeRemoved': '保存时将被删除',
  'clusters.nomad.remove': '删除',
  'clusters.nomad.keepIt': '保留',
  'clusters.nomad.test': '测试连接',
  'clusters.nomad.testing': '正在测试…',
  'clusters.nomad.testedVersion': '已连接到 Nomad {version}，区域 {region}。',
  'clusters.nomad.testedNoVersion': '已连接到区域 {region}。该令牌无法读取 agent，因此版本未知。',
  'clusters.nomad.add': '添加集群',
  'clusters.nomad.added': '已添加 {name}',
  'clusters.nomad.settings': '设置',
  'clusters.nomad.settingsDescription':
    '保存后会重新连接。令牌和 TLS 密钥不会再次显示：留空即可保留。',
  'clusters.nomad.save': '保存更改',
  'clusters.nomad.saved': '集群已保存',

  'clusters.refresh.title': '从 kubeconfig 刷新',
  'clusters.refresh.machine':
    '从本机的 kubeconfig 重新读取上下文 {context}，适用于集群已重建或其凭据已更新的情况。',
  'clusters.refresh.pasted':
    '再次粘贴 kubeconfig 以重新读取上下文 {context}，适用于集群已重建或其凭据已更新的情况。',
  'clusters.refresh.submit': '刷新',
  'clusters.refresh.done': '已从 kubeconfig 刷新集群',

  'clusters.remove.title': '删除集群',
  'clusters.remove.description':
    'Homeport 会忘记该集群及其保存的凭据。集群和你的 kubeconfig 都不会有任何更改。',
  'clusters.remove.confirmTitle': '删除 {name}？',
  'clusters.remove.confirmDescription': '你可以从其 kubeconfig 重新添加它。',
} satisfies Translation<typeof en>;
