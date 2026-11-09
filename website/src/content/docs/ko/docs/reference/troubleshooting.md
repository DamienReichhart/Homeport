---
title: 문제 해결
description: 자주 겪는 문제의 해결 방법을 Homeport가 표시하는 메시지별로 정리했습니다.
---

여기에 없는 문제라면 [로그 파일](/docs/reference/security/#로그)을 확인하고, **설정**에 있는 Homeport 버전과 함께 [이슈를 여세요](https://github.com/DamienReichhart/Homeport/issues).

## 설치와 시작

### Windows에 “Windows의 PC 보호”가 표시됨

설치 프로그램이 아직 상용 인증서로 서명되지 않았습니다. **추가 정보**를 선택한 다음 **실행**을 선택하세요. 먼저 [다운로드를 확인](/docs/reference/verifying/)할 수도 있습니다.

### macOS에서 Homeport를 “열 수 없습니다”라고 표시됨

Homeport는 아직 Apple의 공증을 받지 않았습니다. **시스템 설정 → 개인정보 보호 및 보안**을 열고 Homeport 옆의 **확인 없이 열기**를 선택하거나, 다음을 실행하세요.

```sh
xattr -dr com.apple.quarantine /Applications/Homeport.app
```

### AppImage가 시작되지 않음

FUSE에 대한 메시지가 나오면 `libfuse2`(Ubuntu 24.04 이상에서는 `libfuse2t64`)를 설치하세요. 파일에 실행 권한이 있는지도 확인하세요: `chmod +x Homeport*.AppImage`.

### Homeport가 엉뚱한 위치에서 열림

Homeport는 닫을 때의 창 크기, 위치, 최대화 상태로 다시 열립니다. 창이 있던 디스플레이가 없어졌다면 남아 있는 디스플레이의 가운데에 열립니다. 저장된 위치를 지우려면 Homeport를 종료하고 설정 폴더에서 `.window-state.json`을 삭제하세요.

| 시스템  | 파일                                                                |
| ------- | ------------------------------------------------------------------- |
| Linux   | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%APPDATA%\com.homeport.app\.window-state.json`                     |

### Homeport를 다시 열어도 두 번째 창이 열리지 않음

Homeport는 한 번에 하나만 실행됩니다. 이미 실행 중일 때 다시 열면, [시스템 트레이](/docs/guides/overview/#시스템-트레이)에 숨겨져 있더라도 기존 창이 앞으로 나옵니다. 두 개가 실행되면 모든 호스트에 두 번 연결하고 같은 데이터에 기록하게 됩니다.

### 트레이 아이콘이 없음

Linux에서 트레이에는 appindicator 라이브러리가 필요하며, 일부 데스크톱은 트레이 아이콘을 숨깁니다. GNOME에서는 AppIndicator Support 확장을 설치하세요. Homeport가 appindicator 라이브러리를 찾지 못하면 **설정 → 데스크톱**에 그렇게 표시되며, 창을 닫으면 종료됩니다. [요구 사항](/docs/requirements/#선택-사항-시스템-트레이)을 참고하세요.

### 사용할 수 있는 OS 키체인이 없다는 빨간 배너가 표시됨

Homeport가 키체인에 연결하지 못했습니다. Linux에서는 보통 Secret Service가 실행 중이 아니라는 뜻이며, 최소 구성 데스크톱과 창 관리자에서 흔합니다. GNOME Keyring(`gnome-keyring`)을 설치하고 시작하거나 KWallet의 Secret Service를 켠 다음 Homeport를 다시 시작하세요. 그때까지는 로컬 소켓, TCP, ssh-agent 호스트를 사용하세요.

## 업데이트와 패키지 저장소

### “업데이트를 확인하지 못했습니다”

Homeport가 이 웹사이트에서 업데이트 정보를 다운로드하지 못했습니다. 오프라인이거나, `damienreichhart.github.io`를 차단하는 프록시나 방화벽 뒤에 있을 수 있습니다. Homeport는 계속 작동하며 나중에 다시 확인합니다. **설정 → 업데이트 → 지금 확인**을 선택해도 됩니다. 최신 버전은 언제든 [다운로드 페이지](/download/)에서 받을 수 있습니다.

### 업데이트 설치가 실패함

Homeport는 서명이 업데이트 키와 일치하는 업데이트만 설치하며, 다운로드가 불완전하거나 변조되었으면 중단합니다. 나중에 다시 시도하세요. 계속 실패하면 [다운로드 페이지](/download/)에서 새 버전을 받아 기존 버전 위에 설치하세요.

### apt에 “NO_PUBKEY” 또는 “The repository is not signed”가 표시됨

`/etc/apt/keyrings/homeport.asc`에 Homeport 키가 없습니다. [빠른 설치](/docs/installation/#빠른-설치)를 다시 실행하거나, [수동 설정](/docs/installation/#수동-설정)의 처음 두 명령을 실행하세요.

### dnf나 zypper가 키를 가져올지 물음

패키지 관리자가 저장소를 처음 사용할 때 묻는 Homeport 패키지 키입니다. 표시된 지문이 [다운로드 확인](/docs/reference/verifying/#패키지-서명)의 지문과 일치하는지 확인한 다음 수락하세요.

### 릴리스 직후 apt에 “Hash Sum mismatch”가 표시됨

apt가 저장소를 읽는 동안 저장소가 업데이트되고 있었습니다. 몇 분 후 `sudo apt update`를 다시 실행하세요.

### 패키지 관리자에 새 버전이 보이지 않음

저장소가 아직 설정되어 있는지 확인하세요: `/etc/apt/sources.list.d/homeport.sources`, `/etc/yum.repos.d/homeport.repo` 또는 `/etc/zypp/repos.d/homeport.repo`. 삭제했다면 [빠른 설치](/docs/installation/#빠른-설치)를 실행하여 다시 추가하세요. 0.2.0 이하 버전을 다운로드한 파일로 설치했다면 저장소가 추가된 적이 없습니다. [0.2.0 이하에서 업데이트](/docs/reference/updating/#020-이하에서-업데이트)를 참고하세요.

## 연결

### 연결할 수 없음: Permission denied

사용자가 Docker 소켓을 열 수 없습니다. Linux에서는 사용자를 `docker` 그룹에 추가하고 다시 로그인하세요.

```sh
sudo usermod -aG docker $USER
```

### 연결할 수 없음: No such file or directory

소켓 경로에서 수신 대기하는 것이 없습니다. 엔진(Docker Desktop, Colima, OrbStack, Rancher Desktop)을 시작하거나 경로를 확인하세요. [로컬 엔진](/docs/hosts/local/#소켓-직접-추가)을 참고하세요.

### “Docker Engine이 제시간에 응답하지 않았습니다” 또는 “SSH 연결 시간이 초과되었습니다”

Homeport는 연결을 10초 동안 기다립니다. 주소와 포트, 서버가 켜져 있는지, 방화벽이 막고 있지 않은지 확인하세요. Homeport는 스스로 계속 재시도합니다.

### 액세스 거부됨: “서버가 …의 키를 거부했습니다”

서버가 아직 Homeport의 키를 받아들이지 않습니다. 호스트 페이지의 **공개 키 추가** 명령을 서버에서 실행한 다음 **다시 연결**을 선택하세요. [SSH 호스트](/docs/hosts/ssh/#homeport의-키-승인)를 참고하세요.

### “SSH 서버가 /var/run/docker.sock을 열지 못했습니다”

Homeport가 로그인했지만 Docker에 연결하지 못했습니다. 다음을 확인하세요.

- **원격 소켓 경로**. rootless Docker는 `/run/user/<uid>/docker.sock`을 사용합니다.
- 사용자가 Docker를 쓸 수 있는지. 서버에서 `sudo usermod -aG docker <user>`를 실행하세요.
- 서버가 소켓 포워딩을 허용하는지(`sshd_config`의 `AllowStreamLocalForwarding`, 기본값은 켜짐).

### “실행 중인 ssh-agent가 없습니다”

SSH 에이전트를 시작하고 `ssh-add`로 키를 추가하세요. 데스크톱 메뉴에서 Homeport를 열었다면 터미널이 쓰는 에이전트를 Homeport가 보지 못할 수 있습니다. 로그아웃했다가 다시 로그인하거나, `ssh-add -l`에 키가 나열되는 터미널에서 Homeport를 시작하세요.

Windows에서 Homeport는 OpenSSH Authentication Agent 서비스나 PuTTY의 Pageant를 사용합니다. Pageant를 열고 키를 불러오거나, 관리자 PowerShell에서 서비스를 시작하세요.

```powershell
Get-Service ssh-agent | Set-Service -StartupType Automatic
Start-Service ssh-agent
ssh-add
```

### 액세스 거부됨: “TLS 핸드셰이크 실패”

- CA 인증서는 서버 인증서에 서명한 인증서여야 합니다.
- 입력한 주소가 서버 인증서에 있는 이름이나 IP 주소와 일치해야 합니다.
- 서버는 `--tlsverify`로 실행되어야 하며 클라이언트 인증서의 CA를 신뢰해야 합니다.

### 서버 키가 변경됨

SSH 서버가 신뢰한 키와 다른 키를 제시했습니다. 서버를 다시 설치했거나 키를 바꿨다면 **새 키 신뢰**를 선택하세요. 왜 바뀌었는지 모른다면 연결하지 말고 서버 관리자에게 문의하세요. [SSH 호스트](/docs/hosts/ssh/#서버-키가-바뀌었을-때)를 참고하세요.

### 자격 증명 손실

비밀 정보를 암호화하는 키가 키체인에 없거나 바뀌었습니다. 키체인을 초기화했거나, 새 사용자 프로필을 만들었거나, Homeport의 데이터를 다른 컴퓨터로 복사한 경우입니다(이때는 [내보내기와 가져오기](/docs/reference/security/#다른-컴퓨터로-옮기기)를 사용하세요). Homeport는 호스트를 유지합니다. 호스트 페이지에서 자격 증명을 다시 입력하세요.

- **TLS** — 인증서 파일 세 개를 다시 불러오고 **저장**을 선택합니다.
- **가져온 키를 쓰는 SSH** — 개인 키와 암호 문구를 다시 붙여 넣고 **저장**을 선택합니다.
- **Homeport 키를 쓰는 SSH** — **키 다시 생성**을 선택한 다음 서버에서 새 키를 승인합니다.

이전 키로 봉인된 관리형 스택의 환경 변수도 다시 입력해야 합니다.

## 클러스터

### 액세스 거부됨: “… 클러스터가 kubeconfig의 자격 증명을 거부했습니다”

API 서버가 401로 응답했습니다. 인증서나 토큰이 만료되었거나 취소되었습니다. `kubectl --context <context> get namespaces`가 아직 작동하는지 확인한 다음, 클러스터 페이지에서 **kubeconfig에서 새로 고침**을 사용하세요. Exec 플러그인이라면 Homeport가 이미 한 번 새 토큰을 요청했습니다.

### “… 명령이 이 컴퓨터에 설치되어 있지 않습니다”

클러스터의 [exec 플러그인](/docs/clusters/exec-plugins/)(`aws`, `gke-gcloud-auth-plugin`, `kubelogin`…)을 `PATH`나 일반적인 설치 위치에서 찾지 못했습니다. 설치하고 터미널에서 실행되는지 확인한 다음 **다시 연결**하세요.

### “… 명령은 로그인이 필요한데, Homeport 안에서는 로그인할 수 없습니다”

Exec 플러그인이 브라우저나 장치 코드를 요구합니다. 터미널에서 `kubectl --context <context> get namespaces`를 한 번 실행하여 로그인한 다음 **다시 연결**하세요.

### “… 명령이 … 클러스터의 자격 증명을 가져오지 못했습니다”

Exec 플러그인이 실행되었지만 실패했습니다. **세부 정보**에 플러그인이 출력한 내용의 끝부분이 표시되며, 만료된 프로필이나 잘못된 리전이 원인인 경우가 많습니다. Homeport가 스스로 다시 시도합니다.

### “… 사용자가 gcp 인증 공급자로 로그인하는데, Kubernetes는 이 공급자를 더 이상 지원하지 않습니다”

`gcp`와 `azure` 인증 공급자는 Kubernetes에서 제거되었습니다. `gke-gcloud-auth-plugin`을 설치한 상태로 자격 증명을 다시 가져오거나(GKE), `kubelogin convert-kubeconfig`를 실행한(AKS) 다음, 컨텍스트를 다시 추가하세요.

### “자격 증명에 … Namespace의 …에 대한 ‘…’ 권한이 없습니다”

클러스터의 RBAC가 kubeconfig의 사용자에게 그 작업을 허용하지 않습니다. 오른쪽 위의 선택기에서 볼 수 있는 Namespace를 고르거나, 클러스터 관리자에게 요청하세요. 메시지에 이를 확인하는 `kubectl auth can-i` 명령이 나옵니다. 다른 페이지는 계속 작동합니다.

### 연결할 수 없음: “…에서 연결을 거부했습니다” 또는 “… 클러스터가 제시간에 응답하지 않았습니다”

이 컴퓨터에서 API 서버에 연결할 수 없습니다. (로컬) 클러스터가 중지되었거나, VPN 뒤에 있거나, kubeconfig의 서버 주소가 바뀌었습니다. Homeport는 스스로 계속 재시도합니다. 같은 컨텍스트로 `kubectl cluster-info`를 확인하세요.

### “… 클러스터의 인증서가 kubeconfig에 있는 CA로 서명되지 않았습니다”

kind, k3d, minikube는 클러스터를 다시 만들 때마다 새 인증서를 만듭니다. 클러스터 페이지에서 **kubeconfig에서 새로 고침**을 사용하세요.

### Nomad: “… 클러스터가 Homeport가 보낸 ACL 토큰을 알지 못합니다”

토큰을 잘못 입력했거나, 만료되었거나, 삭제되었습니다. [ACL 정책](/docs/clusters/nomad/#acl-정책)의 정책으로 새 토큰을 만들고(`nomad acl token create -name=homeport -policy=homeport`) 그 secret ID를 클러스터의 **설정**에 붙여 넣으세요. 그때까지 클러스터는 **액세스 거부됨** 상태로 기다립니다.

### Nomad: “… 클러스터가 거부했습니다. ACL 토큰에 이 작업에 필요한 권한이 없습니다”

토큰의 정책이 이 페이지나 작업에 필요한 권한을 부여하지 않습니다. 메시지가 어떤 권한인지 알려 줍니다(job이나 로그 읽기, job 실행이나 중지, 스케일, 셸 열기, 노드 변경). 토큰의 정책에 해당 capability를 추가하세요. [ACL 정책](/docs/clusters/nomad/#acl-정책)에 각각 필요한 것이 나와 있습니다. 다른 페이지는 계속 작동합니다.

### Nomad: “… 클러스터에 리더가 없습니다. 서버들이 리더를 합의하지 못합니다”

클러스터 서버 중 과반수가 실행되고 있지 않거나 서로 연결할 수 없어서 Nomad가 응답할 수 없습니다. 서버에서 `nomad server members`로 확인하고 충분한 수의 서버를 되살리세요. Homeport는 계속 재시도합니다.

### Nomad: “… 클러스터에 … job이 없습니다”

job이 중지되어 purge되었거나, 가비지 컬렉션되었거나, 선택한 네임스페이스가 아닌 다른 네임스페이스에 있습니다. 필요하면 **모든 네임스페이스**를 선택하여 job 페이지를 다시 여세요.

### Nomad: “… task group은 …개보다 많이 실행할 수 없습니다” 또는 “…개보다 적게”

task group의 `scaling` 블록에 `min`과 `max`가 설정되어 있고, 요청한 수가 그 범위를 벗어났습니다. 범위를 넘어서려면 job 명세에서 한계를 바꾸고 다시 실행하세요.

### Nomad: “… job의 deployment가 진행 중인 동안에는 Nomad가 스케일하지 않습니다”

서비스 job의 새 버전을 실행하면 deployment가 시작되며, Nomad는 새 버전의 모든 allocation이 정상이 될 때까지 개수 변경을 거부합니다. job의 allocation이 안정될 때까지 기다린 다음 다시 스케일하세요.

### Nomad: “job 명세를 구문 분석할 수 없습니다(…번째 줄, …번째 열)”

Nomad의 파서가 그 위치에서 문제를 발견했습니다. **세부 정보**에 파서의 메시지가 있습니다. Homeport는 변수 파일을 전달하지 않으므로 `variable` 블록에는 기본값이 필요합니다. JSON 명세는 `nomad job inspect`가 출력하는 것처럼 Nomad API 형식의 job이어야 합니다.

### Nomad: “Nomad가 … job의 모든 allocation을 배치할 수 없습니다”

plan에서 task group이 요구하는 리소스를 갖추거나 제약 조건에 맞는 적격 노드를 찾지 못했습니다. plan에 어느 쪽인지 나옵니다. **그래도 실행**을 선택하면 배치할 수 없는 것을 노드가 받아들일 수 있을 때까지 대기열에 넣습니다. 아니면 리소스를 줄이거나, 제약 조건을 바꾸거나, 노드를 추가하세요.

### Nomad: “계획한 이후 … job이 변경되었으므로 실행하지 않았습니다”

plan과 **실행** 사이에 다른 사람이 job을 실행하거나 중지했으며, Homeport는 그 변경을 덮어쓰지 않습니다. 돌아가서 다시 plan을 만들고, 실행하기 전에 새 diff를 확인하세요.

### Nomad: “allocation …의 … 태스크가 실행 중이 아닙니다”

셸에는 실행 중인 태스크가 필요합니다. 이 태스크는 아직 시작되지 않았거나, 끝났거나, 실패했습니다. 로그는 allocation이 가비지 컬렉션될 때까지 계속 읽을 수 있습니다.

### Nomad: “… 클러스터가 … allocation이 실행되는 노드에 연결할 수 없습니다”

로그와 셸은 서버를 거쳐 클라이언트 노드로 가는데, 서버가 노드에 연결할 수 없습니다. 노드가 다운되었거나, 연결이 끊겼거나, 에이전트가 중지되었습니다. 노드 페이지나 `nomad node status`로 확인하세요.

### Helm: “helm CLI가 이 컴퓨터에 설치되어 있지 않습니다”

[Helm 릴리스](/docs/clusters/helm/)에는 이 컴퓨터에 `helm` 3 이상이 필요합니다. 설치한 다음(`brew install helm`, `winget install Helm.Helm` 또는 `snap install helm --classic`) Helm 페이지를 다시 여세요. Homeport가 찾은 helm은 **설정 → 이 컴퓨터**에 표시됩니다. `helm` 2는 지원하지 않는다고 표시됩니다.

### Helm: “이 릴리스의 다른 설치, 업그레이드 또는 롤백이 아직 진행 중입니다”

릴리스가 `pending-…` 상태입니다. 설치, 업그레이드 또는 롤백이 끝나지 않았으며, 중단된 경우가 많습니다. helm은 그 작업이 끝날 때까지 릴리스 변경을 거부합니다. 기다리거나, 릴리스의 **기록**에서 마지막으로 배포된 리비전으로 롤백하세요.

## Homeport 사용

### 배포가 비활성화됨: “docker compose CLI가 설치되어 있지 않습니다”

Homeport가 스택을 배포하려면 이 컴퓨터에 `docker compose`가 있어야 합니다. Docker Desktop이나 Compose 플러그인이 포함된 Docker CLI를 설치한 다음 Homeport를 다시 시작하세요. 찾았는지는 **설정 → 이 컴퓨터**에 표시됩니다.

설치했는데도 찾지 못한다면 Homeport의 `PATH`가 터미널과 다를 수 있습니다. macOS에서 데스크톱 메뉴로 시작했을 때 흔합니다. `docker`가 `/usr/local/bin` 같은 표준 위치에 있는지 확인하세요.

### 배포 실패: “… 서비스가 build:를 사용하는데”

관리형 스택에는 빌드할 폴더가 없습니다. 대신 [프로젝트 폴더를 링크](/docs/guides/stacks/#링크된-스택-만들기)하거나(링크된 스택은 빌드할 수 있습니다), 이미지를 빌드하여 레지스트리에 푸시한 다음 `image:`를 사용하세요.

### 스택이 외부 스택이 됨

[링크된 스택](/docs/guides/stacks/#폴더가-이동하거나-사라졌을-때)은 폴더가 옮겨지거나, 이름이 바뀌거나, 삭제되었을 때, 또는 그 안의 compose 파일이 그렇게 되었을 때 ⚠ **compose 파일이 더 이상 이 컴퓨터에 없습니다** 표시와 함께 외부 스택으로 표시됩니다. 잃어버린 것은 없습니다. 폴더를 원래 자리로 되돌리면 다음 새로 고침 때 다시 링크된 스택이 됩니다. 폴더를 영구적으로 옮겼다면 스택의 **정의** 탭에서 **잊기**를 선택한 다음, **새 스택 → 이 컴퓨터의 폴더 사용**으로 새 위치에서 다시 링크하세요.

다른 컴퓨터의 폴더에서 시작한 스택은 이 컴퓨터에 그 경로가 없으므로 여기서는 항상 외부 스택입니다.

### “Homeport 밖에서 변경됨”

Homeport가 마지막으로 읽은 이후 링크된 스택의 compose 파일이 Homeport 밖에서(편집기나 `git pull`로) 편집되었습니다. **다시 불러오기**를 선택하면 현재 상태를 볼 수 있습니다. 동시에 Homeport에서 편집하고 있었다면, 어느 쪽 변경도 모르게 사라지지 않도록 **파일을 연 이후 Homeport 밖에서 … 파일이 변경되었습니다**와 함께 저장이 거부됩니다. 다시 불러와서 편집을 다시 하거나, **덮어쓰기**를 선택하여 내 변경을 유지하세요. [링크된 스택 편집](/docs/guides/stacks/#링크된-스택-편집)을 참고하세요.

### 비공개 이미지 풀이 실패함

**설정 → 레지스트리**에서 이미지 레지스트리의 로그인을 추가하고, **로그인 확인**으로 레지스트리가 받아들이는지 확인하세요. GHCR과, 2단계 인증을 쓰는 Docker Hub 계정에는 비밀번호 대신 액세스 토큰이 필요합니다. 풀 대화 상자에 어떤 로그인을 쓸지 표시됩니다. **익명으로**라고 표시되면 이미지의 레지스트리가 저장된 로그인과 일치하지 않는 것입니다. `ghcr.io/org/app`에는 `ghcr.io` 로그인이, `org/app`에는 Docker Hub 로그인이 필요합니다.

### 빌드가 실패함

메시지에 실패한 단계가 나옵니다. 예를 들면 **빌드가 3/5단계(RUN make)에서 실패했습니다**입니다. **세부 정보**에 빌드 출력의 끝부분이 있습니다. Homeport는 엔진의 기본 빌더를 사용하므로 BuildKit이 필요한 Dockerfile(`RUN --mount`, `--secret`, heredoc)은 실패합니다. 이런 이미지는 터미널에서 빌드하세요. 비공개 `FROM` 이미지에는 **설정 → 레지스트리**에 그 레지스트리의 로그인이 필요합니다.

### 새 이미지 표시가 나타나지 않음

표시는 레지스트리에 물어서 응답을 받았을 때만 나타납니다. 로컬에서 빌드했거나 다이제스트로 풀한 이미지에는 표시가 나타나지 않습니다. 레지스트리에 연결할 수 없거나 요청이 제한되면(Docker Hub는 이 확인을 풀 제한에 포함합니다) Homeport는 [로그](/docs/reference/security/#로그)에 한 줄을 기록하고 아무것도 표시하지 않습니다. 응답은 6시간 동안 보관되므로, 지금 다시 물으려면 **새 이미지 확인**을 선택하세요. **설정 → 레지스트리**에 Docker Hub 로그인을 추가하면 제한이 늘어납니다.

### Docker Hub 검색 시 오류가 표시됨

검색은 선택한 호스트에서 실행되므로 그 호스트의 인터넷 접근이 필요하고, 태그는 이 컴퓨터가 `hub.docker.com`에서 읽습니다. 어느 쪽이든 요청이 제한되거나 프록시에 막힐 수 있습니다. 그래도 대화 상자는 `postgres:17-alpine`처럼 입력한 전체 참조를 풀합니다.

### 파일 탭에 컨테이너에 find/stat 도구가 없다고 표시됨

최소 이미지와 distroless 이미지에는 Homeport가 파일 목록을 만들 때 쓰는 `find`와 `stat`이 없습니다. 이런 컨테이너에는 `docker cp`를 사용하세요.

### 터미널이 열리지 않음

컨테이너가 실행 중이어야 하고 `/bin/sh`나 `/bin/bash`가 있어야 합니다. 다른 프로그램을 실행하려면 **새 터미널**을 사용하세요.

### 볼륨이나 네트워크가 삭제되지 않음

컨테이너가(중지된 컨테이너라도) 사용하는 동안에는 엔진이 거부합니다. 먼저 그 컨테이너를 삭제하세요. 확인 대화 상자에 컨테이너가 나열됩니다.

### “Podman 5.8 이상이 필요합니다”

Podman API는 Podman 5.8부터 [Quadlet 유닛](/docs/guides/quadlet/)을 읽고, 설치하고, 편집하고, 삭제하며, 5.7부터 나열합니다. 호스트의 Podman을 업데이트하거나, 그 호스트의 터미널에서 `podman quadlet`을 사용하세요. 유닛 시작과 중지는 모든 Podman에서 작동합니다.

### Quadlet: 시작과 중지가 비활성화됨

Homeport는 `systemctl`로 유닛의 서비스를 시작하고 중지하는데, 이 명령은 SSH를 통해서나 이 컴퓨터(Linux)의 Podman에서만 실행할 수 있습니다. TCP나 TLS 대신 SSH로 호스트에 연결하세요. 서비스가 **아직 생성되지 않음**으로 표시되는 유닛은 시작할 것이 없습니다. systemd의 생성기가 그 파일을 건너뛰었습니다.

### Quadlet: “호스트에서 사용자의 systemd가 실행 중이 아닙니다”

사용자 유닛(rootless Podman)에는 사용자의 systemd 인스턴스가 필요하며, lingering이 켜져 있지 않으면 사용자가 로그인해 있는 동안에만 실행됩니다. 호스트에서 그 사용자로 `loginctl enable-linger "$USER"`를 실행한 다음 다시 시도하세요.

### Quadlet: “systemd가 이 사용자의 … 유닛 관리를 허용하지 않았습니다”

시스템 유닛(rootful Podman)은 `root`처럼 관리가 허용된 사용자만 시작하고 중지할 수 있으며, Homeport는 `sudo`를 절대 사용하지 않습니다. 그런 사용자로 연결하거나, 유닛이 사용자 자신의 것인 rootless Podman을 사용하세요.

### containerd: “/run/containerd/containerd.sock의 containerd를 사용할 권한이 없습니다”

containerd의 소켓은 root 소유입니다. `/etc/containerd/config.toml`의 `[grpc]` 아래 `gid`로 그룹에 접근 권한을 주고, 사용자(또는 SSH 사용자)를 그 그룹에 추가한 다음, containerd를 다시 시작하고 다시 로그인하세요. 명령은 [containerd 페이지](/docs/hosts/containerd/#사용자가-소켓을-쓸-수-있게-하기)에 있습니다. rootless containerd에는 전혀 연결할 수 없습니다. 소켓이 RootlessKit의 네임스페이스 안에만 있기 때문입니다.

### containerd: “이 컨테이너의 출력은 기록되지 않습니다”

containerd는 로그를 보관하지 않으며, 태스크를 시작한 도구가 출력의 행방을 정합니다. `ctr`로 만든 컨테이너는 보통 출력을 아무 데도 보내지 않습니다. nerdctl과 Kubernetes의 컨테이너는 Homeport가 읽는 파일에 로그를 남깁니다. [로그](/docs/hosts/containerd/#로그)를 참고하세요.

### containerd: 로그나 터미널에 SSH가 필요하다고 표시됨

로그와 터미널에는 containerd가 있는 컴퓨터의 파일이 필요하며, Homeport는 SSH로, 또는 Linux의 로컬 소켓이라면 이 컴퓨터에서 접근합니다. SSH로 호스트를 추가하세요. 로그 파일을 읽을 수 없다면 보통 root 소유이므로, 읽기가 허용된 사용자로 연결하세요.

### containerd: “Homeport를 위해 이미지를 풀할 수 없습니다”

풀은 containerd 2.0부터 안정된 containerd의 Transfer 서비스를 통합니다. containerd를 업데이트하거나, 호스트에서 `ctr images pull`이나 `nerdctl pull`로 풀하세요. 그러면 이미지 페이지에 이미지가 나타납니다.

### Incus: “/var/lib/incus/unix.socket의 Incus를 사용할 권한이 없습니다”

Incus의 관리자 소켓은 `incus-admin` 그룹용이고, 제한된 `unix.socket.user`는 `incus` 그룹용입니다. `sudo usermod -aG incus-admin "$USER"`로 사용자(또는 SSH 사용자)를 추가한 다음, 로그아웃했다가 다시 로그인하거나 SSH로 다시 연결하세요. [Incus의 소켓](/docs/hosts/incus/#incus의-소켓)을 참고하세요.

### Incus: 신뢰 토큰으로 추가할 때 “Homeport가 …에서 Incus에 연결하지 못했습니다”

`core.https_address`를 설정하기 전에는 Incus가 네트워크에서 수신 대기하지 않습니다. 서버에서 `incus config set core.https_address :8443`을 실행하고, 방화벽이 8443 포트를 허용하는지 확인한 다음, 새 토큰을 만드세요. 토큰에는 만들 당시 Incus가 수신 대기하던 주소가 들어 있습니다.

### Incus: “Incus가 신뢰 토큰을 거부했습니다”

토큰은 한 번만 쓸 수 있고 만료됩니다. 서버에서 `incus config trust add homeport`로 새 토큰을 만들고 유효 기간 안에 붙여 넣으세요.

### Incus: “…에서 제시한 인증서의 지문은 …입니다. 신뢰하는 지문은 …입니다”

서버의 인증서가 Homeport가 신뢰한 인증서가 아닙니다. Incus를 다시 설치했거나, 인증서를 갱신했거나, 그 주소에서 다른 무언가가 응답하고 있습니다. 서버에서 `incus info`로 지문을 확인하고, 표시된 지문과 같다면 호스트를 삭제한 다음 새 토큰으로 다시 추가하세요. [고정된 인증서](/docs/hosts/incus/#서버-인증서-고정)를 참고하세요.

### Incus: “이 호스트는 가상 머신을 실행할 수 없습니다”

Incus에서 가상 머신을 쓰려면 KVM(`/dev/kvm`)과 QEMU가 필요합니다. 서버의 펌웨어에서 가상화를 켜거나(가상 서버라면 중첩 가상화), 대신 시스템 컨테이너를 만드세요.

### Incus: “가상 머신의 Incus 에이전트가 실행 중이 아닙니다”

가상 머신 안의 Exec는 그 안의 `incus-agent`를 통하며, Incus 자체 이미지는 부팅할 때 에이전트를 시작합니다. 부팅이 끝날 때까지 기다리거나, 에이전트가 필요 없는 **콘솔** 탭을 사용하세요.

### “… 호스트가 연결되어 있지 않습니다”

작업에 호스트가 필요했지만 연결이 끊겼습니다. 다시 연결될 때까지 기다리거나 **지금 다시 시도**를 선택하세요.
