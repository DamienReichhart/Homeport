---
title: API 리소스
description: 사용자 지정 리소스를 포함해 클러스터가 제공하는 모든 종류의 객체를 kubectl get과 같은 열로 살펴봅니다.
---

Homeport에는 워크로드, Pod, Service, Ingress, ConfigMap, Secret, Node, 이벤트를 위한 전용 페이지가 있습니다. 클러스터 사이드바의 **API 리소스** 페이지는 그 밖의 모든 것을 다룹니다. PersistentVolume과 그 클레임, NetworkPolicy, ServiceAccount와 RBAC, 그리고 operator가 CRD로 추가하는 리소스(cert-manager의 Certificate, Argo CD의 Application, Prometheus의 ServiceMonitor)가 여기에 해당합니다.

## 종류 찾기

왼쪽 열에는 `kubectl api-resources`처럼 클러스터가 제공하는 모든 종류가 API 그룹별로(`core`가 먼저) 각 그룹의 기본 버전으로 나열됩니다. 종류, 복수형, 그룹으로 검색하거나 `pvc`, `netpol`, `sa`, `cert` 같은 약칭을 입력하세요.

자물쇠 표시는 목록을 볼 수 없는 종류를 나타냅니다. 선택한 Namespace에서 자격 증명으로 목록을 볼 수 없는 종류(클러스터가 `SelfSubjectRulesReview`로 Homeport에 알려 준 결과)와, API 자체가 목록을 지원하지 않는 몇몇 종류입니다. 이런 종류를 열면 이유가 표시됩니다. 권한 부여자가 webhook인 클러스터처럼 이를 알려 줄 수 없는 클러스터도 있습니다. 이 경우 아무것도 표시되지 않으며, 거부된 목록에서 그 사실을 알려 줍니다.

CRD를 설치하거나 삭제하면 목록이 자동으로 업데이트되며, **새로 고침** 버튼으로 다시 읽을 수 있습니다.

## 객체 나열

종류를 선택하면 그 객체가 `kubectl get`과 같은 열로 나열됩니다. 어떤 열을 표시할지는 API 서버가 정하며 CRD의 `additionalPrinterColumns`도 포함되므로, cert-manager의 Certificate는 터미널에서처럼 **Ready**, **Secret**, **Age** 열을 보여 줍니다. 목록은 watch를 통해 변경 사항을 실시간으로 따라갑니다.

Namespace에 속하는 종류에는 Homeport의 다른 페이지처럼 Namespace 선택기가 있습니다. **모든 Namespace** 항목을 선택하면 **Namespace** 열이 추가됩니다.

## 객체 페이지

객체를 클릭하면 페이지가 열립니다.

- **개요**: Namespace, 객체를 소유한 객체(해당 페이지로 연결), 생성 후 경과 시간, 레이블과 어노테이션.
- **조건**: `status`에 조건이 있을 때, Homeport의 전용 페이지와 같은 방식으로 표시됩니다.
- 객체에 관한 **이벤트** 탭, 그리고 `managedFields`를 제외한 **YAML** 탭.

종류가 허용하면 **편집** 버튼으로 YAML을 열어 미리 보기와 서버 측 적용을 거쳐 변경할 수 있고([YAML 적용과 편집](/docs/clusters/yaml/) 참고), **삭제** 버튼은 객체와 클러스터 이름을 밝히는 확인을 거쳐 객체를 삭제합니다([삭제](/docs/clusters/actions/#삭제) 참고). 목록에서는 여러 객체에 체크하여 함께 삭제할 수 있습니다.
