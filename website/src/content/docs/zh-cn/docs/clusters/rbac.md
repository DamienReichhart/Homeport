---
title: RBAC
description: 查看集群的 Role、绑定和主体，每个主体能在哪里做什么，以及你自己的凭据能做什么。
---

Kubernetes 集群侧边栏中的 **RBAC** 显示谁能在集群上做什么。它是只读的：Homeport 不会创建、更改或删除 RBAC 对象（你仍可以在 [API 资源](/docs/clusters/api-resources/) 中以 YAML 编辑它们）。

和其他页面一样，它跟随 Namespace 选择器：选择了某个 Namespace 时，它显示该 Namespace 的 Role 和 RoleBinding，以及在所有地方生效的 ClusterRole 和 ClusterRoleBinding。

## Role

**Role** 列出 Role（各自在其 Namespace 中）和 ClusterRole，以及它们的规则数量。打开一个可查看它的页面：

- 它的**规则**，按 Kubernetes 的写法：资源、API 组（core 组写作 `core`）、资源名称和动词，或 `/healthz` 这样的非资源 URL；
- **使用它的绑定**，每个都链接到其主体。

标记为**聚合**的 ClusterRole 带有 `aggregationRule`：它的页面显示其标签选择器、这些选择器匹配的 ClusterRole，它的规则也包含这些 ClusterRole 的规则。Kubernetes 自带的 `admin`、`edit` 和 `view` 就是这样构成的，因此带有 `rbac.authorization.k8s.io/aggregate-to-edit: "true"` 标签的 CRD 的 ClusterRole 会出现在 `edit` 中。

## Binding

**Binding** 列出 RoleBinding 和 ClusterRoleBinding，每个都附有它授予的角色（带链接）以及它指定的主体：Users、Groups 和 ServiceAccounts。所引用角色不存在的绑定会标为**未找到**；它不授予任何权限。

## 主体

**主体**列出所有被某个绑定指定的对象。打开一个 User、Group 或 ServiceAccount，可以看到它能做什么：一个资源 × 动词的矩阵，按生效范围分组，先是**整个集群**，再是各个 Namespace。下方是授予这些权限的绑定，以及方式：直接指定它，或通过某个组。

Homeport 会合并每个适用绑定的规则：

- ClusterRoleBinding 在所有地方授予其 ClusterRole：每个 Namespace、Node 等集群范围的资源，以及非资源 URL。
- RoleBinding 只在它自己的 Namespace 中授予其角色，即使它绑定的是 ClusterRole。通过 `prod` 中的 RoleBinding 绑定到 `edit` 的 ServiceAccount 可以在 `prod` 中编辑，在其他地方都不行。
- Kubernetes 自动把每个主体放入的组，其绑定也计算在内：ServiceAccount 属于 `system:serviceaccounts`、`system:serviceaccounts:<它的 Namespace>` 和 `system:authenticated`，同时也是用户 `system:serviceaccount:<Namespace>:<名称>`；User 属于 `system:authenticated`。
- `*` 表示任意动词、API 组或资源，`*/scale` 表示任意资源的该子资源，带资源名称的规则只适用于这些对象（矩阵在**仅限**下显示它们）。已被更宽泛的行覆盖的行会被省略。

**这是仅供参考的视图。** 主体的权限是根据你的凭据能读取的绑定计算出来的，而不是向 API 服务器询问的，因此不需要冒充任何人的权限。它可能看不到其他授权器允许的内容，例如 webhook（托管集群中很常见）或 ABAC，也看不到认证器赋予用户的组（来自 OIDC claims 或客户端证书的组织），这些只有 API 服务器知道。要确认你自己的凭据，请使用**我在这里能做什么？**。

选择了某个 Namespace 时，只读取该 Namespace 的 RoleBinding；选择**所有 Namespace** 可以查看主体在所有地方的权限。

## 我在这里能做什么？

**我在这里能做什么？** 是 API 服务器本身针对 Homeport 所用凭据给出的回答，范围是所选的 Namespace（选择**所有 Namespace** 时为集群的默认 Namespace），与 `kubectl auth can-i --list` 显示的一致。在会告知这些信息的集群上（Kubernetes 1.28 及更高版本），它会说明这些凭据以什么身份登录、属于哪些组，并链接到该主体的页面。

当 API 服务器表示其列表可能不完整时——webhook 之类的授权器可能允许超出其能列出的内容——页面会提示。

**我能……吗？** 像 `kubectl auth can-i` 一样提出一个问题：一个动词、一个资源和一个 Namespace（留空表示所有 Namespace），以及可选的 API 组、子资源和对象名称。资源可以是复数形式（`secrets`）、简称（`deploy`），或带组的复数形式（`deployments.apps`）；Homeport 会像 kubectl 一样解析它。回答附带同一个问题的命令，可供复制，例如 `kubectl auth can-i delete secrets --namespace prod`，它会给出相同的回答。

当你的凭据完全无法列出 RBAC 对象时，页面只显示**我在这里能做什么？**，上方的消息说明被拒绝的内容。当只能列出其中一部分时（例如能列出 RoleBinding 但不能列出 ClusterRole），页面会说明是哪些，显示的内容也不包括它们。
