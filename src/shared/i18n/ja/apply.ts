/** Japanese text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'YAML を適用',
  'apply.title': 'YAML を適用',
  'apply.description': 'マニフェストが {cluster} で何を変えるかを確認してから適用します。',
  'apply.open': 'ファイルを開く…',
  'apply.openTitle': 'マニフェストを開く',
  'apply.openFailed': 'Homeport はそのファイルを読み込めませんでした',
  'apply.manifests': 'Kubernetes マニフェスト',
  'apply.editor': '適用するマニフェスト',
  'apply.defaultNamespace': 'Namespace のないドキュメントは {namespace} に適用されます。',
  'apply.defaultNamespaceCluster':
    'Namespace のないドキュメントは、クラスターのデフォルトの Namespace に適用されます。',
} satisfies Translation<typeof en>;
