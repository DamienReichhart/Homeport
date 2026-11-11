### The `config` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/config.ts:
### change the text there, then run it again.

config-nav = 구성
config-title = 구성
config-configMaps = ConfigMap
config-secrets = Secret
config-configMap = ConfigMap
config-secret = Secret
config-configMapsSummary = { $count ->
       *[other] ConfigMap { NUM($count) }개
    }
config-secretsSummary = { $count ->
       *[other] Secret { NUM($count) }개
    }
config-searchConfigMaps = ConfigMap 검색
config-searchSecrets = Secret 검색
config-column-name = 이름
config-column-namespace = Namespace
config-column-type = 유형
config-column-keys = 키
config-column-age = 경과 시간
config-tab-data = 데이터
config-tab-overview = 개요
config-tab-events = 이벤트
config-tab-source = YAML
config-detail-namespace = Namespace
config-detail-type = 유형
config-detail-labels = 레이블
config-detail-annotations = 어노테이션
config-noKeys = 키가 없습니다.
config-binary = (바이너리 데이터)
config-base64 = base64
config-reveal = 표시
config-revealOne = { $key } 표시
config-hide = 숨기기
config-copy = { $key } 복사
config-reveal-failed = Homeport가 { $key } 값을 표시하지 못했습니다
