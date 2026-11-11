### The `containerd` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/containerd.ts:
### change the text there, then run it again.

containerd-state-none = 태스크 없음
containerd-state-created = 생성됨
containerd-state-running = 실행 중
containerd-state-paused = 일시 중지됨
containerd-state-pausing = 일시 중지하는 중
containerd-state-stopped = 중지됨
containerd-state-unknown = 알 수 없음
containerd-state-exited = 종료됨({ $code })
containerd-createdBy-kubernetes = Kubernetes
containerd-createdBy-nerdctl = nerdctl
containerd-createdBy-docker = Docker
containerd-createdBy-other = ctr 또는 기타
containerd-containers-nav = 컨테이너
containerd-containers-title = 컨테이너
containerd-containers-summary = { $namespace } 네임스페이스에 { $count ->
       *[other] 컨테이너 { NUM($count) }개
    }, { $running }개 실행 중.
containerd-containers-search = 이름, ID 또는 이미지로 필터
containerd-containers-showSandboxes = Pod의 pause 컨테이너 표시
containerd-containers-sandbox = pause
containerd-containers-empty = { $namespace }에 컨테이너 없음
containerd-containers-emptyDescription = nerdctl, ctr 또는 Kubernetes로 만든 컨테이너가 여기에 표시됩니다. 다른 컨테이너를 보려면 호스트 전환기 아래에서 다른 네임스페이스를 선택하세요.
containerd-containers-column-name = 이름
containerd-containers-column-image = 이미지
containerd-containers-column-state = 태스크
containerd-containers-column-createdBy = 생성 도구
containerd-containers-column-created = 생성일
containerd-containers-action-start = 시작
containerd-containers-action-stop = 중지
containerd-containers-action-pause = 일시 중지
containerd-containers-action-resume = 재개
containerd-containers-action-kill = 신호 보내기
containerd-containers-actionOne-start = { $name } 시작
containerd-containers-actionOne-stop = { $name } 중지
containerd-containers-actionOne-pause = { $name } 일시 중지
containerd-containers-actionOne-resume = { $name } 재개
containerd-containers-removeOne = { $name } 삭제
containerd-containers-done-start = { $name } 시작됨
containerd-containers-done-stop = { $name } 중지됨
containerd-containers-done-pause = { $name } 일시 중지됨
containerd-containers-done-resume = { $name } 재개됨
containerd-containers-done-kill = { $name }에 { $signal } 보냄
containerd-containers-removed = { $name } 삭제됨
containerd-containers-kubelet = Kubernetes가 이 컨테이너를 관리합니다. kubelet은 중지된 컨테이너를 다시 시작하고, 삭제된 컨테이너를 다시 만듭니다. 대신 클러스터에서 해당 Pod를 변경하세요.
containerd-containers-remove-title = { $name } 컨테이너를 삭제하시겠습니까?
containerd-containers-remove-running = 태스크가 실행 중입니다. Homeport가 태스크를 강제 종료한 다음 태스크와 컨테이너를 삭제합니다.
containerd-containers-remove-stopped = Homeport가 태스크와 컨테이너를 삭제합니다.
containerd-containers-remove-snapshot = 변경된 파일과 함께 스냅샷({ $key })도 삭제
containerd-containers-remove-killAndRemove = 강제 종료 후 삭제
containerd-containers-kill-title = { $name }에 신호 보내기
containerd-containers-kill-description = ctr task kill -s처럼 태스크의 주 프로세스에 보냅니다. SIGHUP 같은 이름이나 숫자를 입력하세요.
containerd-containers-kill-signal = 신호
containerd-containers-kill-send = 보내기
containerd-containers-backToList = 컨테이너로 돌아가기
containerd-containers-tab-overview = 개요
containerd-containers-tab-logs = 로그
containerd-containers-tab-exec = Exec
containerd-containers-tab-spec = 스펙
containerd-logs-from = 원본: { $path }
containerd-logs-none-title = 이 컨테이너의 출력은 기록되지 않습니다
containerd-logs-none-description = containerd는 로그를 보관하지 않습니다. 태스크를 시작하는 도구가 출력이 갈 곳을 정합니다. ctr과 대부분의 다른 클라이언트로 만든 컨테이너는 출력을 어디에도 보내지 않으며, nerdctl과 Kubernetes는 Homeport가 읽는 파일에 출력을 씁니다.
containerd-logs-driver-title = nerdctl이 이 컨테이너의 출력을 { $driver } 드라이버로 보냅니다
containerd-logs-driver-description = Homeport는 nerdctl의 기본 json-file 로그만 읽습니다. { $driver } 드라이버가 로그를 보관하는 곳에서 읽거나, json-file 드라이버로 컨테이너를 다시 실행하세요.
containerd-exec-startFirst = 터미널을 열려면 컨테이너를 시작하세요.
containerd-exec-command = 명령
containerd-exec-commandPlaceholder = 명령을 지정하지 않으면 셸
containerd-exec-user = 사용자 ID
containerd-exec-workingDir = 폴더
containerd-exec-open = 다시 열기
containerd-exec-exited = 프로세스가 종료되었습니다.
containerd-exec-exitedWithCode = 프로세스가 종료되었습니다(코드 { $code }).
containerd-containers-field-id = ID
containerd-containers-field-namespace = 네임스페이스
containerd-containers-field-image = 이미지
containerd-containers-field-runtime = 런타임
containerd-containers-field-snapshot = 스냅샷
containerd-containers-field-pid = PID
containerd-containers-field-exit = 마지막 종료
containerd-containers-field-pod = Kubernetes Pod
containerd-containers-field-created = 생성일
containerd-containers-field-updated = 업데이트
containerd-containers-field-extensions = 확장
containerd-containers-labels = 레이블
containerd-containers-noSpec = containerd에 이 컨테이너의 런타임 스펙이 없습니다.
containerd-images-nav = 이미지
containerd-images-title = 이미지
containerd-images-summary = { $namespace } 네임스페이스에 { $count ->
       *[other] 이미지 { NUM($count) }개
    }.
