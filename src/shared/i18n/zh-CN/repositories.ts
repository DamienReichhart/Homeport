/** Simplified Chinese text of the `repositories` namespace. */
import type en from '../en/repositories';
import type { Translation } from '../types';

export default {
  'repositories.title': '仓库',
  'repositories.description': '用于在各个主机上部署同步堆栈的 Git 仓库。Homeport 只读取它们。',
  'repositories.add': '添加仓库',
  'repositories.empty': '暂无仓库。添加一个仓库即可创建同步堆栈。',
  'repositories.manageOne': '管理对 {repository} 的访问',
  'repositories.removeOne': '删除 {repository}',
  'repositories.remove.title': '删除 {repository}？',
  'repositories.remove.description':
    'Homeport 会忘记它的部署密钥或令牌，并删除本机上的克隆。仓库本身不受影响。请同时在你的代码托管平台上删除该部署密钥。',
  'repositories.noKeychain':
    '部署密钥和令牌使用保存在系统钥匙串中的密钥加密，但当前没有可用的钥匙串，因此只能添加公开仓库。',
  'repositories.public': '公开仓库：Homeport 无需凭据即可读取。',

  'repositories.added': '已添加 {repository}',
  'repositories.removed': '已删除 {repository}',
  'repositories.keyRotated': '已生成新的部署密钥。请将它添加到仓库，替换旧密钥。',
  'repositories.tokenReplaced': '已替换令牌',
  'repositories.hostKeyReset': '下次连接时，Homeport 将信任服务器出示的密钥。',

  'repositories.auth.withKey': '使用部署密钥，位于 {host}',
  'repositories.auth.withToken': '以 {username} 身份使用令牌，位于 {host}',
  'repositories.auth.public': '公开，位于 {host}',

  'repositories.dialog.addTitle': '添加仓库',
  'repositories.dialog.description':
    'Homeport 会在本机克隆它，以便从中部署同步堆栈。它从不提交或推送。',
  'repositories.dialog.url': '仓库 URL',
  'repositories.dialog.auth': '访问方式',
  'repositories.dialog.authKey': '部署密钥',
  'repositories.dialog.authToken': '令牌',
  'repositories.dialog.authNone': '公开',
  'repositories.dialog.authKeyHelp':
    'Homeport 会生成一个仅供此仓库使用的密钥。你以只读权限将它添加到仓库；绝不会使用你自己的 SSH 密钥。',
  'repositories.dialog.authTokenHelp':
    '可读取仓库内容的访问令牌，通过 HTTPS 发送。在本机加密，且不会再次显示。',
  'repositories.dialog.authNoneHelp': '适用于通过 HTTPS 访问的公开仓库。不保存任何内容。',
  'repositories.dialog.username': '用户名',
  'repositories.dialog.token': '访问令牌',
  'repositories.dialog.add': '添加',
  'repositories.dialog.addWithKey': '添加并生成部署密钥',

  'repositories.missing.url': '请输入仓库的 URL。',
  'repositories.missing.validUrl':
    '请输入 SSH 或 HTTPS URL，例如 git@github.com:owner/repo.git 或 https://gitlab.com/owner/repo.git。',
  'repositories.missing.auth': '请选择 Homeport 访问此仓库的方式。',
  'repositories.missing.username': '请输入令牌所属的用户名。',
  'repositories.missing.token': '请输入访问令牌。',

  'repositories.deployKey.title': '部署密钥',
  'repositories.deployKey.github':
    'GitHub：仓库的 <strong>Settings → Deploy keys → Add deploy key</strong>。',
  'repositories.deployKey.gitlab':
    'GitLab：项目的 <strong>Settings → Repository → Deploy keys</strong>。',
  'repositories.deployKey.gitea':
    'Gitea 和 Forgejo：仓库的 <strong>Settings → Deploy keys → Add deploy key</strong>。',
  'repositories.deployKey.readOnly': '不要开启写入权限：Homeport 只读取。然后在下方测试访问。',
  'repositories.deployKey.fingerprint': '指纹：<mono>{fingerprint}</mono>',
  'repositories.rotate.button': '新密钥',
  'repositories.rotate.title': '生成新的部署密钥？',
  'repositories.rotate.description':
    '当前密钥将对 Homeport 失效。请将新密钥添加到仓库，并在那里删除旧密钥。',
  'repositories.rotate.confirm': '生成新密钥',

  'repositories.token.replace': '替换令牌',
  'repositories.token.save': '保存令牌',

  'repositories.hostKey.trusted': '{host} 的受信任密钥：<mono>{fingerprint}</mono>',
  'repositories.hostKey.reset': '重置',
  'repositories.hostKey.resetTitle': '重置 {host} 的受信任密钥？',
  'repositories.hostKey.resetDescription':
    '仅当 {host} 更换了 SSH 密钥，且你已在其网站上核对过新指纹时才这样做。Homeport 将信任它下次出示的任何密钥。',

  'repositories.test.button': '测试访问',
  'repositories.test.testing': '正在测试…',
  'repositories.test.success':
    'Homeport 可以读取它：{branches, plural, other {# 个分支}}和 {tags, plural, other {# 个标签}}。',
} satisfies Translation<typeof en>;
