### The `volumes` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/volumes.ts:
### change the text there, then run it again.

volumes-nav = 볼륨
volumes-title = 볼륨
volumes-summary = { $count ->
       *[other] 볼륨 { NUM($count) }개
    } · 사용 안 함 { $unused }개
volumes-cleanUp = 정리
volumes-cleanUp-anonymous = 사용하지 않는 익명 볼륨
volumes-cleanUp-all = 사용하지 않는 모든 볼륨
volumes-create = 볼륨 만들기
volumes-search = 볼륨 검색
volumes-empty-title = 볼륨 없음
volumes-empty-description = 볼륨은 컨테이너를 다시 시작하거나 다시 만들어도 데이터를 유지합니다.
volumes-column-name = 이름
volumes-column-stack = 스택
volumes-column-driver = 드라이버
volumes-column-size = 크기
volumes-size-label = 크기:
volumes-size-measuring = 측정 중
volumes-size-unknown = 이 볼륨의 드라이버는 크기를 보고하지 않습니다.
volumes-size-refresh = 크기 새로 고침
volumes-column-usedBy = 사용 중인 컨테이너
volumes-column-created = 생성일
volumes-unused = 사용 안 함
volumes-inspectOne = { $name } 검사
volumes-deleteOne = { $name } 삭제
volumes-browseOne = { $name } 볼륨의 파일 찾아보기
volumes-browse-title = { $name } 볼륨의 파일
volumes-browse-description = 읽기 전용: 컨테이너가 사용하는 중에도 여기서는 볼륨을 변경할 수 없습니다.
volumes-browse-saveOne = { $name } 저장
volumes-browse-saved = { $name } 저장됨
volumes-browse-savedArchive = { $name } 항목을 .tar 아카이브로 저장했습니다
volumes-helper-needed = Homeport는 짧게 실행되는 도우미 컨테이너로 볼륨을 여는데, 여기에는 { $image } 이미지(약 { $size })가 필요합니다. 이 호스트에는 이 이미지가 없습니다.
volumes-helper-removed = Homeport는 동의한 경우에만 이 이미지를 풀하며, 작업이 끝나면 다른 곳에서 사용하지 않는 한 다시 삭제합니다.
volumes-helper-pull = { $image } 풀
volumes-helper-pulling = { $image } 풀하는 중…
volumes-helper-progress = 풀 진행률
volumes-moreActions = { $name } 추가 작업
volumes-backupMenu = 파일로 백업…
volumes-restoreMenu = 파일에서 복원…
volumes-cloneMenu = 복제…
volumes-clone-title = { $volume } 볼륨 복제
volumes-clone-description = 파일을 소유자 및 권한과 함께 이 호스트의 새 볼륨으로 복사합니다. 복사본은 어떤 스택에도 속하지 않습니다.
volumes-clone-name = 복사본 이름
volumes-clone-start = 복제
volumes-clone-into = 대상: { $target }
volumes-clone-copying = 복사하는 중…
volumes-clone-done = { $source } 볼륨을 { $target } 볼륨으로 복제했습니다
volumes-restoreFromFile = 파일에서 복원
volumes-backup-title = { $volume } 볼륨 백업
volumes-backup-saveAs = { $volume } 볼륨을 백업할 위치
volumes-backup-to = 대상: { $path }
volumes-backup-done = { $volume } 볼륨을 { $path }에 백업했습니다
volumes-restore-pick = 볼륨을 복원할 파일
volumes-restore-title = 볼륨 복원
volumes-restore-titleInto = { $volume } 볼륨 복원
volumes-restore-from = 원본: { $path }
volumes-restore-into = 복원 대상
volumes-restore-newVolume = 새 볼륨
volumes-restore-existingVolume = 기존 볼륨
volumes-restore-volume = 볼륨
volumes-restore-chooseVolume = 볼륨 선택
volumes-restore-nameTaken = 이름이 { $volume }인 볼륨이 이미 있습니다.
volumes-restore-start = 복원
volumes-restore-done = { $volume } 볼륨을 복원했습니다
volumes-restore-cancelled = { $volume } 볼륨의 복원이 취소되었습니다. 볼륨에 일부만 기록되었을 수 있습니다.
volumes-transfer-written = 기록됨
volumes-transfer-sent = 전송됨
volumes-transfer-archives = Tar 아카이브
volumes-flow-running-backup = 실행 중인 컨테이너 { $count ->
       *[other] { NUM($count) }개
    }({ $containers })가 { $volume } 볼륨을 사용합니다. 컨테이너가 쓰는 동안 백업하면 쓰기 도중의 데이터베이스처럼 일관성이 깨질 수 있습니다.
volumes-flow-running-restore = 실행 중인 컨테이너 { $count ->
       *[other] { NUM($count) }개
    }({ $containers })가 { $volume } 볼륨을 사용합니다. Homeport는 복원하는 동안 컨테이너를 중지하고 복원 후 다시 시작합니다.
volumes-flow-running-clone = 실행 중인 컨테이너 { $count ->
       *[other] { NUM($count) }개
    }({ $containers })가 { $volume } 볼륨을 사용합니다. 컨테이너가 쓰는 동안 복사하면 쓰기 도중의 데이터베이스처럼 일관성이 깨질 수 있습니다.
volumes-flow-anyway-backup = 그래도 백업
volumes-flow-anyway-clone = 그래도 복사
volumes-flow-stopDuring-backup = 백업하는 동안 { $count ->
       *[other] 컨테이너 중지
    }
volumes-flow-stopDuring-restore = 중지 후 복원
volumes-flow-stopDuring-clone = 복사하는 동안 { $count ->
       *[other] 컨테이너 중지
    }
volumes-flow-replace = { $volume } 볼륨에 이미 파일이 있습니다. 복원하면 이 파일을 삭제하고 백업의 파일로 바꿉니다.
volumes-flow-replaceConfirm = 내용 바꾸기
volumes-usedBy = 사용 중인 컨테이너: { $containers }.
volumes-notUsed = 이 볼륨을 사용하는 컨테이너가 없습니다.
volumes-createDialog-title = 볼륨 만들기
volumes-createDialog-name = 이름
volumes-createDialog-driver = 드라이버
volumes-deleted = { $name } 볼륨을 삭제했습니다
volumes-pruned = { $count ->
       *[other] 볼륨 { NUM($count) }개
    }를 삭제하고 공간 { $space }를 확보했습니다
volumes-remove-title = { $name } 볼륨을 삭제하시겠습니까?
volumes-remove-warning = <strong>이 볼륨의 모든 데이터가 영구적으로 삭제됩니다.</strong>
volumes-remove-mounted = {" "}컨테이너 { $containers }에 마운트되어 있습니다. 먼저 해당 컨테이너를 삭제하세요.
volumes-remove-confirm = 볼륨 및 데이터 삭제
volumes-prune-allTitle = 사용하지 않는 모든 볼륨을 삭제하시겠습니까?
volumes-prune-anonymousTitle = 사용하지 않는 익명 볼륨을 삭제하시겠습니까?
volumes-prune-allWarning = 이름 있는 볼륨을 포함해, 컨테이너가 사용하지 않는 모든 볼륨의 데이터가 영구적으로 삭제됩니다.
volumes-prune-anonymousWarning = 컨테이너가 사용하지 않는 이름 없는 볼륨의 데이터가 영구적으로 삭제됩니다.
volumes-prune-confirm = 볼륨 및 데이터 삭제