containerd-images-pull = 이미지 풀
containerd-images-pulled = { $reference } 이미지를 { $namespace } 네임스페이스로 풀했습니다
containerd-images-removed = { $name } 삭제됨
containerd-images-search = 이름으로 필터
containerd-images-empty = { $namespace }에 이미지 없음
containerd-images-emptyDescription = 이미지를 풀하세요. containerd가 이미지를 가져온 다음 컨테이너를 실행할 수 있도록 이 호스트의 플랫폼에 맞게 압축을 풉니다.
containerd-images-column-name = 이름
containerd-images-column-platforms = 플랫폼
containerd-images-column-size = 크기
containerd-images-column-containers = 컨테이너
containerd-images-column-created = 생성일
containerd-images-sizeUnknown = 이 호스트 플랫폼용 콘텐츠가 모두 있지는 않습니다.
containerd-images-removeOne = { $name } 삭제
containerd-images-inUse = 이 네임스페이스의 컨테이너가 이 이미지로 만들어졌습니다. 먼저 컨테이너를 삭제하세요.
containerd-images-remove-title = { $name } 이미지를 삭제하시겠습니까?
containerd-images-remove-description = containerd가 이름을 삭제하고, 다른 곳에서 콘텐츠와 스냅샷을 사용하지 않게 되면 해제합니다.
containerd-snapshots-nav = 스냅샷
containerd-snapshots-title = 스냅샷
containerd-snapshots-summary = { $namespace }의 { $count ->
       *[other] 스냅샷 { NUM($count) }개
    }가 { $total } 용량을 사용합니다. { $unused ->
        [0] 사용하지 않는 스냅샷은 없습니다
       *[other] { NUM($unused) }개는 사용하지 않습니다
    }({ $unusedSize }).
containerd-snapshots-snapshotter = 스냅샷터
containerd-snapshots-unusedOnly = 사용하지 않는 항목만
containerd-snapshots-unused = 사용 안 함
containerd-snapshots-usedByContainer = 컨테이너 { $id }
containerd-snapshots-usedByImage = 이미지 { $name }
containerd-snapshots-empty = { $snapshotter }에 스냅샷 없음
containerd-snapshots-emptyDescription = 스냅샷은 압축 해제된 이미지의 레이어와 컨테이너의 쓰기 가능한 레이어입니다.
containerd-snapshots-column-key = 키
containerd-snapshots-column-kind = 종류
containerd-snapshots-column-size = 크기
containerd-snapshots-column-usedBy = 사용 주체
containerd-snapshots-kind-committed = 레이어
containerd-snapshots-kind-active = 쓰기 가능
containerd-snapshots-kind-view = 보기
containerd-snapshots-kind-unknown = 알 수 없음
containerd-snapshots-removeOne = { $key } 삭제
containerd-snapshots-removed = 스냅샷 { $key } 삭제됨
containerd-snapshots-inUse = 이 스냅샷을 사용하거나 이 스냅샷을 기반으로 하는 항목이 있습니다. 먼저 컨테이너나 이미지를 삭제하세요.
containerd-snapshots-remove-title = 스냅샷({ $key })을 삭제하시겠습니까?
containerd-snapshots-remove-description = 사용하는 항목이 없습니다. 파일({ $size })이 호스트에서 삭제됩니다.
containerd-namespaces-nav = 네임스페이스
containerd-namespaces-title = 네임스페이스
containerd-namespaces-summary = { $count ->
       *[other] 네임스페이스 { NUM($count) }개
    }. 모든 containerd 객체는 하나의 네임스페이스에 속합니다. nerdctl은 default, Kubernetes는 k8s.io, Docker는 moby를 사용합니다.
containerd-namespaces-new = 새 네임스페이스
containerd-namespaces-created = { $name } 네임스페이스를 만들었습니다
containerd-namespaces-removed = { $name } 네임스페이스를 삭제했습니다
containerd-namespaces-shown = 표시 중
containerd-namespaces-show = 표시
containerd-namespaces-removeOne = { $name } 삭제
containerd-namespaces-notEmpty = 비어 있는 네임스페이스만 삭제할 수 있습니다. 먼저 컨테이너와 이미지를 삭제하세요.
containerd-namespaces-remove-title = { $name } 네임스페이스를 삭제하시겠습니까?
containerd-namespaces-remove-description = 개수에 표시되지 않는 스냅샷이나 콘텐츠가 아직 있으면 containerd가 거부합니다.
containerd-namespaces-column-name = 이름
containerd-namespaces-column-containers = 컨테이너
containerd-namespaces-column-images = 이미지
containerd-namespaces-column-labels = 레이블
containerd-namespaces-empty = 네임스페이스 없음
containerd-namespaces-emptyDescription = containerd는 default 네임스페이스에 처음 무언가가 만들어질 때 이 네임스페이스를 만듭니다.
containerd-namespaces-newDialog-title = 새 네임스페이스
containerd-namespaces-newDialog-description = 컨테이너와 이미지를 다른 항목과 분리해 두기 위한 빈 네임스페이스입니다.
containerd-namespaces-newDialog-name = 이름
containerd-namespaces-newDialog-invalid = 문자와 숫자를 사용하고, 점, 대시, 밑줄 하나로 구분하세요.
containerd-namespaces-newDialog-create = 만들기
