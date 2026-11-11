### The `overview` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/overview.ts:
### change the text there, then run it again.

overview-nav = 개요
overview-title = 개요
overview-addHost = 호스트 추가
overview-addCluster = 클러스터 추가
overview-cluster-nodes = Node { $total }개 중 { $ready }개 준비됨
overview-cluster-nodesNotAllowed = Node: 허용되지 않음
overview-cluster-pods = Pod { $total }개 중 { $running }개 실행 중
overview-cluster-podsNotAllowed = Pod: 허용되지 않음
overview-cluster-notAllowed = 허용되지 않음.
overview-cluster-versionTitle = 클러스터
overview-cluster-version = 버전
overview-cluster-platform = 플랫폼
overview-cluster-server = API 서버
overview-nomad-region = 리전
overview-nomad-address = 주소
overview-cluster-namespace = Namespace
overview-cluster-allNamespaces = 모든 Namespace
overview-cluster-nodesTitle = Node
overview-cluster-nodesReady = 준비됨
overview-cluster-allNodes = 모든 Node
overview-cluster-podsTitle = Pod
overview-cluster-running = { $count }개 실행 중
overview-cluster-pending = { $count }개 대기 중
overview-cluster-crashing = { $count }개 실패 중
overview-cluster-done = { $count }개 완료 또는 기타
overview-cluster-allPods = 모든 Pod
overview-cluster-unreadyTitle = 준비되지 않은 워크로드
overview-cluster-allReady = 모든 Deployment, StatefulSet, DaemonSet의 Pod가 준비되었습니다.
overview-cluster-warningsTitle = 최근 경고
overview-cluster-noWarnings = 경고 이벤트가 없습니다.
overview-cluster-allEvents = 모든 이벤트
overview-host-running = <strong>{ $running }</strong> / { $total } 실행 중
overview-host-images = { $count ->
       *[other] 이미지 { NUM($count) }개
    }
overview-host-containerdContainers = { $namespaces ->
       *[other] 네임스페이스 { NUM($namespaces) }개
    }에 { $count ->
       *[other] 컨테이너 { NUM($count) }개
    }
overview-host-incusInstances = { $projects ->
       *[other] 프로젝트 { NUM($projects) }개
    }에 { $count ->
       *[other] 인스턴스 { NUM($count) }개
    }
overview-incus-title = Incus
overview-incus-version = 버전
overview-incus-os = 시스템
overview-incus-arch = 아키텍처
overview-incus-projects = 프로젝트
overview-incus-shown = 표시 중
overview-incus-holds = { $instances ->
       *[other] 인스턴스 { NUM($instances) }개
    }, { $images ->
       *[other] 이미지 { NUM($images) }개
    }
overview-incus-instances = 인스턴스
overview-incus-images = 이미지
overview-incus-storage = 스토리지
overview-containerd-title = containerd
overview-containerd-version = 버전
overview-containerd-revision = 리비전
overview-containerd-platform = 플랫폼
overview-containerd-namespaces = 네임스페이스
overview-containerd-shown = 표시 중
overview-containerd-holds = { $containers ->
       *[other] 컨테이너 { NUM($containers) }개
    }, { $images ->
       *[other] 이미지 { NUM($images) }개
    }
overview-containerd-containers = 컨테이너
overview-containerd-images = 이미지
overview-containerd-snapshots = 스냅샷
overview-host-cpu = { $cpus ->
       *[other] CPU { NUM($cpus) }개
    } 중 { $percent }
overview-host-cpuTitle = 컨테이너 CPU 사용량, CPU당 100%
overview-host-memoryTitle = 컨테이너 메모리 사용량
overview-engine-title = 엔진
overview-engine-name = 이름
overview-engine-version = 버전
overview-engine-os = OS
overview-engine-kernel = 커널
overview-engine-architecture = 아키텍처
overview-engine-resources = 리소스
overview-engine-resourcesValue = { $cpus ->
       *[other] CPU { NUM($cpus) }개
    } · { $memory }
overview-engine-storageDriver = 스토리지 드라이버
overview-engine-rootDir = 루트 디렉터리
overview-disk-title = 디스크 사용량
overview-disk-cleanUp = 정리…
overview-disk-column-type = 유형
overview-disk-column-total = 전체
overview-disk-column-active = 사용 중
overview-disk-column-size = 크기
overview-disk-column-reclaimable = 확보 가능
overview-disk-images = 이미지
overview-disk-containers = 컨테이너
overview-disk-volumes = 볼륨
overview-disk-buildCache = 빌드 캐시
overview-disk-partial = 이 Docker Engine은 버전 29보다 오래되어 보고하는 정보가 적습니다. 이미지 크기는 추정치이며 볼륨과 빌드 캐시 크기는 확인할 수 없습니다.
overview-events-title = 최근 이벤트
overview-events-all = 모든 이벤트
overview-events-empty = Homeport가 연결된 이후 발생한 일이 없습니다.
overview-prune-title = 정리
overview-prune-description = 이 호스트에서 삭제할 항목을 선택하세요. 이 작업은 되돌릴 수 없습니다.
overview-prune-confirm = 정리
overview-prune-confirmWithVolumes = 볼륨 데이터를 포함해 삭제
overview-prune-containers = 중지된 컨테이너
overview-prune-containers-detail = 실행 중이 아닌 모든 컨테이너를 삭제합니다.
overview-prune-images = 댕글링 이미지
overview-prune-images-detail = 빌드와 풀 후에 남은 태그 없는 레이어입니다.
overview-prune-allImages = 사용하지 않는 모든 이미지
overview-prune-allImages-detail = 컨테이너가 사용하지 않는 태그된 이미지도 삭제합니다.
overview-prune-networks = 사용하지 않는 네트워크
overview-prune-networks-detail = 연결된 컨테이너가 없는 사용자 지정 네트워크입니다.
overview-prune-buildCache = 빌드 캐시
overview-prune-buildCache-detail = 캐시된 빌드 레이어입니다.
overview-prune-volumes = 사용하지 않는 볼륨
overview-prune-volumes-detail = 이름 있는 볼륨을 포함해, 컨테이너가 사용하지 않는 모든 볼륨의 데이터를 삭제합니다.
overview-pruned = { $containers ->
       *[other] 컨테이너 { NUM($containers) }개
    }, { $images ->
       *[other] 이미지 { NUM($images) }개
    }, { $networks ->
       *[other] 네트워크 { NUM($networks) }개
    }, { $volumes ->
       *[other] 볼륨 { NUM($volumes) }개
    }를 삭제하고 공간 { $space }를 확보했습니다.
overview-palette-prune = 선택한 호스트 정리
overview-host-onBoard = 보드에서 이 호스트의 컨테이너 보기
