---
title: 스케일, 다시 시작, 삭제
description: Deployment와 StatefulSet을 스케일하고, 롤아웃을 다시 시작하고, CronJob을 일시 중단하거나 실행하고, 클러스터의 모든 객체를 삭제합니다. 각 작업은 대상을 밝히는 확인을 거칩니다.
---

클러스터에 대한 일상적인 변경은 페이지에서 클릭 한 번으로 할 수 있으며, `kubectl`처럼 클러스터 자체의 자격 증명으로 Kubernetes API에 전달됩니다. 목록은 watch를 통해 변경 사항을 실시간으로 따라갑니다.

무언가를 삭제하거나 교체하는 모든 작업은 먼저 확인을 요청하며, 확인 창에는 클러스터, Namespace, 종류, 이름이 표시되므로 실수로 엉뚱한 클러스터나 Namespace를 변경하는 일이 없습니다.

## 워크로드

워크로드 페이지에서는 헤더에, **워크로드** 페이지의 각 행에서는 **⋯** 메뉴에 작업이 있습니다.

| 작업          | 대상                               | 하는 일(같은 명령)                                       |
| ------------- | ---------------------------------- | -------------------------------------------------------- |
| **스케일**    | Deployment, StatefulSet            | `kubectl scale --replicas=N`                             |
| **다시 시작** | Deployment, StatefulSet, DaemonSet | `kubectl rollout restart`                                |
| **일시 중단** | CronJob                            | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'` |
| **재개**      | 일시 중단된 CronJob                | 같은 명령에 `false`                                      |
| **지금 실행** | CronJob                            | `kubectl create job --from=cronjob/…`                    |
| **삭제**      | 모두                               | `kubectl delete`(Pod와 Job도 백그라운드에서 함께 삭제)   |

### 스케일

**스케일** 버튼을 누르면 현재 실행 중인 Pod 수를 보여 주며 실행할 Pod 수를 묻습니다. `0`으로 스케일하면 다시 스케일 업할 때까지 워크로드가 중지됩니다. HorizontalPodAutoscaler가 워크로드를 관리하면 대화 상자에 그 사실이 표시됩니다. 오토스케일러가 다음 확인 때 수를 되돌리므로, 대신 오토스케일러의 최솟값과 최댓값을 바꾸세요.

### 다시 시작

**다시 시작** 버튼은 `kubectl rollout restart`와 똑같이 Pod 템플릿에 `kubectl.kubernetes.io/restartedAt` 어노테이션을 추가합니다. 워크로드는 전략에 따라 새 Pod 집합을 롤아웃하고(Deployment라면 새 ReplicaSet), 새 Pod가 준비되면 이전 Pod가 사라집니다.

### CronJob

**일시 중단** 버튼은 CronJob이 새 Job을 시작하지 못하게 하고(이미 실행 중인 Job은 계속 실행됨), **재개** 버튼은 다시 시작할 수 있게 합니다. 일시 중단된 CronJob은 **일시 중지됨** 상태로 표시됩니다. **지금 실행** 버튼은 템플릿으로 Job을 즉시 시작합니다. 이 Job은 `<cronjob>-manual-<id>`라는 이름으로, CronJob이 소유하며, kubectl과 마찬가지로 `cronjob.kubernetes.io/instantiate: manual`이 표시됩니다. CronJob의 **Job** 탭에 표시됩니다.

## 삭제

워크로드, Pod, Service, Ingress, ConfigMap, Secret, Node, 또는 **API 리소스** 아래의 모든 객체 등 객체의 모든 페이지 헤더에 **삭제** 버튼이 있습니다. 목록에서는 행에 체크한 다음 **N개 삭제** 버튼을 누르면, 대상을 나열한 확인을 한 번 거쳐 모두 삭제합니다.

객체는 `kubectl delete`처럼 `Background` 전파로 삭제됩니다. 객체는 즉시 사라지고, 가비지 컬렉터가 그 객체가 소유한 것(Deployment의 ReplicaSet과 Pod, CronJob의 Job)을 삭제합니다.

확인 창에는 그다음에 일어날 일이 표시됩니다.

- **컨트롤러가 소유한 Pod**(ReplicaSet, StatefulSet, DaemonSet, Job)는 다시 생깁니다. 컨트롤러가 그 자리에 새 Pod를 시작합니다. 없애려면 워크로드를 삭제하거나 스케일하세요.
- **Namespace** 객체를 삭제하면 그 안의 모든 것, 즉 워크로드, Pod, Service, ConfigMap, Secret이 함께 삭제됩니다.
- **Node** 객체는 먼저 드레인되지 않고 클러스터 목록에서 삭제됩니다.

## 작동하지 않을 때

- **허용되지 않음**: 자격 증명으로 할 수 없는 변경은 "자격 증명으로 shop Namespace의 deployments를 patch할 수 없습니다."처럼 동사와 리소스를 밝히고, 확인용 `kubectl auth can-i` 명령을 함께 표시합니다.
- **그사이에 변경됨**: 확인한 시점과 작업한 시점 사이에 다른 사람이 객체를 바꿨다면 Homeport가 알려 줍니다. 객체를 다시 확인한 다음 재시도하세요.
- **이미 없음**: 이미 삭제된 것을 삭제하면 더 이상 존재하지 않는다고 표시됩니다.
