### The `rbac` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/rbac.ts:
### change the text there, then run it again.

rbac-nav = RBAC
rbac-title = RBAC
rbac-summary = { $roles ->
       *[other] 역할 { NUM($roles) }개
    }, { $bindings ->
       *[other] 바인딩 { NUM($bindings) }개
    }
rbac-refresh = 새로 고침
rbac-tab-roles = Role
rbac-tab-bindings = Binding
rbac-tab-subjects = 주체
rbac-tab-access = 여기서 무엇을 할 수 있나요?
rbac-search-roles = 역할 검색
rbac-search-bindings = 바인딩 검색
rbac-search-subjects = 주체 검색
rbac-column-kind = 종류
rbac-column-name = 이름
rbac-column-namespace = Namespace
rbac-column-rules = 규칙
rbac-column-role = 역할
rbac-column-subjects = 주체
rbac-column-bindings = 바인딩
rbac-column-how = 방식
rbac-column-age = 경과 시간
rbac-clusterWide = 클러스터 전체
rbac-aggregated = 집계됨
rbac-roleMissing = 찾을 수 없음
rbac-empty = 여기에는 아무것도 없습니다.
rbac-refused = 자격 증명으로 일부 RBAC 객체의 목록을 볼 수 없어, 표시된 내용에는 그 객체가 빠져 있습니다:
rbac-forbidden = 자격 증명으로 이 클러스터의 RBAC 객체 목록을 볼 수 없습니다. API 서버가 알려 주는 대로, 허용된 내용만 표시합니다.
rbac-role-kind = 종류
rbac-role-namespace = Namespace
rbac-role-aggregation = 다음과 일치하는 ClusterRole을 집계
rbac-role-aggregatedFrom = 다음의 규칙을 포함
rbac-role-labels = 레이블
rbac-role-age = 경과 시간
rbac-role-rules = 규칙
rbac-role-noRules = 규칙이 없습니다.
rbac-role-bindings = 이 역할을 쓰는 바인딩
rbac-role-noBindings = 여기 표시된 바인딩 중 이 역할을 쓰는 것이 없습니다.
rbac-role-gone = 이 역할은 목록에 없습니다(더 이상 없을 수 있습니다).
rbac-rule-resources = 리소스
rbac-rule-apiGroups = API 그룹
rbac-rule-resourceNames = 리소스 이름
rbac-rule-urls = 비리소스 URL
rbac-rule-verbs = 동사
rbac-coreGroup = core
rbac-subject-bestEffort = Homeport가 읽을 수 있는 바인딩으로 계산한 최선의 추정입니다. 웹훅이나 ABAC 인가자가 허용하는 것, 인증기가 사용자에게 부여하는 그룹은 반영되지 않을 수 있습니다.
rbac-subject-groups = 다음 그룹에 대한 바인딩도 포함됩니다:
rbac-subject-onlyNamespace = { $namespace }의 RoleBinding만 계산합니다. 다른 것도 보려면 모든 Namespace를 선택하세요.
rbac-subject-cluster = 클러스터 전체
rbac-subject-clusterHint = 모든 Namespace, 클러스터 범위 리소스, 비리소스 URL에 적용됩니다.
rbac-subject-inNamespace = { $namespace } 안
rbac-subject-none = 여기 표시된 바인딩 중 아무것도 부여하지 않습니다.
rbac-subject-bindings = 바인딩
rbac-subject-direct = 직접 지정
rbac-subject-through = { $group } 그룹을 통해
rbac-subject-via = { $bindings }에서 부여
rbac-matrix-resource = 리소스
rbac-matrix-names = 대상만
rbac-matrix-yes = 허용
rbac-matrix-all = 허용: 모든 동사(*)
rbac-matrix-url = 비리소스 URL
rbac-matrix-verbs = 동사
rbac-matrix-empty = 없음.
rbac-access-about = Homeport가 { $name }에서 쓰는 자격 증명이 { $namespace }에서 할 수 있는 일을 API 서버의 답변대로 보여 줍니다.
rbac-access-user = 로그인 사용자
rbac-access-groups = 그룹
rbac-access-myBindings = 나를 지정한 바인딩
rbac-access-incomplete = API 서버에 따르면 이 목록은 불완전할 수 있습니다. 웹훅이나 ABAC처럼 규칙을 나열할 수 없는 인가자가 더 많이 허용할 수 있습니다. 확실히 하려면 아래 “…할 수 있나요?”로 물어보세요.
rbac-canI-title = …할 수 있나요?
rbac-canI-verb = 동사
rbac-canI-resource = 리소스
rbac-canI-resourcePlaceholder = secrets, deploy, deployments.apps
rbac-canI-namespace = Namespace
rbac-canI-allNamespaces = 모든 Namespace
rbac-canI-group = API 그룹
rbac-canI-subresource = 하위 리소스
rbac-canI-name = 이름
rbac-canI-optional = 선택 사항
rbac-canI-ask = 묻기
rbac-canI-yes = 예
rbac-canI-no = 아니요
rbac-canI-denied = 명시적으로 거부됨
rbac-canI-checked = 대상: { $resource }
rbac-canI-unknown = 이 클러스터에는 { $resource }라는 리소스가 없어, 입력한 그대로 물었습니다.
