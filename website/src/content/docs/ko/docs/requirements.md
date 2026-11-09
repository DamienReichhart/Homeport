---
title: 시스템 요구 사항
description: Homeport 실행에 필요한 것과, SSH, TLS, 스택 배포, Kubernetes 및 Nomad 클러스터, Helm을 사용하는 데 필요한 선택 구성 요소.
---

## 운영 체제

| 시스템  | 버전                                                        | 아키텍처            |
| ------- | ----------------------------------------------------------- | ------------------- |
| Windows | Windows 10 및 11                                            | x64, ARM64          |
| macOS   | macOS 10.15 Catalina 이상                                   | Apple 실리콘, Intel |
| Linux   | glibc 2.35 이상: Ubuntu 22.04, Debian 12, Fedora 36 및 이후 | x64, ARM64          |

Homeport는 시스템의 웹 뷰로 창을 그립니다. Windows에서는 WebView2(Windows 10 및 11에 포함), macOS에서는 WebKit, Linux에서는 WebKitGTK(`.deb` 및 `.rpm` 패키지의 의존성으로 설치되고 AppImage에는 함께 들어 있음)를 사용합니다.

## 관리할 Docker Engine

Homeport에는 컨테이너 런타임이 포함되어 있지 않습니다. 이미 어딘가에서 실행 중인 Docker Engine을 관리합니다.

- **이 컴퓨터에서** — Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop 또는 Podman. Homeport가 자동으로 찾아 줍니다. [로컬 엔진](/docs/hosts/local/)을 참고하세요.
- **서버에서** — [SSH](/docs/hosts/ssh/)나 [TLS 또는 TCP](/docs/hosts/tls-tcp/)로 접근할 수 있는 엔진.

Podman의 Docker 호환 소켓을 포함해, Docker Engine API를 지원하는 엔진이면 모두 작동합니다.

[containerd](/docs/hosts/containerd/) 1.7 이상도 이 컴퓨터의 소켓이나 SSH를 통해 직접 관리합니다. k3s의 containerd, Docker Engine 자체의 containerd, 서버의 containerd 서비스가 해당합니다. 여기서 이미지를 풀하려면 containerd 2.0 이상이 필요하고, 로그와 터미널을 쓰려면 SSH로 연결한 호스트이거나 이 컴퓨터(Linux)의 containerd여야 합니다.

[Incus](/docs/hosts/incus/) 6.0 이상도 이 컴퓨터의 소켓이나 SSH, 또는 신뢰 토큰을 사용한 HTTPS로 관리합니다. 가상 머신을 쓰려면 Incus 서버에 하드웨어 가상화(`/dev/kvm`)가 필요하고, 가상 머신에서 터미널을 쓰려면 이미지가 `incus-agent`를 실행해야 합니다.

Podman 호스트에서는 Podman 자체 API도 사용합니다. [Pod](/docs/guides/podman-pods/)에는 Podman 4.0 이상이 필요하고, [Quadlet 유닛](/docs/guides/quadlet/)은 Podman 5.7부터 목록에 표시되며 5.8부터 설치, 편집, 삭제할 수 있습니다. Quadlet 유닛을 시작하고 중지하려면 호스트에 SSH로 연결하거나 이 컴퓨터(Linux)의 Podman이어야 합니다.

## 선택 사항: Kubernetes 클러스터

[클러스터](/docs/clusters/)를 살펴보려면 클러스터에 접근할 수 있는 kubeconfig(`kubectl`이 쓰는 것이나 직접 붙여 넣은 것)와 API 서버에 대한 네트워크 접근이 필요합니다. Homeport는 Kubernetes API와 직접 통신하므로 `kubectl`은 필요하지 않습니다.

