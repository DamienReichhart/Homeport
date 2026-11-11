### The `pods` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/pods.ts:
### change the text there, then run it again.

pods-nav = Pod
pods-title = Pod
pods-summary = { $count ->
       *[other] Pod { NUM($count) }개
    }
pods-search = Pod 검색
pods-empty = Pod가 없습니다.
pods-column-name = 이름
pods-column-namespace = Namespace
pods-column-status = 상태
pods-column-ready = 준비
pods-column-restarts = 재시작 횟수
pods-column-node = Node
pods-column-age = 경과 시간
pods-tab-overview = 개요
pods-tab-conditions = 조건
pods-tab-events = 이벤트
pods-tab-source = YAML
pods-detail-readyOf = 컨테이너 { $total }개 중 { $ready }개 준비됨
pods-detail-namespace = Namespace
pods-detail-node = Node
pods-detail-ip = Pod IP
pods-detail-hostIp = Node IP
pods-detail-qos = QoS 클래스
pods-detail-serviceAccount = 서비스 어카운트
pods-detail-owner = 제어 주체
pods-detail-age = 경과 시간
pods-detail-labels = 레이블
pods-detail-annotations = 어노테이션
pods-container-name = 컨테이너
pods-container-init = init
pods-container-image = 이미지
pods-container-state = 상태
pods-container-ready = 준비
pods-container-isReady = 예
pods-container-notReady = 아니요
pods-container-restarts = 재시작 횟수
pods-container-restartsWhy = { $count }(마지막: { $reason })
pods-container-started = 시작 시각
pods-container-ports = 포트
pods-state-running = 실행 중
pods-state-waiting = 대기 중
pods-state-terminated = 종료됨
pods-tab-logs = 로그
pods-logs-container = 컨테이너
pods-logs-initContainer = { $name }(init)
pods-logs-previous = 이전 실행
pods-logs-noRestart = 다시 시작한 적이 없으므로 이전 실행이 없습니다.
pods-logs-pod = Pod
pods-logs-allPods = { $count ->
        [1] Pod
       *[other] 전체 Pod { NUM($count) }개
    }
pods-logs-noPods = 지금 실행 중인 Pod가 없습니다.
pods-tab-terminal = 터미널
pods-terminal-shell = 셸
pods-terminal-command = 명령
pods-terminal-connect = 연결
pods-terminal-reconnect = 다시 연결
pods-terminal-notRunning = 이 컨테이너는 실행 중이 아닙니다.
pods-terminal-exited = 프로세스가 종료되었습니다.
pods-terminal-exitedWithCode = 프로세스가 종료되었습니다(코드 { $code }).
pods-column-cpu = CPU
pods-column-memory = 메모리
pods-tab-metrics = 메트릭
pods-metrics-noSample = metrics-server에 아직 이 Pod의 샘플이 없습니다. 방금 시작되었거나 실행 중이 아닐 수 있습니다.
