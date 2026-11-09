/** Simplified Chinese text of the `clusterevents` namespace. */
import type en from '../en/clusterevents';
import type { Translation } from '../types';

export default {
  'clusterevents.nav': '事件',
  'clusterevents.title': '事件',
  'clusterevents.summary': '{count, plural, other {# 个事件}}',
  'clusterevents.search': '按原因、对象或消息搜索',
  'clusterevents.empty': '没有事件。Kubernetes 只保留约一小时内的事件。',
  'clusterevents.filter': '类型',
  'clusterevents.all': '全部',
  'clusterevents.warnings': '警告',
  'clusterevents.normal': '正常',
  'clusterevents.column.type': '类型',
  'clusterevents.column.reason': '原因',
  'clusterevents.column.object': '对象',
  'clusterevents.column.namespace': 'Namespace',
  'clusterevents.column.message': '消息',
  'clusterevents.column.count': '次数',
  'clusterevents.column.lastSeen': '最后出现',
} satisfies Translation<typeof en>;
