/** Simplified Chinese text of the `portforwards` namespace. */
import type en from '../en/portforwards';
import type { Translation } from '../types';

export default {
  'portforwards.nav': '端口转发',
  'portforwards.title': '端口转发',
  'portforwards.summary': '{count, plural, other {# 个转发}}',
  'portforwards.empty.title': '没有端口转发',
  'portforwards.empty.description':
    '在 Pod 或 Service 的页面上转发端口，即可从本机访问它。Homeport 退出时转发随之结束。',
  'portforwards.column.local': '本地地址',
  'portforwards.column.target': '转发到',
  'portforwards.column.namespace': 'Namespace',
  'portforwards.column.cluster': '集群',
  'portforwards.column.connections': '连接数',
  'portforwards.forward': '转发…',
  'portforwards.forwardPort': '转发端口 {port}',
  'portforwards.open': '打开',
  'portforwards.stop': '停止',
  'portforwards.stopFailed': 'Homeport 无法停止该转发',
  'portforwards.started': '正在将 {address} 转发到 {target}:{port}',
  'portforwards.failed': '端口转发失败',
  'portforwards.ended': '端口转发已结束',
  'portforwards.dialog.title': '转发 {target}:{port}',
  'portforwards.dialog.description':
    '位于 Namespace {namespace}。转发运行期间，可通过本地端口访问。',
  'portforwards.dialog.localPort': '本地端口',
  'portforwards.dialog.anyPort': '任意空闲端口',
  'portforwards.dialog.loopback': '只有本机可以连接：转发监听 127.0.0.1（以及 ::1）。',
  'portforwards.dialog.submit': '转发',
} satisfies Translation<typeof en>;
