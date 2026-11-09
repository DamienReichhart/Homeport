/** Simplified Chinese text of the `registries` namespace. */
import type en from '../en/registries';
import type { Translation } from '../types';

export default {
  'registries.title': '镜像仓库',
  'registries.description': '容器镜像仓库的登录信息，用于在每个主机上拉取私有镜像、推送和构建。',
  'registries.add': '添加登录',
  'registries.noKeychain.title': '没有系统钥匙串',
  'registries.noKeychain.description':
    '镜像仓库的密码和令牌使用保存在系统钥匙串中的密钥加密，但当前没有可用的钥匙串。仍可匿名拉取镜像。',
  'registries.empty': '暂无登录信息：镜像以匿名方式拉取。',
  'registries.editOne': '编辑 {registry} 的登录',
  'registries.removeOne': '删除 {registry} 的登录',
  'registries.remove.title': '删除 {registry} 的登录？',
  'registries.remove.description': '在重新添加登录之前，将以匿名方式从 {registry} 拉取镜像。',

  'registries.saved': '已保存 {registry} 的登录',
  'registries.removed': '已删除 {registry} 的登录',

  'registries.dialog.editTitle': '编辑 {registry} 的登录',
  'registries.dialog.addTitle': '添加镜像仓库登录',
  'registries.dialog.description':
    '用于在每个主机上拉取、推送和构建。密码或令牌会在本机加密，且不会再次显示。',
  'registries.dialog.address': '镜像仓库',
  'registries.dialog.username': '用户名',
  'registries.dialog.secret': '密码或访问令牌',
  'registries.dialog.keepSecret': '留空以保留已保存的值',
  'registries.dialog.accepted': '{registry} 接受了此登录。',
  'registries.dialog.checking': '正在检查…',
  'registries.dialog.check': '检查登录',

  'registries.missing.address': '请输入镜像仓库的地址。',
  'registries.missing.username': '请输入用户名。',
  'registries.missing.secret': '请输入密码或访问令牌。',

  'registries.login.as': '在 {registry} 上以 {username} 身份',
  'registries.login.anonymously': '匿名',
  'registries.login.pulledAs': '已在 {registry} 上以 {username} 身份拉取',
  'registries.login.pulledAnonymously': '已匿名拉取',
  'registries.login.pushedAs': '已在 {registry} 上以 {username} 身份推送',
  'registries.login.pushedAnonymously': '已匿名推送',

  'registries.noLogin.title': '没有 {registry} 的登录',
  'registries.noLogin.description':
    '大多数镜像仓库拒绝匿名推送。请先<link>在“设置 → 镜像仓库”中添加登录</link>。',
} satisfies Translation<typeof en>;
