### The `nodes` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/nodes.ts:
### change the text there, then run it again.

nodes-nav = Node
nodes-title = Node
nodes-summary = { $count ->
       *[other] Node { NUM($count) }개
    }
nodes-search = Node 검색
nodes-ready = 준비됨
nodes-notReady = 준비되지 않음
nodes-cordoned = 스케줄링 사용 안 함
nodes-column-name = 이름
nodes-column-status = 상태
nodes-column-roles = 역할
nodes-column-version = 버전
nodes-column-ip = 내부 IP
nodes-column-platform = 플랫폼
nodes-column-cpu = CPU
nodes-column-memory = 메모리
nodes-column-age = 경과 시간
nodes-tab-overview = 개요
nodes-tab-pods = Pod
nodes-tab-conditions = 조건
nodes-tab-events = 이벤트
nodes-tab-source = YAML
nodes-detail-roles = 역할
nodes-detail-version = Kubelet
nodes-detail-os = OS 이미지
nodes-detail-kernel = 커널
nodes-detail-runtime = 컨테이너 런타임
nodes-detail-addresses = 주소
nodes-detail-taints = 테인트
nodes-detail-noTaints = 없음
nodes-detail-age = 경과 시간
nodes-detail-labels = 레이블
nodes-detail-resources = 리소스
nodes-resource-name = 리소스
nodes-resource-capacity = 용량
nodes-resource-allocatable = 할당 가능
nodes-column-cpuUsed = CPU 사용량
nodes-column-memoryUsed = 메모리 사용량
nodes-tab-metrics = 메트릭
