/** Korean text of the `apply` namespace. */
import type en from '../en/apply';
import type { Translation } from '../types';

export default {
  'apply.nav': 'YAML 적용',
  'apply.title': 'YAML 적용',
  'apply.description':
    'kubectl apply처럼, 매니페스트를 붙여 넣거나 열고 {cluster} 클러스터에서 무엇이 바뀌는지 확인한 다음 적용합니다.',
  'apply.open': '파일 열기…',
  'apply.openTitle': '매니페스트 열기',
  'apply.openFailed': 'Homeport가 해당 파일을 읽지 못했습니다',
  'apply.manifests': 'Kubernetes 매니페스트',
  'apply.editor': '적용할 매니페스트',
  'apply.defaultNamespace': 'Namespace가 없는 문서는 {namespace} Namespace에 적용됩니다.',
  'apply.defaultNamespaceCluster':
    'Namespace가 없는 문서는 클러스터의 기본 Namespace에 적용됩니다.',
} satisfies Translation<typeof en>;
