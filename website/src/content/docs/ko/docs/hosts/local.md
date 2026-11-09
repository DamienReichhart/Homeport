---
title: 로컬 엔진
description: 이 컴퓨터의 Docker Engine, Docker Desktop, Colima, OrbStack, Rancher Desktop 또는 Podman에 연결합니다.
---

**로컬 소켓** 호스트는 Unix 소켓(Windows에서는 명명된 파이프)을 통해 이 컴퓨터의 엔진과 통신합니다. 자격 증명도 키체인도 필요하지 않습니다.

## 자동 감지

Homeport는 일반적인 위치에서 실행 중인 엔진을 찾고, 응답하는 엔진을 시작 화면과 **호스트 추가** 페이지의 **이 컴퓨터에서 찾음** 아래에 나열합니다. 옆의 **추가**를 선택하면 그 이름을 딴 호스트가 만들어집니다.

| 런타임                  | macOS 및 Linux                                                                                              | Windows                           |
| ----------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Docker Engine           | `/var/run/docker.sock`                                                                                      | —                                 |
| Docker Engine(rootless) | `$XDG_RUNTIME_DIR/docker.sock`                                                                              | —                                 |
| Docker Desktop          | `~/.docker/desktop/docker.sock`, `~/.docker/run/docker.sock`                                                | `//./pipe/docker_engine`          |
| Colima                  | `~/.colima/default/docker.sock`                                                                             | —                                 |
| OrbStack                | `~/.orbstack/run/docker.sock`                                                                               | —                                 |
| Rancher Desktop         | `~/.rd/docker.sock`                                                                                         | —                                 |
| Podman                  | `$XDG_RUNTIME_DIR/podman/podman.sock`, `/run/podman/podman.sock`. [macOS의 머신](#podman-machines-on-macos) | `//./pipe/podman-machine-default` |

각 후보는 엔진에 버전을 물어 확인하므로 실행 중인 엔진만 나열됩니다. 여러 경로가 같은 소켓으로 이어지는 경우(macOS에서는 `/var/run/docker.sock`이 보통 Docker Desktop, OrbStack 또는 Rancher Desktop을 가리킵니다) 엔진은 처음 찾은 이름으로 한 번만 나열됩니다.

### macOS의 Podman 머신

macOS에서 Podman은 VM(`podman machine`) 안에서 엔진을 실행하며, 소켓 위치는 머신마다 다릅니다. Homeport는 `podman` CLI에 실행 중인 모든 머신의 소켓을 물어(`podman machine list`와 `podman machine inspect`) 각각을 **Podman(_머신 이름_)** 형식으로 나열합니다. 예를 들면 **Podman(podman-machine-default)** 같은 이름입니다. 중지된 머신은 나열되지 않습니다. `podman machine start`로 시작한 다음 **호스트 추가**를 다시 여세요.

Homeport가 `podman`을 실행할 수 없으면(PATH에도, `/opt/homebrew/bin`이나 `/usr/local/bin`에도 없는 경우) Podman이 소켓을 두는 위치를 대신 찾습니다. Podman 5는 `$TMPDIR/podman/<machine>-api.sock`, Podman 4는 `~/.local/share/containers/podman/machine/…/podman.sock`입니다. `podman-mac-helper`가 `/var/run/docker.sock`을 머신에 연결해 두었다면 그 머신은 Podman 이름으로 한 번만 나열됩니다.

## 소켓 직접 추가

엔진이 감지되지 않으면(예: `default`가 아닌 Colima 프로필, Windows에서 기본 이름이 아닌 Podman 머신, 사용자 지정 경로) 직접 추가하세요.

1. 호스트 전환기에서 **호스트 추가…** 항목을 선택합니다.
2. **로컬 소켓**을 선택합니다.
3. **이름**과 **소켓 경로**를 입력합니다. `/run/user/1000/docker.sock` 같은 Unix 소켓이나, Windows에서는 `//./pipe/docker_engine` 같은 명명된 파이프입니다.
4. **호스트 추가**를 선택합니다.

Docker CLI가 사용하는 소켓을 찾으려면 다음을 실행합니다.

```sh
docker context inspect --format '{{.Endpoints.docker.Host}}'
```

Podman 머신이라면 다음을 실행합니다.

```sh
podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}'
```

## Linux의 권한

기본 소켓 `/var/run/docker.sock`은 `docker` 그룹에 속합니다. 사용자가 이 그룹에 없으면 호스트가 _Permission denied_ 오류와 함께 **연결할 수 없음**으로 표시됩니다. 그룹에 자신을 추가하고 다시 로그인하세요.

```sh
sudo usermod -aG docker $USER
```

`docker` 그룹 구성원은 그 컴퓨터에서 root와 같은 권한을 가집니다. 이것이 문제가 된다면 rootless Docker나 Podman을 사용하세요.
