---
title: API 资源
description: 浏览集群提供的每种对象类型（包括自定义资源），并显示与 kubectl get 相同的列。
---

Homeport 为工作负载、Pod、Service、Ingress、ConfigMap、Secret、Node 和事件提供了专门的页面。集群侧边栏中的 **API 资源**涵盖其余的一切：PersistentVolume 及其 claim、NetworkPolicy、ServiceAccount 和 RBAC，以及 operator 通过其 CRD 添加的资源——cert-manager 的 Certificate、Argo CD 的 Application、Prometheus 的 ServiceMonitor。

## 查找类型

左栏列出集群提供的每种类型，按 API 组分组（`core` 在最前），每个组使用其首选版本，与 `kubectl api-resources` 相同。可以按 kind、复数名或组搜索，也可以输入简称：`pvc`、`netpol`、`sa`、`cert`。

锁形图标标记你无法列出的类型：你的凭据不允许在所选 namespace 中列出的类型（由集群通过 `SelfSubjectRulesReview` 告知 Homeport），以及少数其 API 根本无法列出的类型。打开其中一个会说明原因。有些集群（例如授权器为 webhook 的集群）无法告知这些信息；此时不会标记任何类型，被拒绝的列表会说明这一点。

安装或删除 CRD 会自动更新该列表；**刷新**会重新读取它。

## 列出对象

选择一种类型后，其对象会以与 `kubectl get` 相同的列列出：由 API server 决定显示哪些列，包括 CRD 的 `additionalPrinterColumns`，因此 cert-manager 的 Certificate 会像在终端中一样显示 **Ready**、**Secret** 和 **Age**。列表通过 watch 实时跟进更改。

带 namespace 的类型提供 namespace 选择器，与 Homeport 的其他页面相同；选择**所有 Namespace**时，会添加一个 **Namespace** 列。

## 对象的页面

点击一个对象可打开其页面：

- **概览**：其 namespace、拥有它的对象（链接到它们的页面）、存在时间、标签和注解。
- **条件**：当其 `status` 中有条件时显示，与 Homeport 自己的页面相同。
- 关于它的**事件**，以及其 **YAML**（不含 `managedFields`）。

当其类型允许时，**编辑**会打开其 YAML，以便通过预览和服务器端应用来更改它（参见[应用和编辑 YAML](/docs/clusters/yaml/)），**删除**会在一次注明该对象和集群的确认之后将其删除（参见[删除](/docs/clusters/actions/#删除)）。在列表中，勾选多个对象可将它们一起删除。
