---
title: CPU 和内存
description: 通过 metrics-server 查看 Pod、其容器和 Node 使用的 CPU 和内存，并与请求、限制和可分配量对照。
---

当集群运行 [metrics-server](https://github.com/kubernetes-sigs/metrics-server) 时，Homeport 会显示每个 Pod 和每个 Node 使用的 CPU 和内存：在列表中，以及在它们页面上的实时图表中。它和 `kubectl top` 一样，读取 metrics-server 提供的 Metrics API（`metrics.k8s.io`）。

## 需要什么

metrics-server 必须已经在集群上运行：Homeport 不会在集群上安装任何东西。是否已安装取决于集群的创建方式：

| 集群                 | metrics-server                                        |
| -------------------- | ----------------------------------------------------- |
| GKE                  | 已安装                                                |
| AKS                  | 已安装                                                |
| k3s 和 k3d           | 已安装（除非 k3s 以 `--disable metrics-server` 启动） |
| Docker Desktop       | 随其 Kubernetes 一起安装                              |
| kind                 | 未安装                                                |
| EKS                  | 未安装：添加 metrics-server 附加组件，或自行安装      |
| minikube             | 未安装：`minikube addons enable metrics-server`       |
| 其他集群（kubeadm…） | 通常未安装                                            |

要自行安装，请按照[它的说明](https://github.com/kubernetes-sigs/metrics-server#installation)操作；在 kubelet 使用自签名证书的本地集群（kind）上，metrics-server 还需要 `--kubelet-insecure-tls`。

没有 metrics-server 也不会出问题：CPU 和内存列不会显示，Pod 或 Node 的 **指标** 标签页会说明未安装 metrics-server，并附上它的链接。刚安装后，metrics-server 大约需要一分钟才会有样本；在此期间，标签页会说明 Metrics API 还没有响应。

你的凭据还需要能读取指标：对 Namespace（或所有 Namespace）中的 `pods.metrics.k8s.io`，以及 Node 的 `nodes.metrics.k8s.io` 拥有 `list` 和 `get` 权限。没有权限时，**指标** 标签页会说明缺少哪项权限，其他页面照常可用。

## 在列表中

在 **Pod** 页面上，**CPU** 和 **内存** 列显示每个 Pod 的用量（其容器之和），以及它占限制的比例；没有限制时则是占请求的比例。只有当所有容器都设置了限制时，Pod 才有限制：没有限制的容器可以用满整个 Node。CPU 以毫核为单位，和 `kubectl top` 的写法一样（`250m` 是四分之一个核），内存是工作集，也就是 kubelet 驱逐 Pod 时依据的值。

在 **Node** 页面上，**CPU 已用** 和 **内存已用** 显示每个 Node 的用量，以及它占 Pod **可分配** 量的比例。

点击列标题即可按该列排序：负载最高的 Pod 或 Node 排在最前。

## 在 Pod 或 Node 的页面上

Pod 的 **指标** 标签页为每个容器绘制 CPU 和内存图表，如果设置了请求和限制，会各用一条虚线标出。Node 的标签页绘制它的 CPU 和内存，并用一条线标出可分配量。

metrics-server 每 15 秒读取一次 kubelet，Homeport 也以相同频率请求新样本，因此负载变化会在 30 秒内出现在图表上。无论你在哪个标签页，样本都会在页面打开期间保留，最多 15 分钟，并且只保存在内存中：关闭页面就会停止读取并丢弃它们。Homeport 不保存任何指标历史；如需历史数据，请使用 Prometheus 等监控系统。
