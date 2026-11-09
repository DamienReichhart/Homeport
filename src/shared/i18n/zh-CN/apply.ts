/** Simplified Chinese text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': '应用 YAML',
  'apply.title': '应用 YAML',
  'apply.description':
    '粘贴或打开清单，查看它们会在 {cluster} 上更改什么，然后应用，与 kubectl apply 相同。',
  'apply.open': '打开文件…',
  'apply.openTitle': '打开清单',
  'apply.openFailed': 'Homeport 无法读取该文件',
  'apply.manifests': 'Kubernetes 清单',
  'apply.editor': '要应用的清单',
  'apply.defaultNamespace': '未指定 Namespace 的文档会进入 {namespace}。',
  'apply.defaultNamespaceCluster': '未指定 Namespace 的文档会进入集群的默认 Namespace。',
} satisfies Translation<typeof en>;