- **Kubernetes 버전**: Kubernetes 1.24 이상을 실행하는 클러스터. Homeport는 안정 API(`v1`, `apps/v1`, `batch/v1`, `networking.k8s.io/v1`)만 사용하며, 1.33 API를 기준으로 빌드되었습니다.
- **Exec 플러그인**: 클라우드 kubeconfig는 자격 증명을 얻기 위해 이 컴퓨터에서 실행할 프로그램을 지정합니다. EKS는 [AWS CLI](https://aws.amazon.com/cli/)(버전 2), GKE는 Google Cloud CLI의 `gke-gcloud-auth-plugin`, AKS는 `kubelogin`입니다. kubeconfig에 지정된 프로그램을 설치하세요. [Exec 플러그인](/docs/clusters/exec-plugins/)을 참고하세요.

## 선택 사항: Nomad 클러스터

[Nomad 클러스터](/docs/clusters/nomad/)를 관리하려면 Nomad 에이전트의 HTTP API(기본 포트 4646)에 대한 네트워크 접근이 필요하고, 클러스터에서 ACL이 켜져 있으면 토큰도 필요합니다. 필요한 [ACL 정책](/docs/clusters/nomad/#acl-정책)을 참고하세요. Homeport는 Nomad API와 직접 통신하므로 `nomad` CLI는 필요하지 않으며, 클러스터에는 아무것도 설치하지 않습니다.

- **Nomad 버전**: Nomad 1.6 이상. **편집 후 실행**은 job의 제출된 명세에서 시작하는데, Nomad는 1.6부터 이 명세를 보관합니다.

## 선택 사항: OS 키체인

Homeport는 SSH 키, TLS 인증서, 클러스터의 kubeconfig 자격 증명, 스택 환경 변수, 레지스트리 로그인, 저장소의 배포 키와 토큰을 운영 체제의 키체인에 보관하는 키로 암호화합니다.

- **macOS** — 키체인
- **Windows** — 자격 증명 관리자
- **Linux** — GNOME Keyring이나 KWallet 같은 Secret Service 제공자

키체인이 없으면 Homeport는 배너를 표시하지만, 로컬 소켓, 일반 TCP, **ssh-agent**를 사용하는 SSH로는 계속 작동합니다. TLS 호스트, 저장된 키를 쓰는 SSH 호스트, 클러스터, 스택 환경 변수, 레지스트리 로그인, 비공개 저장소에는 키체인이 필요합니다. 키체인이 없으면 이미지는 익명으로 풀되고 공개 저장소만 추가할 수 있습니다. [보안과 데이터](/docs/reference/security/)를 참고하세요.

## 선택 사항: Docker Compose

[관리형, 링크된, 동기화 스택](/docs/guides/stacks/)을 배포하려면 Homeport가 호스트가 아닌 **이** 컴퓨터에서 `docker compose` 명령을 실행합니다. Compose 플러그인이 포함된 Docker CLI(Docker Desktop에 포함)나 독립 실행형 `docker-compose` v2를 설치하세요.

Compose가 없어도 링크된 스택을 포함한 스택은 계속 표시되고 시작, 중지, 삭제할 수 있습니다. 배포에만 Compose가 필요합니다. Homeport가 Compose를 찾았는지는 **설정 → 이 컴퓨터**에서 확인할 수 있습니다.

[Swarm](/docs/guides/swarm/)에 스택을 배포할 때는 이 컴퓨터에서 `docker stack deploy`를 실행하므로 `docker` 명령 자체가 필요합니다(Compose 플러그인은 사용하지 않습니다). Docker Desktop에는 포함되어 있고, Linux에서는 배포판의 Docker CLI 패키지(`docker-ce-cli`, `docker.io` 또는 `docker`)를 설치하세요. 이 컴퓨터에서 엔진이 실행될 필요는 없습니다.

## 선택 사항: git

[동기화 스택](/docs/guides/synced-stacks/)은 **이** 컴퓨터에 설치된 `git`으로 저장소를 클론하고 가져옵니다. Git for Windows, Apple의 명령줄 도구(`xcode-select --install`) 또는 배포판의 `git` 패키지를 사용하세요. Windows에서는 SSH 저장소에 시스템의 OpenSSH를 쓰도록 설정합니다. 다른 기능은 모두 git 없이 작동합니다. Homeport가 git을 찾았는지는 **설정 → 이 컴퓨터**에서 확인할 수 있습니다.

## 선택 사항: Helm

클러스터에서 [Helm 릴리스](/docs/clusters/helm/)를 관리하려면 Homeport가 **이** 컴퓨터에서 `helm` 명령(버전 3 이상)을 실행합니다. 비공개 연결을 통해 helm이 클러스터를 가리키게 하므로 클러스터 자격 증명이 디스크에 기록되지 않으며, helm은 터미널에서처럼 자체 차트 저장소를 사용합니다. `brew install helm`(macOS), `winget install Helm.Helm`, Chocolatey 또는 Scoop(Windows), `snap install helm --classic`, 배포판의 패키지 또는 [helm.sh](https://helm.sh/docs/intro/install/)의 스크립트(Linux)로 설치하세요. 다른 클러스터 페이지는 모두 helm 없이 작동합니다. Homeport가 helm을 찾았는지는 **설정 → 이 컴퓨터**에서 확인할 수 있습니다.

## 선택 사항: 시스템 트레이

Homeport는 시스템 트레이(macOS에서는 메뉴 막대, Windows에서는 알림 영역)에 아이콘을 두고, 모든 호스트를 상태와 함께 나열합니다. Windows와 macOS에는 항상 트레이가 있습니다. Linux에서는 **appindicator** 라이브러리가 필요합니다. AppImage에는 포함되어 있고, `.deb`와 `.rpm` 패키지는 이를 권장 패키지로 지정하므로 권장 패키지 설치를 끄지 않았다면 `apt`, `dnf`, `zypper`가 설치합니다. 그렇지 않으면 `libayatana-appindicator3-1`(Debian, Ubuntu, openSUSE)이나 `libayatana-appindicator-gtk3`(Fedora)을 설치하세요. 일부 데스크톱은 트레이 아이콘을 숨깁니다. GNOME은 AppIndicator Support 같은 확장이 있어야 표시합니다. 트레이가 없으면 Homeport는 창으로만 작동하고, 창을 닫으면 종료됩니다.

## 기능별 요구 사항

| 기능                                      | 키체인 | Compose CLI | git  | helm |
| ----------------------------------------- | :----: | :---------: | :--: | :--: |
| Swarm에 스택 배포                         |   —    | docker CLI  |  —   |  —   |
| 로컬 소켓 및 TCP 호스트                   |   —    |      —      |  —   |  —   |
| ssh-agent를 쓰는 SSH 호스트               |   —    |      —      |  —   |  —   |
| Homeport 키나 가져온 키를 쓰는 SSH 호스트 |  필요  |      —      |  —   |  —   |
| TLS 호스트                                |  필요  |      —      |  —   |  —   |
| 컨테이너, 이미지, 볼륨, 네트워크          |   —    |      —      |  —   |  —   |
| 스택 시작, 중지, 삭제                     |   —    |      —      |  —   |  —   |
| 관리형 스택과 링크된 스택 배포            |   —    |    필요     |  —   |  —   |
| 관리형 스택이나 동기화 스택의 환경 변수   |  필요  |      —      |  —   |  —   |
| 레지스트리 로그인(비공개 이미지, 푸시)    |  필요  |      —      |  —   |  —   |
| 공개 저장소의 동기화 스택                 |   —    |    필요     | 필요 |  —   |
| 배포 키나 토큰을 쓰는 동기화 스택         |  필요  |    필요     | 필요 |  —   |
| 폴더에서 이미지 빌드                      |   —    |      —      |  —   |  —   |
| Kubernetes 클러스터                       |  필요  |      —      |  —   |  —   |
| Nomad 클러스터                            |  필요  |      —      |  —   |  —   |
| Helm 릴리스                               |  필요  |      —      |  —   | 필요 |
