/** Simplified Chinese text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': '主菜单',
  'app.nav.docs': '文档',
  'app.nav.github': 'GitHub',
  'app.nav.opensInBrowser': '{label}（在浏览器中打开）',
  'app.keychainMissing':
    '没有可用的系统钥匙串，因此无法保存 SSH 密钥、TLS 证书和堆栈环境变量。本地、TCP 和 ssh-agent 主机仍可使用。',
  'app.crash.title': '出现问题',
  'app.crash.description':
    'Homeport 在此页面上遇到了意外问题。重新加载通常可以解决；如果问题反复出现，请从“详细信息”中复制报告并提交 issue。',
  'app.crash.reload': '重新加载',
  'app.crash.backToOverview': '返回概览',
  'app.shortcuts.title': '键盘快捷键',
  'app.shortcuts.description': '此处可用的快捷键。在输入框中输入时，N 和 / 等按键不会触发。',
  'app.shortcuts.general': '常规',
  'app.shortcuts.navigation': '转到',
  'app.shortcuts.hosts': '主机',
  'app.shortcuts.help': '显示键盘快捷键',
  'app.shortcuts.settings': '打开设置',
  'app.shortcuts.goTo': '转到 {page}',
  'app.shortcuts.previousHost': '上一个主机',
  'app.shortcuts.nextHost': '下一个主机',
  'app.shortcuts.refresh': '刷新此主机的列表',
  'app.palette.title': '命令面板',
  'app.palette.description': '转到页面、切换主机或运行命令。',
  'app.palette.open': '打开命令面板',
  'app.palette.placeholder': '输入页面、主机或命令…',
  'app.palette.empty': '没有匹配项。',
  'app.palette.recent': '最近使用',
  'app.palette.goTo': '转到',
  'app.palette.hosts': '主机',
  'app.palette.actions': '操作',
  'app.palette.switchTo': '切换到 {name}',
} satisfies Translation<typeof en>;
