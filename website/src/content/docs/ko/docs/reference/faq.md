---
title: 자주 묻는 질문
description: Homeport에 대해 자주 묻는 질문.
---

### Homeport는 무료입니까?

예. Homeport는 개인 용도와 상업 용도 모두 무료로 사용할 수 있으며, [GNU Affero General Public License 버전 3](https://github.com/DamienReichhart/Homeport/blob/main/LICENSE)(AGPL)에 따른 오픈 소스입니다.

### AGPL은 저에게 어떤 의미입니까?

Homeport를 사용하기만 한다면 달라지는 것이 없습니다. 설치하고, 집이나 직장에서, 원하는 만큼 많은 컴퓨터에서 실행하세요. AGPL은 수정 여부와 관계없이 Homeport를 **배포**하거나, 수정한 버전을 네트워크를 통해 다른 사람에게 제공하는 사람에게만 요구 사항이 있습니다. 그런 경우 같은 라이선스로 전체 소스 코드를 공개해야 합니다. 모든 Homeport 사본은 **설정** 하단의 **라이선스**에서 정확히 그 버전의 소스로 연결됩니다.

0.11까지의 버전은 MIT 라이선스로 출시되었으며, 계속 MIT 라이선스로 이용할 수 있습니다.

### Homeport가 Docker Desktop을 대체합니까?

완전히 대체하지는 않습니다. Homeport는 관리 도구이며, Docker Engine을 직접 실행하지 않습니다. Linux에서는 Docker Engine을 설치하고 Homeport를 인터페이스로 사용하세요. macOS와 Windows에서는 엔진을 실행할 무언가(Docker Desktop, Colima, OrbStack, Rancher Desktop 또는 Podman)가 여전히 필요하며, Homeport는 원격 서버와 함께 그 위에서 작동할 수 있습니다.

### containerd에서도 작동합니까?

예. Docker 없이 직접 작동합니다. 호스트의 엔진을 [containerd](/docs/hosts/containerd/)로 지정하고 이 컴퓨터의 소켓이나 SSH로 연결할 수 있습니다. k3s의 containerd, Docker Engine 자체의 containerd, 서버의 containerd 서비스가 해당합니다. Homeport는 네임스페이스, 컨테이너와 태스크(시작, 중지, 일시 중지, 신호, 삭제), nerdctl이나 Kubernetes가 기록한 로그, 컨테이너 안의 터미널, 이미지(풀, 삭제), 스냅샷을 다룹니다. 컨테이너를 만들거나 스택을 배포하지는 않습니다. 그것은 nerdctl이 합니다.

### Incus에서도 작동합니까?

예. 호스트의 엔진을 [Incus](/docs/hosts/incus/) 6.0 이상으로 지정하고, 이 컴퓨터의 소켓이나 SSH, 또는 `incus config trust add`로 만든 신뢰 토큰을 사용한 HTTPS로 연결할 수 있습니다. Homeport는 프로젝트의 [인스턴스](/docs/guides/instances/)(시스템 컨테이너와 가상 머신)를 나열하고, 시작, 중지, 정지, 삭제, 생성하며, 콘솔과 터미널을 열고, 스냅샷을 찍고 복원하며, 이미지, 프로필, 스토리지 풀, 네트워크를 관리합니다. 스택은 배포하지 않으며 호스트에 아무것도 설치하지 않습니다.

### LXD에서도 작동합니까?

그대로는 작동하지 않습니다. Homeport는 LXD의 커뮤니티 포크인 Incus를 지원하며, API가 달라진 LXD 서버는 거부합니다. `lxd-to-incus`로 마이그레이션한 다음 Incus 호스트로 추가하세요.

### Podman에서도 작동합니까?

예. Podman의 Docker 호환 API 소켓을 통해 작동합니다. Homeport는 Linux의 일반적인 Podman 소켓, 실행 중인 모든 [macOS의 Podman 머신](/docs/hosts/local/#macos의-podman-머신), Windows의 기본 Podman 머신 파이프를 감지합니다. 그 밖의 소켓은 [직접 추가](/docs/hosts/local/#소켓-직접-추가)할 수 있습니다.

Homeport는 연결할 때 Podman을 인식하고, 같은 소켓에서 Podman 자체 API를 통해 Podman에만 있는 기능을 제공합니다. [Pod](/docs/guides/podman-pods/)는 나열, 시작, 중지, 생성, 삭제할 수 있고 실행 대화 상자에서 그 안에 컨테이너를 실행할 수 있습니다. [Quadlet 유닛](/docs/guides/quadlet/)은 나열, 읽기, 설치, 편집, 삭제할 수 있고 SSH로 시작하거나 중지할 수 있습니다.

### Kubernetes, Nomad, Docker Swarm에서도 작동합니까?

Docker Swarm은 됩니다. Homeport는 매니저 호스트 중 하나가 호스트로 추가되어 있으면 매니저 호스트를 통해 Swarm을 찾고 완전히 관리합니다. 노드, 서비스, 태스크, 시크릿, config, Swarm 스택은 물론, Swarm에 배포한 관리형, 링크된, 동기화 스택도 관리합니다. 호스트에서 Swarm을 시작하거나, Swarm에 가입하거나, 떠날 수도 있습니다. 모든 작업은 연결된 매니저의 엔진 API를 통하며, 노드에는 아무것도 설치하지 않습니다. [Swarm](/docs/guides/swarm/)을 참고하세요.

Kubernetes도 됩니다. 이 컴퓨터의 kubeconfig나 붙여 넣은 kubeconfig에서 클러스터를 추가하며, 로컬 클러스터(Docker Desktop, OrbStack, Rancher Desktop, kind, k3d, minikube)는 Homeport가 자동으로 찾습니다. EKS, GKE, AKS는 kubeconfig에 지정된 exec 플러그인으로 작동합니다. Namespace, 워크로드, Pod, Service, Ingress, ConfigMap, Secret, Node, Event, 그리고 사용자 지정 리소스를 포함한 다른 모든 API 리소스를 실시간으로 살펴보고, 로그를 따라가고, Pod에서 터미널을 열고, 포트를 포워딩하고, 스케일, 다시 시작, 삭제하며, 미리 보기와 함께 YAML을 적용하거나 편집할 수 있습니다. 클러스터에는 아무것도 설치하지 않습니다. Homeport는 kubeconfig가 이미 접근할 수 있는 Kubernetes API와 자격 증명의 권한으로만 통신합니다. 이 컴퓨터에 `helm` CLI가 있으면 [Helm 릴리스](/docs/clusters/helm/)를 나열, 설치, 업그레이드, 롤백, 제거하고 helm의 차트 저장소를 관리할 수도 있습니다. [클러스터](/docs/clusters/)를 참고하세요.

Nomad도 됩니다. 에이전트 주소로 클러스터를 추가하며, ACL 토큰과 TLS를 쓰는 경우 함께 지정합니다. job과 allocation을 실시간으로 나열하고, Nomad의 plan을 확인한 뒤 HCL이나 JSON으로 job을 실행하고, job을 편집하여 다시 실행하거나 중지 또는 purge하고, task group을 스케일하고, 태스크의 로그를 따라가고 셸을 열고, 노드를 드레인하고 스케줄링 가능 여부를 바꾸고, 네임스페이스를 고를 수 있습니다. 클러스터에는 아무것도 설치하지 않으며 `nomad` CLI도 필요하지 않습니다. Homeport는 토큰의 권한으로 Nomad의 HTTP API와 통신합니다. [Nomad 클러스터](/docs/clusters/nomad/)를 참고하세요.

### kubectl이 필요합니까?

아니요. Homeport는 kubeconfig를 읽고 Kubernetes API와 직접 통신합니다. Homeport가 실행하는 프로그램은 kubeconfig에 지정된 [exec 플러그인](/docs/clusters/exec-plugins/)뿐입니다. EKS의 AWS CLI, GKE의 `gke-gcloud-auth-plugin`, AKS의 `kubelogin`처럼 `kubectl`이 실행할 것과 같은 프로그램입니다. 그래도 자격 증명으로 무엇을 할 수 있는지 확인할 때는 `kubectl`이 있으면 도움이 됩니다(`kubectl auth can-i`).

### 서버에 무언가를 설치해야 합니까?

아니요. 이미 있을 가능성이 높은 Docker Engine과 SSH 서버만 있으면 됩니다. Homeport는 에이전트를 설치하지 않습니다.

Swarm에도 엔진 외에는 필요한 것이 없습니다. 다만 노드끼리 서로 연결할 수 있어야 하며, 이것은 Homeport가 대신 설정해 줄 수 없습니다. 노드 사이에서 2377/tcp(매니저 방향), 7946/tcp와 7946/udp, 4789/udp를 여세요. [노드 간 포트](/docs/guides/swarm/#노드-간-포트)를 참고하세요.

### 원격 서버를 관리하려면 제 컴퓨터에 Docker가 설치되어 있어야 합니까?

아니요. Homeport는 엔진 API와 SSH를 직접 사용합니다. 예외는 관리형, 링크된, 동기화 스택의 배포뿐입니다. 배포에는 이 컴퓨터의 `docker compose` 명령을 사용하고, Swarm에 배포할 때는 `docker` 명령(`docker stack deploy`)을 사용합니다. 어느 쪽도 이 컴퓨터에 엔진이 필요하지 않습니다.

### Homeport가 닫혀 있을 때도 스택을 동기화합니까?

아니요. [동기화 스택](/docs/guides/synced-stacks/)은 Homeport가 열려 있는 동안에만 새 커밋을 확인하고, 그 설정을 켠 경우 자동으로 배포합니다. Homeport는 서버에 에이전트도 웹훅 수신기도 설치하지 않으므로, 닫혀 있는 동안에는 저장소를 감시하는 것이 없습니다. 다시 열면 한 주기를 기다린 다음 모든 저장소를 확인합니다. 모든 푸시를 24시간 내내 따라 배포해야 한다면 CI에서 배포하세요.

창 없이 Homeport를 계속 실행하려면 [설정 → 데스크톱](/docs/guides/settings/#데스크톱)에서 **닫으면 시스템 트레이로 이동**을 켜고, **로그인 시 시작**과 **트레이에 숨긴 채로 시작**을 켜세요.

### 컨테이너가 비정상 종료되면 Homeport가 알려 줍니까?

예. 시스템 트레이에 숨겨져 있더라도 실행 중이라면 알려 줍니다. 컨테이너가 오류로 중지되거나, 메모리가 부족하거나, 비정상 상태가 되면, 그리고 호스트에 연결할 수 없게 되거나 다시 연결되면 OS 알림을 표시합니다. 각 종류는 [설정 → 알림](/docs/guides/settings/#알림)에서 끌 수 있습니다. Homeport는 각 호스트와 이미 유지하고 있는 이벤트 스트림에서 이를 알아내므로 서버에 아무것도 설치하지 않습니다. Homeport가 실행 중이 아니면 아무에게도 알리지 않습니다.

### Homeport에 git이 필요합니까?

[동기화 스택](/docs/guides/synced-stacks/)에만 필요합니다. 동기화 스택은 이 컴퓨터에 설치된 `git`을, 저장소마다 자체 배포 키나 토큰으로 실행합니다. Homeport가 git을 찾았는지는 **설정 → 이 컴퓨터**에서 알 수 있습니다. 사용자 자신의 git 설정, 자격 증명 도우미, SSH 키는 절대 사용하지 않습니다.

### 여러 사람이 Homeport의 호스트를 공유할 수 있습니까?

각자 자신의 호스트와 키로 자신의 Homeport를 실행합니다. 사람마다, 서버마다 Homeport가 만든 키를 쓰면 `authorized_keys`에서 각 사람의 접근을 확인하고 취소할 수 있습니다.

### Homeport를 다른 컴퓨터로 옮길 수 있습니까?

예. 두 가지 방법이 있습니다.

- **전체**: **설정 → 데이터 → 백업…** 메뉴는 모든 호스트, 클러스터, 스택, 레지스트리 로그인, 저장소, 설정을 비밀 정보와 함께 12자 이상의 암호 문구로 보호되는 파일 하나에 저장합니다. 새 컴퓨터에서 같은 곳의 **복원…** 메뉴로 파일을 열면 들어 있는 내용을 나열한 뒤 기존 내용을 대체합니다. 키나 비밀번호를 다시 묻지 않고 모든 것이 연결되고 배포됩니다. [데이터와 백업](/docs/reference/data-and-backups/#homeport에서-백업하기)을 참고하세요.
- **일부 호스트**: **호스트** 페이지에서 고른 호스트를 **내보내기**하고, 다른 컴퓨터에서 **가져오기**하면 그 컴퓨터에 이미 있는 호스트 옆에 추가됩니다. [다른 컴퓨터로 옮기기](/docs/reference/security/#다른-컴퓨터로-옮기기)를 참고하세요.

`homeport.db`를 복사하는 방법은 비밀 정보에는 통하지 않습니다. 비밀 정보는 이 컴퓨터의 키체인에 보관된 키로 암호화되어 있어서, 다른 곳에서는 **자격 증명 손실**로 표시됩니다. [파일 직접 복사하기](/docs/reference/data-and-backups/#파일-직접-복사하기)를 참고하세요.

### Homeport는 데이터를 어디에 보관합니까?

사용자의 애플리케이션 데이터 폴더에 있는 SQLite 데이터베이스 하나, `homeport.db`에 보관하며, 그 비밀 정보를 암호화하는 키는 운영 체제의 키체인에 둡니다. 서버나 온라인 어디에도 보관하지 않습니다. 시스템별 폴더, 백업할 대상, 복원 방법은 [데이터와 백업](/docs/reference/data-and-backups/)에서 설명합니다.

### 백업에 컨테이너와 볼륨도 포함됩니까?

아니요. 백업에는 Homeport가 보관하는 것, 즉 호스트에 연결하는 방법, 스택의 정의와 환경 변수, 로그인, 설정이 들어 있고, 호스트에서 실행되는 것은 들어 있지 않습니다. 컨테이너, 이미지, 볼륨은 호스트에 남습니다. 볼륨의 데이터를 복사하려면 볼륨 페이지에서 [파일로 백업](/docs/guides/volumes-networks/#백업과-복원)하세요.

### Homeport로 이미지를 빌드할 수 있습니까?

예. 두 곳에서 할 수 있습니다. **이미지** 페이지에서는 이 컴퓨터의 폴더로 이미지를 빌드합니다. 그리고 compose 파일이 이 컴퓨터의 폴더에 있는 스택인 [링크된 스택](/docs/guides/stacks/#링크된-스택)은 `build:`를 사용할 수 있습니다. Homeport가 그 폴더에서 `docker compose`를 실행하므로 빌드 컨텍스트가 그곳에서 호스트의 엔진으로 전송되고, 엔진이 빌드합니다. 코드를 바꾼 뒤에는 **다시 빌드 후 배포**로 다시 빌드합니다. 관리형 스택은 빌드할 수 없습니다. Homeport가 YAML만 보관하고 빌드할 폴더가 없기 때문입니다. 대신 폴더를 링크하거나, 이미지를 레지스트리에 푸시하세요.

### 다른 컴퓨터에서는 왜 스택이 외부 스택으로 표시됩니까?

폴더에서 `docker compose up`으로 시작한 스택은 그 폴더와 compose 파일이 같은 경로에 있고 실행 중인 서비스를 정의하는 컴퓨터에서만 [링크된 스택](/docs/guides/stacks/#링크된-스택을-찾는-방법)으로 표시됩니다. 다른 컴퓨터에는 그 경로가 없으므로 Homeport에 compose 파일이 없고, 외부 스택으로 표시합니다. 그 컴퓨터에서도 관리하려면 프로젝트를 그 컴퓨터에 클론하고 **새 스택 → 이 컴퓨터의 폴더 사용**으로 링크하거나, 관리형 스택으로 전환하세요.

### Homeport가 저에 대한 데이터를 수집합니까?

아니요. 계정, 원격 측정, 분석, 충돌 보고가 없습니다. Homeport가 스스로 이 컴퓨터에서 보내는 요청은 업데이트 확인뿐이며, 이 웹사이트에서 작은 파일을 다운로드할 뿐 사용자에 대한 정보는 보내지 않습니다. **설정 → 업데이트**에서 끌 수 있습니다. 이와 별도로, 새 이미지 확인은 각 호스트의 엔진을 통해 이미지의 레지스트리에 태그가 가리키는 버전을 묻습니다. 로컬 호스트라면 이 요청도 이 컴퓨터에서 나갑니다. [보안과 데이터](/docs/reference/security/#homeport가-연결하는-곳)를 참고하세요.

### Homeport에서 비공개 레지스트리를 사용할 수 있습니까?

예. **설정 → 레지스트리**에서 Docker Hub, GHCR 또는 자체 레지스트리마다 로그인을 추가하세요. Homeport는 어느 호스트에서든 풀, 푸시, 빌드할 때 이 로그인을 엔진에 넘기고, 관리형, 링크된, 동기화 스택을 배포하기 전에 이 로그인으로 스택의 이미지를 풀합니다. 비밀번호와 토큰은 OS 키체인으로 암호화됩니다. Homeport는 이 컴퓨터에서 `docker login`으로 한 로그인은 읽지 않습니다.

### Homeport는 새 이미지가 있는지 어떻게 압니까?

호스트의 엔진을 통해 이미지의 레지스트리에 태그가 지금 어떤 다이제스트를 가리키는지 묻고, 호스트가 풀한 다이제스트와 비교합니다. 아무것도 다운로드하지 않지만, Docker Hub는 새로운 것이 없는 `docker pull`처럼 이 질문도 풀 요청 제한에 포함합니다. 그래서 Homeport는 각 응답을 6시간 동안 보관하고, 한 번에 최대 네 개의 레지스트리에만 묻고, **새 이미지 확인**을 선택할 때만 더 일찍 다시 묻습니다. **설정 → 레지스트리**에 Docker Hub 로그인을 추가하면 제한이 늘어납니다. 레지스트리에 연결할 수 없거나 요청이 제한되면 Homeport는 해당 이미지에 표시를 하지 않고 로그에 한 줄을 기록합니다.

### 어떻게 업데이트합니까?

Windows, macOS, AppImage에서는 새 버전이 나오면 Homeport가 알려 줍니다. **설치 후 다시 시작**을 선택하세요. Linux에서는 `.deb`와 `.rpm` 패키지가 Homeport 패키지 저장소에서 시스템의 다른 패키지와 함께 업데이트됩니다. [업데이트](/docs/reference/updating/)를 참고하세요.

### 업데이트하면 호스트가 사라집니까?

아니요. Homeport의 모든 버전은 0.1.0까지 거슬러 올라가는 모든 이전 버전의 데이터를 열고, 호스트, 스택, 클러스터, 설정, 비밀 정보를 하나도 잃지 않고 업그레이드합니다. 지원하지 않는 것은 이전 버전으로 돌아가는 것 하나뿐이며, 이때 이전 Homeport는 아무것도 바꾸지 않고 메시지를 표시하며 멈춥니다. [버전 간 데이터](/docs/reference/updating/#버전-간-데이터)를 참고하세요.

### Homebrew, winget, Flathub, AUR에 Homeport가 있습니까?

아직 없습니다. Linux에서는 Homeport 자체 APT와 RPM 저장소가 있어서 `apt`, `dnf`, `zypper`로 설치하고 업데이트합니다. Homebrew, winget, Scoop, AUR, Flathub, Snap은 계획되어 있으며, [설치 페이지](/docs/installation/#추후-지원)에 각각의 이슈 링크가 있습니다.

### Homeport를 제 언어로 사용할 수 있습니까?

Homeport는 영어, 프랑스어, 스페인어, 독일어, 이탈리아어, 브라질 포르투갈어, 러시아어, 중국어 간체, 일본어, 한국어, 힌디어, 아랍어를 지원하며, [설정 → 모양](/docs/guides/settings/#모양)에서 언어를 고르지 않으면 운영 체제의 언어를 따릅니다. 웹사이트와 이 문서도 이 모든 언어로 제공됩니다. 법률 페이지만 영어로 되어 있습니다. 언어를 추가하려면 [기여하기](/docs/project/contributing/#homeport-번역)를 참고하세요.

### Homeport는 접근성을 지원합니까?

키보드만으로 사용할 수 있고, 시스템의 동작 줄이기 설정을 따르며, WCAG 2.2 AA 대비를 충족하고, 모든 페이지를 자동 접근성 도구로 검사합니다. 화면 낭독기로 직접 하는 감사가 진행 중입니다. 작동하는 것, 알려진 한계, 장벽을 보고하는 방법은 [키보드와 접근성](/docs/reference/accessibility/)을 참고하세요.

### 명령줄 인터페이스가 있습니까?

아니요. Homeport는 데스크톱 앱입니다. 기존 `docker` CLI를 함께 쓸 수 있으며, 터미널에서 무언가를 바꾸면 Homeport가 실시간으로 갱신됩니다.

### 어떻게 도울 수 있습니까?

[GitHub 이슈](https://github.com/DamienReichhart/Homeport/issues)에서 버그를 보고하고 기능을 제안하거나, [기여하기](/docs/project/contributing/)를 참고하세요.
