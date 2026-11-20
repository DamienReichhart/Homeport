import type { Translation } from '../../types';
import type en from '../en/site';

/** The layout, the header and the footer, in Simplified Chinese. */
export default {
  // The layout and the facts every page repeats.
  'site.tagline': '一个应用，管理所有容器引擎。',
  'site.description':
    'Homeport 是一款免费、开源的桌面应用，用于 Docker、Podman、containerd、Incus、Kubernetes 和 Nomad。它运行在你的电脑上，通过 SSH、TLS 或你的 kubeconfig 连接服务器，服务器上无需安装任何东西。支持 Windows、macOS 和 Linux。',
  'site.ogImageAlt': 'Homeport：一个应用，管理所有容器引擎。',
  'site.skipToContent': '跳到正文',

  // The header, the language menu and the theme switch.
  'header.home': 'Homeport 首页',
  'header.logo': 'Homeport',
  'header.nav': '主导航',
  'header.engines': '引擎',
  'header.docs': '文档',
  'header.changelog': '更新日志',
  'header.download': '下载',
  'header.github': 'GitHub 上的 Homeport',
  'header.githubShort': 'GitHub',
  'header.menu': '菜单',
  'language.label': '语言',
  'theme.toLight': '切换到浅色主题',
  'theme.toDark': '切换到深色主题',

  // The footer.
  'footer.product': '产品',
  'footer.engines': '引擎',
  'footer.download': '下载',
  'footer.changelog': '更新日志',
  'footer.releases': '所有版本',
  'footer.documentation': '文档',
  'footer.introduction': '简介',
  'footer.installation': '安装',
  'footer.hosts': '连接主机',
  'footer.troubleshooting': '故障排除',
  'footer.project': '项目',
  'footer.source': '源代码',
  'footer.issue': '报告问题',
  'footer.contributing': '参与贡献',
  'footer.security': '安全',
  'footer.legal': '法律信息',
  'footer.legalNotice': '法律声明',
  'footer.privacy': '隐私',
  'footer.terms': '条款',
  'footer.cookies': 'Cookie',
  'footer.licence': '© {year} Damien Reichhart。Homeport 依据 {licence} 开源。',
  'footer.licenceName': 'GNU AGPL 第 3 版',
  'footer.trademark':
    'Docker 是 Docker, Inc. 的商标。Homeport 是一个独立项目，与 Docker, Inc. 无关联，也未获其认可。',
} satisfies Translation<typeof en>;
