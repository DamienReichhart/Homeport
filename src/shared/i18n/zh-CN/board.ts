/** Simplified Chinese text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': '看板',
  'board.title': '看板',
  'board.summary':
    '{count, plural, other {# 个容器}}中有 {running} 个正在 {hosts, plural, other {# 个主机}}上运行',
  'board.column.host': '主机',
  'board.column.stack': '堆栈',
  'board.sortBy': '按 {column} 排序',
  'board.filter.hosts': '主机',
  'board.filter.allHosts': '所有主机',
  'board.filter.someHosts': '{count, plural, other {# 个主机}}',
  'board.filter.state': '状态',
  'board.filter.stack': '堆栈',
  'board.filter.anyStack': '任意堆栈',
  'board.filter.clear': '清除筛选',
  'board.state.all': '任意状态',
  'board.state.running': '运行中',
  'board.state.stopped': '已停止',
  'board.state.unhealthy': '不健康',
  'board.search': '搜索所有主机',
  'board.skipped': '{name}（{status}）未显示。',
  'board.openHost': '打开主机',
  'board.listFailed': '无法列出 {name} 的容器。',
  'board.noHosts.title': '没有已连接的主机',
  'board.noHosts.description': '看板列出所有已连接主机的容器。',
  'board.empty.title': '暂无容器',
  'board.empty.description': '你已连接的主机上都没有运行容器。',
  'board.noMatch': '没有容器符合这些筛选条件。',
  'board.selection': '已选容器',
  'board.selectAll': '选择显示的所有容器',
  'board.selectOne': '选择 {host} 上的 {name}',
  'board.remove.title': '删除 {count, plural, other {# 个容器}}？',
  'board.remove.description': '它们会连同匿名卷一起删除；命名卷会保留。正在运行的容器会先停止。',
  'board.remove.onHost': '在 {host} 上：{names}',
} satisfies Translation<typeof en>;
