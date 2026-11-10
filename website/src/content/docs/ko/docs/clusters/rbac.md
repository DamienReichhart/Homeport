---
title: RBAC
description: 클러스터의 Role, 바인딩, 주체와 각 주체가 어디서 무엇을 할 수 있는지, 그리고 내 자격 증명으로 무엇을 할 수 있는지 확인합니다.
---

Kubernetes 클러스터 사이드바의 **RBAC**는 클러스터에서 누가 무엇을 할 수 있는지 보여 줍니다. 읽기 전용입니다. Homeport는 RBAC 객체를 만들거나 바꾸거나 삭제하지 않습니다(YAML로 편집하려면 여전히 [API 리소스](/docs/clusters/api-resources/)를 쓰면 됩니다).

다른 페이지처럼 Namespace 선택을 따릅니다. Namespace를 고르면 그 Namespace의 Role과 RoleBinding, 그리고 어디서나 적용되는 ClusterRole과 ClusterRoleBinding을 보여 줍니다.

## Role

**Role**은 Role(각각 자신의 Namespace 안)과 ClusterRole을 규칙 수와 함께 나열합니다. 하나를 열면 그 페이지가 나옵니다:

- Kubernetes가 쓰는 그대로의 **규칙**: 리소스, API 그룹(core 그룹은 `core`), 리소스 이름과 동사, 또는 `/healthz` 같은 비리소스 URL
- **이 역할을 쓰는 바인딩**, 각각 주체로 가는 링크 포함

**집계됨**으로 표시된 ClusterRole에는 `aggregationRule`이 있습니다. 그 페이지에는 레이블 셀렉터와 그에 일치하는 ClusterRole이 표시되며, 그 규칙에는 일치하는 ClusterRole의 규칙이 포함됩니다. Kubernetes 기본 `admin`, `edit`, `view`가 이렇게 만들어지므로, `rbac.authorization.k8s.io/aggregate-to-edit: "true"` 레이블이 붙은 CRD의 ClusterRole은 `edit`에 나타납니다.

## Binding

**Binding**은 RoleBinding과 ClusterRoleBinding을 나열하며, 각각이 부여하는 역할(링크 포함)과 지정한 주체(Users, Groups, ServiceAccounts)를 보여 줍니다. 역할이 존재하지 않는 바인딩은 **찾을 수 없음**으로 표시되며 아무것도 부여하지 않습니다.

## 주체

**주체**는 바인딩이 지정한 모든 대상을 나열합니다. User, Group 또는 ServiceAccount를 열면 할 수 있는 일을 리소스 × 동사 행렬로 보여 주며, 적용 범위별로 묶습니다. 먼저 **클러스터 전체**, 그다음 각 Namespace입니다. 그 아래에는 권한을 부여하는 바인딩과 방식(직접 지정 또는 그룹을 통해)이 나옵니다.

Homeport는 적용되는 모든 바인딩의 규칙을 합칩니다:

- ClusterRoleBinding은 ClusterRole을 어디서나 부여합니다. 모든 Namespace, Node 같은 클러스터 범위 리소스, 비리소스 URL에 적용됩니다.
- RoleBinding은 ClusterRole을 바인딩하더라도 자신의 Namespace 안에서만 역할을 부여합니다. `prod`의 RoleBinding으로 `edit`에 바인딩된 ServiceAccount는 `prod`에서는 편집할 수 있지만 다른 곳에서는 할 수 없습니다.
- Kubernetes가 모든 주체를 자동으로 넣는 그룹에 대한 바인딩도 포함됩니다. ServiceAccount는 `system:serviceaccounts`, `system:serviceaccounts:<자신의 Namespace>`, `system:authenticated`에 속하며, 사용자 `system:serviceaccount:<Namespace>:<이름>`이기도 합니다. User는 `system:authenticated`에 속합니다.
- `*`는 모든 동사, API 그룹 또는 리소스를, `*/scale`은 모든 리소스의 해당 하위 리소스를 뜻하며, 리소스 이름이 있는 규칙은 그 객체에만 적용됩니다(행렬의 **대상만**에 표시). 더 넓은 행이 이미 포함하는 행은 생략됩니다.

**이것은 최선의 추정입니다.** 주체의 권한은 API 서버에 묻지 않고 내 자격 증명으로 읽을 수 있는 바인딩으로 계산하므로, 다른 사람을 가장할 권한이 필요하지 않습니다. 따라서 웹훅(관리형 클러스터에서 흔함)이나 ABAC 같은 다른 인가자가 허용하는 것, 그리고 인증기가 사용자에게 부여하는 그룹(OIDC 클레임이나 클라이언트 인증서의 조직에서 오는 것)은 반영되지 않을 수 있습니다. 이것들은 API 서버만 압니다. 내 자격 증명에 대해 확실히 하려면 **여기서 무엇을 할 수 있나요?**를 쓰세요.

Namespace를 고른 상태에서는 그 Namespace의 RoleBinding만 읽습니다. 주체의 권한을 모든 곳에서 보려면 **모든 Namespace**를 고르세요.

## 여기서 무엇을 할 수 있나요?

**여기서 무엇을 할 수 있나요?**는 Homeport가 쓰는 자격 증명에 대해, 고른 Namespace(**모든 Namespace**일 때는 클러스터의 기본 Namespace)에서 API 서버 자체가 내놓는 답이며, `kubectl auth can-i --list`가 보여 주는 것과 같습니다. 이를 알려 주는 클러스터(Kubernetes 1.28 이상)에서는 어떤 사용자로 로그인했고 어떤 그룹에 속하는지 보여 주며, 그 주체의 페이지로 가는 링크도 제공합니다.

API 서버가 목록이 불완전할 수 있다고 하면(웹훅 같은 인가자는 나열할 수 있는 것보다 더 많이 허용할 수 있습니다) 페이지가 그렇게 알려 줍니다.

**…할 수 있나요?**는 `kubectl auth can-i`처럼 질문 하나를 합니다. 동사, 리소스, Namespace(비워 두면 모든 Namespace)를 지정하고, 필요하면 API 그룹, 하위 리소스, 객체 이름도 지정합니다. 리소스는 복수형(`secrets`), 약칭(`deploy`), 그룹이 붙은 복수형(`deployments.apps`) 모두 되며, Homeport가 kubectl처럼 해석합니다. 답에는 같은 질문을 복사할 수 있는 명령이 함께 나옵니다. 예를 들어 `kubectl auth can-i delete secrets --namespace prod`는 같은 답을 줍니다.

내 자격 증명으로 RBAC 객체 목록을 전혀 볼 수 없으면, 페이지는 거부된 내용을 알리는 메시지 아래에 **여기서 무엇을 할 수 있나요?**만 보여 줍니다. 일부만 볼 수 있으면(예: RoleBinding은 되지만 ClusterRole은 안 되는 경우) 페이지가 어느 것인지 알려 주며, 표시된 내용에는 그것들이 빠져 있습니다.
