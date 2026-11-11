### The `podmanpods` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/podmanpods.ts:
### change the text there, then run it again.

podmanpods-nav = Pod
podmanpods-title = Pod
podmanpods-summary = { $count ->
       *[other] Pod { NUM($count) }개
    }
podmanpods-search = Pod 검색
podmanpods-empty = Pod 없음
podmanpods-emptyDescription = Pod는 네트워크를 공유하며 함께 시작하고 중지하는 컨테이너의 묶음입니다. Pod를 만든 다음 그 안에서 컨테이너를 실행하세요.
podmanpods-new = 새 Pod
podmanpods-column-name = 이름
podmanpods-column-status = 상태
podmanpods-column-containers = 컨테이너
podmanpods-column-networks = 네트워크
podmanpods-column-created = 생성일
podmanpods-running = { $count }개 중 { $running }개 실행 중
podmanpods-status-running = 실행 중
podmanpods-status-degraded = 일부 실행 중
podmanpods-status-paused = 일시 중지됨
podmanpods-status-created = 생성됨
podmanpods-status-stopped = 중지됨
podmanpods-status-failed = 실패
podmanpods-action-start = 시작
podmanpods-action-stop = 중지
podmanpods-action-restart = 다시 시작
podmanpods-action-pause = 일시 중지
podmanpods-action-unpause = 재개
podmanpods-actionOne-start = { $name } 시작
podmanpods-actionOne-stop = { $name } 중지
podmanpods-actionOne-restart = { $name } 다시 시작
podmanpods-actionOne-pause = { $name } 일시 중지
podmanpods-actionOne-unpause = { $name } 재개
podmanpods-removeOne = { $name } 삭제
podmanpods-done-start = { $name } 시작됨
podmanpods-done-stop = { $name } 중지됨
podmanpods-done-restart = { $name } 다시 시작됨
podmanpods-done-pause = { $name } 일시 중지됨
podmanpods-done-unpause = { $name } 재개됨
podmanpods-removed = { $name } 삭제됨, { $count ->
        [0] 컨테이너 없음
       *[other] 컨테이너 { NUM($count) }개 포함
    }
podmanpods-created = { $name } 생성됨
podmanpods-remove-title = { $name } Pod를 삭제하시겠습니까?
podmanpods-remove-stopped = { $count ->
        [0] Pod와 인프라 컨테이너가 삭제됩니다.
       *[other] Pod, 인프라 컨테이너, 컨테이너 { NUM($count) }개가 삭제됩니다.
    } 볼륨은 유지됩니다.
podmanpods-remove-running = { $count ->
        [0] Pod가 실행 중입니다. Pod를 중지한 다음 인프라 컨테이너와 함께 삭제합니다.
       *[other] Pod가 실행 중입니다. 컨테이너 { NUM($count) }개를 중지한 다음 Pod 및 인프라 컨테이너와 함께 삭제합니다.
    } 볼륨은 유지됩니다.
podmanpods-remove-stopAndRemove = 중지 후 삭제
podmanpods-detail-containers = { $count ->
        [0] 컨테이너
       *[other] 컨테이너 { NUM($count) }개
    }
podmanpods-detail-noContainers = 이 Pod에서는 아직 인프라 컨테이너 외에 아무것도 실행되지 않습니다.
podmanpods-detail-infra = 인프라
podmanpods-detail-settings = 설정
podmanpods-detail-ports = 게시된 포트
podmanpods-detail-noPorts = 없음
podmanpods-detail-shared = 공유 네임스페이스
podmanpods-detail-networks = 네트워크
podmanpods-detail-hostname = 호스트 이름
podmanpods-detail-created = 생성일
podmanpods-detail-labels = 레이블
podmanpods-detail-command = 생성 명령
podmanpods-detail-runContainer = 컨테이너 실행
podmanpods-detail-runContainerIn = { $name } Pod에서 컨테이너 실행
podmanpods-newDialog-title = 새 Pod
podmanpods-newDialog-description = Pod의 컨테이너는 하나의 네트워크를 공유하므로 포트는 모든 컨테이너를 위해 Pod에 게시됩니다.
podmanpods-newDialog-name = 이름
podmanpods-newDialog-portsHint = 호스트 포트를 비워 두면 Podman이 사용 가능한 포트를 선택합니다. Pod 안에서 실행되는 컨테이너는 자체 포트를 게시할 수 없습니다.
podmanpods-newDialog-labels = 레이블
podmanpods-newDialog-create = Pod 만들기
podmanpods-form-nameInvalid = 문자나 숫자로 시작한 다음 문자, 숫자, _, ., - 를 사용하세요.
podmanpods-form-portInvalid = 포트는 1에서 65535 사이의 숫자입니다.
podmanpods-form-labelInvalid = 한 줄에 key=value 레이블을 하나씩 쓰세요.
