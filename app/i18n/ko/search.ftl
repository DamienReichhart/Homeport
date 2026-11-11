### The `search` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/search.ts:
### change the text there, then run it again.

search-kind-container = 컨테이너
search-kind-stack = 스택
search-kind-image = 이미지
search-kind-volume = 볼륨
search-kind-network = 네트워크
search-onHostState = { $host } · { $state ->
        [running] 실행 중
       *[other] 중지됨
    }
search-showAll = { $count }개 모두 보기
search-skipped = { $count ->
       *[other] 호스트 { NUM($count) }개
    }는 검색하지 않음(연결되지 않음)
