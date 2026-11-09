---
title: Exec 플러그인
description: Homeport가 kubeconfig에 지정된 exec 자격 증명 플러그인으로 EKS, GKE, AKS 클러스터에 로그인하는 방식과, 무엇이 어디서 실행되는지 설명합니다.
---

클라우드 kubeconfig에는 비밀번호나 인증서가 없습니다. 대신 사용자 항목에 **exec 플러그인** 하나가 지정되어 있습니다. exec 플러그인은 요청을 받으면 수명이 짧은 토큰을 출력하는, 이 컴퓨터의 프로그램입니다. Homeport는 Kubernetes `ExecCredential` 프로토콜(`client.authentication.k8s.io/v1` 및 `v1beta1`)에 따라 `kubectl`이 실행할 것과 같은 프로그램을 이 컴퓨터에서 실행합니다.

## 주요 제공업체

| 제공업체 | kubeconfig의 사용자가 실행하는 명령  | 설치                                                                                   |
| -------- | ------------------------------------ | -------------------------------------------------------------------------------------- |
| EKS      | `aws eks get-token --cluster-name …` | AWS CLI 버전 2                                                                         |
| GKE      | `gke-gcloud-auth-plugin`             | Google Cloud CLI를 설치한 다음 `gcloud components install gke-gcloud-auth-plugin` 실행 |
| AKS      | `kubelogin get-token …`              | `az aks install-cli` 또는 kubelogin 릴리스 페이지에서 설치                             |

평소처럼 kubeconfig를 작성한 다음(`aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, 또는 `az aks get-credentials` 후 `kubelogin convert-kubeconfig`) [컨텍스트를 추가](/docs/clusters/)하세요.

## 무엇이 어디서 실행되는가

- 플러그인은 **이 컴퓨터에서** 사용자 권한으로, kubeconfig가 지정한 인수와 환경 변수(예: `AWS_PROFILE`)로 실행됩니다. Homeport는 플러그인에 대화형 입력을 요구하지 않으며 아무 입력도 주지 않습니다.
- 데스크톱 런처는 짧은 `PATH`로 Homeport를 시작하므로, Homeport는 AWS, Google Cloud, Azure CLI와 kubelogin의 일반적인 설치 위치(Homebrew, SDK 자체 폴더, `~/.local/bin`, `~/.krew/bin`, Program Files…)도 찾아봅니다. kubeconfig에 전체 경로로 지정된 플러그인은 그 위치에서 실행됩니다.
- 플러그인이 출력한 토큰이나 인증서는 만료 직전까지 **메모리에만** 보관되며, 그때 플러그인이 다시 실행됩니다. 플러그인은 요청마다가 아니라 토큰 수명마다 한 번 실행됩니다. API가 토큰을 거부하면 Homeport는 **액세스 거부됨** 상태를 표시하기 전에 플러그인에 새 토큰을 한 번 요청합니다.
- 플러그인 자체의 캐시(`~/.kube/cache`, `~/.aws/sso/cache`, gcloud 구성)는 플러그인이 관리하며, Homeport는 그곳에 아무것도 쓰지 않습니다.

## 로그인할 수 없을 때

- **"… 명령이 이 컴퓨터에 설치되어 있지 않습니다."**: 프로그램을 찾지 못했습니다. 프로그램을 설치하고(메시지에 설치 방법이 나오며, kubeconfig에 `installHint`가 있으면 그 내용을 사용합니다), 터미널에서 작동하는지 확인한 다음 **다시 연결** 버튼을 누르세요.
- **"… 명령은 로그인이 필요한데, Homeport 안에서는 로그인할 수 없습니다."**: 플러그인이 브라우저나 디바이스 코드를 요구합니다(AWS SSO, Azure 디바이스 로그인, 만료된 `gcloud` 세션). 터미널에서 그 컨텍스트로 `kubectl`을 한 번 실행해 로그인한 다음 **다시 연결** 버튼을 선택하세요. 그러면 플러그인이 캐시된 로그인 정보를 찾습니다.
- **"… 명령이 … 클러스터의 자격 증명을 가져오지 못했습니다."**: 플러그인이 실행되었지만 실패했습니다. **세부 정보** 항목에 플러그인 출력의 마지막 부분이 표시됩니다. Homeport가 자동으로 다시 시도합니다.

다른 메시지는 [문제 해결](/docs/reference/troubleshooting/#clusters)을 참고하세요.
