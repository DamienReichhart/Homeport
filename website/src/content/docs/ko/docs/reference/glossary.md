---
title: 용어집
description: Homeport가 쓰는 용어와 그 의미.
---

Homeport는 대상을 정확한 이름으로 부릅니다. 이 용어들은 앱과 이 문서 전체에 나옵니다. 각 항목에는 제품의 영어 용어와 그 뒤에 한국어 앱이 쓰는 용어를 함께 적었습니다.

### Host (_호스트_)

Homeport가 연결하는 Docker Engine으로, 이름과 접근 방법(로컬 소켓, TCP, TLS 또는 SSH)으로 정의됩니다. 모든 컨테이너, 이미지, 볼륨, 네트워크, 스택은 정확히 하나의 호스트에 속합니다. [호스트와 연결](/docs/hosts/)을 참고하세요.

### Engine kind (_엔진 종류_)

호스트가 가리키는 엔진의 종류입니다. Docker 호환(Docker Engine, 또는 호환 API를 통한 Podman), containerd, Incus가 있습니다. 엔진 종류에 따라 호스트에 표시되는 페이지가 정해지며, containerd와 Incus 호스트에는 전용 페이지가 있습니다. Podman은 별도의 종류가 아니며, Homeport가 연결할 때마다 인식합니다. [엔진 종류](/docs/hosts/#엔진-종류)를 참고하세요.

### Namespace (containerd's) (_containerd 네임스페이스_)

containerd 호스트에서 모든 컨테이너, 이미지, 스냅샷이 속하는 공간입니다. nerdctl은 `default`, Kubernetes는 `k8s.io`, Docker는 `moby`를 사용합니다. containerd 호스트의 페이지는 한 번에 하나씩 보여 줍니다. 클러스터의 Namespace는 Kubernetes의 Namespace입니다. [containerd 네임스페이스](/docs/hosts/containerd/#네임스페이스)를 참고하세요.

### Snapshot (_스냅샷_)

containerd 호스트에서는 디스크에 있는 파일 레이어입니다. 압축을 푼 이미지 레이어나 컨테이너의 쓰기 가능한 레이어가 해당합니다. [스냅샷](/docs/hosts/containerd/#스냅샷)을 참고하세요. Incus 호스트에서는 인스턴스를 복원할 수 있도록 저장한 인스턴스의 상태입니다. [인스턴스 스냅샷](/docs/guides/instances/#스냅샷)을 참고하세요.

### Instance (_인스턴스_)

Incus 호스트가 실행하는 것입니다. 호스트의 커널 위에서 Linux 배포판 전체를 부팅하는 시스템 컨테이너, 또는 자체 커널이 있는 가상 머신입니다. 애플리케이션 하나를 실행하는 컨테이너와는 다릅니다. [인스턴스](/docs/guides/instances/)를 참고하세요.

### Project (Incus's) (_프로젝트_)

Incus 호스트에서 인스턴스, 이미지, 프로필, 사용자 지정 볼륨, 네트워크가 속하는 공간입니다. Incus 호스트의 페이지는 호스트 전환기 아래에서 고른 프로젝트 하나만 보여 줍니다. [프로젝트](/docs/hosts/incus/#프로젝트)를 참고하세요.

### Profile (_프로필_)

Incus 호스트에서, 프로필을 나열한 인스턴스들이 공유하는 구성과 장치입니다. [프로필](/docs/guides/instances/#프로필)을 참고하세요.

### Trust token (_신뢰 토큰_)

Incus 서버에서 `incus config trust add`가 출력하는 것으로, HTTPS로 그 서버를 추가할 때 Homeport에 한 번 붙여 넣습니다. [신뢰 토큰](/docs/hosts/incus/#신뢰-토큰을-사용한-https)을 참고하세요.

### Pod (_Podman Pod_)

Podman 호스트에서 네트워크 하나를 공유하고 함께 시작하고 중지하는 컨테이너 그룹으로, Pod가 게시하는 포트를 보유하는 infra 컨테이너를 중심으로 구성됩니다. 클러스터의 Pod는 Kubernetes Pod입니다. [Podman Pod](/docs/guides/podman-pods/)를 참고하세요.

### Quadlet unit (_Quadlet 유닛_)

`web.container`처럼 Podman 호스트에 있는 파일로, systemd가 `web.service`라는 서비스로 바꿉니다. 그래서 Podman의 컨테이너가 부팅할 때 시작되고 systemd가 계속 실행되도록 관리합니다. [Quadlet 유닛](/docs/guides/quadlet/)을 참고하세요.

### Cluster (_클러스터_)

Homeport가 API를 통해 연결하는 Kubernetes 클러스터 또는 Nomad 클러스터입니다. Kubernetes 클러스터는 kubeconfig의 컨텍스트에서 추가하며, 이름, API 서버 주소, kubeconfig가 제공하는 자격 증명으로 구성되고 암호화하여 저장합니다. Nomad 클러스터는 에이전트 중 하나의 주소로 추가하며, ACL 토큰과 TLS 자료와 함께 암호화하여 저장합니다. Homeport는 어느 쪽에도 아무것도 설치하지 않으며, 그 자격 증명이 허용하는 범위에만 접근합니다. 호스트를 통해 찾고 따로 추가하지 않는 Swarm과는 다릅니다. Kubernetes 자체의 객체는 Namespace, Pod, Deployment, Service, Node…처럼 Kubernetes 이름을 그대로 씁니다. Nomad의 객체는 job, task group, task, allocation, client node, namespace처럼 Nomad 이름을 그대로 씁니다. [클러스터](/docs/clusters/)와 [Nomad 클러스터](/docs/clusters/nomad/)를 참고하세요.

### Allocation (_allocation_)

Nomad 클러스터에서 job의 task group 하나를 클라이언트 노드에 배치한 것으로, 그 태스크들을 실행합니다. allocation의 로그와 셸은 Nomad 서버를 거쳐 노드로 갑니다. [로그와 셸](/docs/clusters/nomad/#로그와-셸)을 참고하세요.

### Swarm (_Swarm_)

Docker Swarm 클러스터로, 그에 속한 호스트를 통해 찾으며 따로 추가하지 않습니다. Homeport는 연결된 아무 매니저 호스트를 통해 Swarm에 접근하고, Swarm ID로 Swarm을 구별하며, Swarm에 대해 아무것도 저장하지 않습니다. Docker가 모든 Swarm에 같은 이름을 붙이므로, 첫 번째 매니저 호스트의 이름을 따서 부릅니다. [Swarm](/docs/guides/swarm/)을 참고하세요.

### Port forward (_포트 포워딩_)

`kubectl port-forward`처럼, Homeport가 Kubernetes API를 통해 Pod의 포트나 준비된 Pod 하나를 거쳐 Service의 포트에 연결하는 이 컴퓨터 루프백의 포트입니다. Homeport가 종료되면 끝납니다. [포트 포워딩](/docs/clusters/port-forward/)을 참고하세요.

### Release (_릴리스_)

Helm 릴리스입니다. 클러스터에 차트를 한 번 설치한 것으로, Namespace 안에서 이름이 붙고, 설치, 업그레이드, 롤백마다 번호가 매겨진 리비전이 있습니다. Helm이 클러스터에 보관하므로, 터미널에서 만든 릴리스와 Homeport에서 만든 릴리스는 같습니다. [Helm 릴리스](/docs/clusters/helm/)를 참고하세요.

### Chart repository (_차트 저장소_)

helm이 차트를 다운로드하는 서버로, 이 컴퓨터에 있는 helm 자체 목록(`helm repo list`가 보여 주는 목록)에 들어 있습니다. OCI 레지스트리의 차트는 대신 `oci://` 참조로 설치합니다. [차트 저장소](/docs/clusters/helm/#차트-저장소)를 참고하세요.

### Node (_노드_)

Swarm 안의 엔진 하나로, 매니저나 워커입니다. 클러스터의 Node는 Kubernetes Node, 즉 클러스터가 Pod를 실행하는 머신입니다. 노드는 Homeport가 연결할 때만 호스트가 되며, 호스트 페이지에서 해당 노드로 연결됩니다.

### Service (_서비스_)

Swarm이 실행하는 것입니다. 이미지, 설정, 레플리카 수로 구성되며, 글로벌 서비스라면 모든 노드에 태스크가 하나씩 있습니다.

### Task (_태스크_)

한 노드에서 서비스가 차지하는 자리 하나로, 그 노드에서 컨테이너로 실행됩니다. 서비스가 수렴하지 않으면 태스크가 그 이유를 알려 줍니다. containerd 호스트에서 태스크는 메타데이터일 뿐인 컨테이너를 위해 실행되는 것이며, 컨테이너를 시작하면 태스크가 만들어집니다.

### Stack (_스택_)

호스트에서 실행되는 Compose 프로젝트, 또는 Swarm에서 실행되는 Swarm 스택으로, 프로젝트 이름으로 식별합니다. 모든 스택은 관리형, 링크된, 동기화, 외부 스택 중 하나입니다. [스택](/docs/guides/stacks/)과 [Swarm 스택](/docs/guides/swarm/#swarm-스택)을 참고하세요.

### Managed Stack (_관리형 스택_)

Homeport가 compose 정의와 환경 변수를 보관하는 스택으로, Homeport에서 배포, 다시 배포, 편집할 수 있습니다.

### Linked Stack (_링크된 스택_)

compose 파일이 이 컴퓨터의 폴더에 있는 스택입니다. Homeport는 폴더와 compose 파일의 위치를 기억하고, 파일을 그 자리에서 읽고 쓰며, 그 폴더를 프로젝트 디렉터리로 삼아 배포하므로 `build:`, `env_file:`, 상대 경로가 작동합니다. 실행 중인 컨테이너의 compose 레이블로 찾거나 폴더에서 만듭니다. 파일이 없는 동안에는 외부 스택이 됩니다. [링크된 스택](/docs/guides/stacks/#링크된-스택)을 참고하세요.

### Synced Stack (_동기화 스택_)

compose 정의가 git 저장소에서 오는 스택입니다. Homeport는 이 컴퓨터에 저장소의 클론을 두고, Homeport가 열려 있는 동안 요청할 때나 주기적으로 브랜치나 태그를 가져오며, 변경 사항을 보여 주고, compose 파일의 폴더에서 커밋을 배포하므로 `build:`와 상대 경로가 작동합니다. Homeport에서는 편집하지 않으며 파일은 git을 통해 바뀝니다. 환경 변수와 동기화 설정만 Homeport가 관리합니다. [동기화 스택](/docs/guides/synced-stacks/)을 참고하세요.

### Repository (_저장소_)

Homeport가 동기화 스택을 위해 읽을 수 있는 git 저장소입니다. URL과 Homeport가 인증하는 방법(Homeport가 생성한 배포 키, 사용자 이름과 토큰, 또는 공개 저장소라면 없음)으로 구성됩니다. 모든 호스트에 대해 한 번만 보관하며, 그 저장소로 만든 모든 동기화 스택이 공유합니다.

### Deploy key (_배포 키_)

Homeport가 저장소 하나를 위해 생성하는 Ed25519 키로, Git 호스팅 서비스에서 읽기 전용 권한으로 그 저장소에 추가합니다. 사용자 자신의 SSH 키가 아닙니다.

### External Stack (_외부 스택_)

호스트에서 실행 중인 것으로 발견되었지만, Homeport가 compose 정의를 저장하지도 않았고, 이 컴퓨터의 폴더에도, 따라가는 저장소에도 없는 스택입니다. 시작, 중지, 삭제할 수 있지만 다시 배포하거나 편집할 수는 없습니다.

### Adopt (_관리형으로 전환_)

compose 정의를 제공하여 외부 스택을 관리형 스택으로 바꾸는 것입니다.

### Registry login (_레지스트리 로그인_)

Docker Hub나 GHCR 같은 컨테이너 레지스트리 하나에 대해 Homeport가 보관하는 사용자 이름과 비밀번호 또는 액세스 토큰으로, 모든 호스트에서 풀, 푸시, 빌드할 때 사용합니다. [설정](/docs/guides/settings/#레지스트리)을 참고하세요.

### Helper container (_도우미 컨테이너_)

엔진 API로는 볼륨을 읽을 수 없으므로, Homeport가 볼륨을 열기 위해(탐색, 백업, 복원, 복제) 호스트에 만드는 수명이 짧은 `busybox` 컨테이너입니다. 네트워크가 없고, 써야 하는 경우가 아니면 볼륨을 읽기 전용으로 마운트하며, 작업이 끝나면 삭제됩니다. 컨테이너 목록이나 이벤트 페이지에는 나타나지 않습니다. [볼륨 파일 탐색](/docs/guides/volumes-networks/#도우미-컨테이너)을 참고하세요.

### Board (_보드_)

연결된 모든 호스트의 컨테이너를 표 하나에 나열하고, 각 행에 호스트를 표시하는 페이지입니다. [보드](/docs/guides/overview/#보드)를 참고하세요.

### Command palette (_명령 팔레트_)

<kbd>Mod</kbd> <kbd>K</kbd>로 여는 검색 상자입니다. 페이지로 이동하고, 호스트를 전환하고, 명령을 실행하고, 연결된 모든 호스트에서 컨테이너, 이미지, 볼륨, 네트워크, 스택을 찾을 수 있습니다. [키보드](/docs/guides/keyboard/#명령-팔레트)를 참고하세요.

### Engine (_엔진_)

컴퓨터에서 컨테이너를 실행하는 Docker Engine(`dockerd`) 또는 Podman 같은 호환 엔진입니다. Homeport는 Docker Engine API로 엔진과 통신합니다.

### Socket (_소켓_)

엔진이 수신 대기하는 주소입니다. `/var/run/docker.sock` 같은 Unix 소켓 파일이나, Windows에서는 `//./pipe/docker_engine` 같은 명명된 파이프입니다.

### Keychain (_키체인_)

비밀번호와 키를 보관하는 운영 체제의 보안 저장소(macOS 키체인, Windows 자격 증명 관리자, Linux Secret Service)로, Homeport는 여기에 비밀 정보를 암호화하는 키를 보관합니다.

### Trust on first use (_최초 사용 시 신뢰_)

Homeport가 SSH 서버를 확인하는 방식입니다. 첫 연결에서 서버 키의 지문을 확인하며, 나중에 키가 바뀌면 다시 확인할 때까지 Homeport가 연결을 거부합니다.

## 한국어

한국어 앱, 문서, 웹사이트는 화면과 페이지가 항상 같은 말을 하도록 다음 용어를 씁니다.

| 영어           | 한국어          |
| -------------- | --------------- |
| Host           | 호스트          |
| Engine         | 엔진            |
| Instance       | 인스턴스        |
| Swarm          | Swarm           |
| Cluster        | 클러스터        |
| Stack          | 스택            |
| Managed Stack  | 관리형 스택     |
| Linked Stack   | 링크된 스택     |
| Synced Stack   | 동기화 스택     |
| External Stack | 외부 스택       |
| Adopt          | 관리형으로 전환 |
| Board          | 보드            |
| Bastion        | 배스천          |
| Deploy key     | 배포 키         |
| Port forward   | 포트 포워딩     |
| Release (Helm) | 릴리스          |
