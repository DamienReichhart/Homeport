### The `portforwards` namespace, in 한국어 (ko).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/ko/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = 포트 포워딩
portforwards-title = 포트 포워딩
portforwards-summary = { $count ->
       *[other] 포워딩 { NUM($count) }개
    }
portforwards-empty-title = 포트 포워딩 없음
portforwards-empty-description = Pod나 Service의 페이지에서 포트를 포워딩하면 이 컴퓨터에서 연결할 수 있습니다. 포워딩은 Homeport를 종료하면 끝납니다.
portforwards-column-local = 로컬 주소
portforwards-column-target = 포워딩 대상
portforwards-column-namespace = Namespace
portforwards-column-cluster = 클러스터
portforwards-column-connections = 연결
portforwards-forward = 포워딩…
portforwards-forwardPort = 포트 { $port } 포워딩
portforwards-open = 열기
portforwards-stop = 중지
portforwards-stopFailed = Homeport가 포워딩을 중지하지 못했습니다
portforwards-started = { $address } → { $target }:{ $port } 포워딩 중
portforwards-failed = 포트 포워딩이 실패했습니다
portforwards-ended = 포트 포워딩이 끝났습니다
portforwards-dialog-title = { $target }:{ $port } 포워딩
portforwards-dialog-description = { $namespace } Namespace에 있습니다. 포워딩이 실행되는 동안 로컬 포트로 연결하세요.
portforwards-dialog-localPort = 로컬 포트
portforwards-dialog-anyPort = 사용 가능한 아무 포트
portforwards-dialog-loopback = 이 컴퓨터만 연결할 수 있습니다. 포워딩은 127.0.0.1(및 ::1)에서 수신 대기합니다.
portforwards-dialog-submit = 포워딩
