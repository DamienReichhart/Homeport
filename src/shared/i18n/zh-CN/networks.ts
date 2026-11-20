/** Simplified Chinese text of the `networks` namespace. */
import type en from '../en/networks';
import type { Translation } from '../types';

export default {
  'networks.nav': '网络',
  'networks.title': '网络',
  'networks.summary': '{count, plural, other {# 个网络}}',
  'networks.cleanUp': '清理',
  'networks.create': '创建网络',
  'networks.search': '搜索网络',
  'networks.empty': '没有网络',
  'networks.empty.description': '同一网络中的容器可以通过名称互相访问。',

  'networks.column.name': '名称',
  'networks.column.stack': '堆栈',
  'networks.column.driver': '驱动',
  'networks.column.subnet': '子网',
  'networks.column.containers': '容器',
  'networks.builtIn': '内置',
  'networks.internal': '内部',
  'networks.moreActions': '{name} 的更多操作',
  'networks.action.inspect': '检查',

  'networks.remove.title': '从 {host} 删除网络 {name}？',
  'networks.remove.attached': '它仍连接着 {count, plural, other {# 个容器}}；请先断开它们。',
  'networks.remove.unused': '没有容器使用它。按名称引用它的容器将无法启动。',
  'networks.prune.title': '删除 {host} 上未使用的网络？',
  'networks.prune.description': '所有未连接任何容器的自定义网络都会被删除。',

  'networks.connected': '已连接',
  'networks.disconnected': '已断开连接',
  'networks.removed': '已删除 {name}',
  'networks.pruned': '已删除 {count, plural, other {# 个网络}}',

  'networks.createDialog.title': '创建网络',
  'networks.createDialog.name': '名称',
  'networks.createDialog.driver': '驱动',
  'networks.createDialog.subnet': '子网',
  'networks.createDialog.gateway': '网关',
  'networks.createDialog.automatic': '自动',
  'networks.createDialog.internal': '内部——无法访问外部网络',
  'networks.createDialog.attachable': '可连接',

  'networks.dialog.containers': '容器',
  'networks.dialog.raw': '原始 JSON',
  'networks.dialog.noContainer': '没有连接任何容器。',
  'networks.dialog.disconnect': '断开连接',
  'networks.dialog.containerToConnect': '要连接的容器',
  'networks.dialog.connectPlaceholder': '连接容器…',
  'networks.dialog.connect': '连接',
} satisfies Translation<typeof en>;
