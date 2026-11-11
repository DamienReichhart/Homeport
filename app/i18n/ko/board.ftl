### The `board` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/board.ts:
### change the text there, then run it again.

board-nav = 보드
board-title = 보드
board-summary = { $hosts ->
       *[other] 호스트 { NUM($hosts) }개
    }에서 { $count ->
       *[other] 컨테이너 { NUM($count) }개
    } 중 { $running }개 실행 중
board-column-host = 호스트
board-column-stack = 스택
board-sortBy = { $column } 기준 정렬
board-filter-hosts = 호스트
board-filter-allHosts = 모든 호스트
board-filter-someHosts = { $count ->
       *[other] 호스트 { NUM($count) }개
    }
board-filter-state = 상태
board-filter-stack = 스택
board-filter-anyStack = 모든 스택
board-filter-clear = 필터 지우기
board-state-all = 모든 상태
board-state-running = 실행 중
board-state-stopped = 중지됨
board-state-unhealthy = 비정상
board-search = 모든 호스트 검색
board-skipped = { $name }({ $status }) 호스트는 표시되지 않습니다.
board-openHost = 호스트 열기
board-listFailed = { $name } 호스트의 컨테이너 목록을 가져오지 못했습니다.
board-noHosts-title = 연결된 호스트가 없습니다
board-noHosts-description = 보드에는 연결된 모든 호스트의 컨테이너가 표시됩니다.
board-empty-title = 아직 컨테이너가 없습니다
board-empty-description = 연결된 호스트 중 컨테이너를 실행하는 호스트가 없습니다.
board-noMatch = 이 필터와 일치하는 컨테이너가 없습니다.
board-selection = 선택한 컨테이너
board-selectAll = 표시된 모든 컨테이너 선택
board-selectOne = { $host } 호스트의 { $name } 선택
board-remove-title = { $count ->
       *[other] 컨테이너 { NUM($count) }개
    }를 삭제하시겠습니까?
board-remove-description = 익명 볼륨과 함께 삭제되며, 이름 있는 볼륨은 유지됩니다. 실행 중인 컨테이너는 먼저 중지됩니다.
board-remove-onHost = { $host } 호스트: { $names }
