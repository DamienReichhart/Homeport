---
title: CPU와 메모리
description: metrics-server로 Pod와 그 컨테이너, Node가 쓰는 CPU와 메모리를 요청, 제한, 할당 가능량과 비교해 확인합니다.
---

클러스터에서 [metrics-server](https://github.com/kubernetes-sigs/metrics-server)가 실행 중이면, Homeport는 각 Pod와 각 Node가 쓰는 CPU와 메모리를 목록과 각 페이지의 실시간 그래프에 표시합니다. `kubectl top`처럼 metrics-server가 제공하는 Metrics API(`metrics.k8s.io`)를 읽습니다.

## 필요한 것

metrics-server가 이미 클러스터에서 실행 중이어야 합니다. Homeport는 클러스터에 아무것도 설치하지 않습니다. 설치되어 있는지는 클러스터를 만든 방식에 따라 다릅니다.

| 클러스터                | metrics-server                                                       |
| ----------------------- | -------------------------------------------------------------------- |
| GKE                     | 설치됨                                                               |
| AKS                     | 설치됨                                                               |
| k3s와 k3d               | 설치됨(k3s를 `--disable metrics-server`로 시작한 경우 제외)          |
| Docker Desktop          | 해당 Kubernetes와 함께 설치됨                                        |
| kind                    | 설치되지 않음                                                        |
| EKS                     | 설치되지 않음: metrics-server 추가 기능을 추가하거나 직접 설치하세요 |
| minikube                | 설치되지 않음: `minikube addons enable metrics-server`               |
| 기타 클러스터(kubeadm…) | 보통 설치되지 않음                                                   |

직접 설치하려면 [설치 안내](https://github.com/kubernetes-sigs/metrics-server#installation)를 따르세요. kubelet이 자체 서명 인증서를 쓰는 로컬 클러스터(kind)에서는 metrics-server에 `--kubelet-insecure-tls`도 필요합니다.

metrics-server가 없어도 아무것도 망가지지 않습니다. CPU와 메모리 열이 표시되지 않고, Pod나 Node의 **메트릭** 탭에 metrics-server가 설치되어 있지 않다는 안내와 링크가 표시됩니다. 설치 직후의 metrics-server는 샘플이 생기기까지 1분 정도 걸리며, 그동안 탭에는 Metrics API가 아직 응답하지 않는다고 표시됩니다.

자격 증명에 메트릭을 읽을 권한도 있어야 합니다. Namespace(또는 모든 Namespace)의 `pods.metrics.k8s.io`와, Node용 `nodes.metrics.k8s.io`에 대한 `list`와 `get`입니다. 권한이 없으면 **메트릭** 탭에 어떤 권한이 빠졌는지 표시되고, 다른 페이지는 모두 그대로 작동합니다.

## 목록

**Pod**에서 **CPU**와 **메모리** 열은 각 Pod가 컨테이너를 합쳐 쓰는 양과, 그것이 제한(제한이 없으면 요청)의 몇 퍼센트인지 보여 줍니다. 모든 컨테이너에 제한이 있을 때만 Pod에 제한이 있습니다. 제한이 없는 컨테이너는 Node 전체를 쓸 수 있기 때문입니다. CPU는 `kubectl top`처럼 밀리코어 단위(`250m`은 코어의 4분의 1)이고, 메모리는 kubelet이 Pod를 축출할 때 기준으로 삼는 워킹 셋입니다.

**Node**에서 **CPU 사용량**과 **메모리 사용량**은 각 Node의 사용량과, Pod에 **할당 가능**한 양 대비 비율을 보여 줍니다.

열 머리글을 클릭하면 그 열로 정렬됩니다. 부하가 가장 큰 Pod나 Node가 맨 위에 옵니다.

## Pod나 Node의 페이지

Pod의 **메트릭** 탭은 각 컨테이너의 CPU와 메모리를 그래프로 그리고, 설정되어 있으면 요청과 제한을 점선으로 표시합니다. Node의 탭은 CPU와 메모리를 할당 가능량 선과 함께 그립니다.

metrics-server는 15초마다 kubelet을 읽고, Homeport도 같은 간격으로 새 샘플을 요청하므로 부하 변화는 30초 안에 그래프에 나타납니다. 샘플은 어느 탭을 보고 있든 페이지가 열려 있는 동안 최대 15분까지, 메모리에만 보관됩니다. 페이지를 닫으면 읽기가 멈추고 샘플은 버려집니다. Homeport는 메트릭 기록을 저장하지 않습니다. 기록이 필요하면 Prometheus 같은 모니터링 시스템을 쓰세요.
