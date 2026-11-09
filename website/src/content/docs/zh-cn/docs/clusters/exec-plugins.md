---
title: Exec 插件
description: Homeport 如何通过 kubeconfig 中指定的 exec 凭据插件登录 EKS、GKE 和 AKS 集群，以及什么在哪里运行。
---

云 kubeconfig 中没有密码或证书：其用户指定了一个 **exec 插件**，即本机上的一个程序，被调用时会输出一个短期有效的令牌。Homeport 会在本机上运行 `kubectl` 会运行的同一个程序，遵循 Kubernetes `ExecCredential` 协议（`client.authentication.k8s.io/v1` 和 `v1beta1`）。

## 常见提供商

| 提供商 | kubeconfig 的用户运行的命令          | 安装                                                                          |
| ------ | ------------------------------------ | ----------------------------------------------------------------------------- |
| EKS    | `aws eks get-token --cluster-name …` | AWS CLI 第 2 版                                                               |
| GKE    | `gke-gcloud-auth-plugin`             | Google Cloud CLI，然后运行 `gcloud components install gke-gcloud-auth-plugin` |
| AKS    | `kubelogin get-token …`              | `az aks install-cli`，或从 kubelogin 的发布页面获取                           |

像往常一样写入 kubeconfig——`aws eks update-kubeconfig`、`gcloud container clusters get-credentials`、`az aks get-credentials` 之后再运行 `kubelogin convert-kubeconfig`——然后[添加其上下文](/docs/clusters/)。

## 什么在哪里运行

- 插件**在本机上**以你的身份运行，使用 kubeconfig 提供给它的参数和环境变量（例如 `AWS_PROFILE`）。Homeport 绝不会要求它进行交互，也不会向它提供任何输入。
- 桌面启动器启动 Homeport 时使用的 `PATH` 很短，因此 Homeport 还会在 AWS、Google Cloud 和 Azure CLI 以及 kubelogin 的常见安装位置中查找（Homebrew、SDK 自身的文件夹、`~/.local/bin`、`~/.krew/bin`、Program Files…）。在 kubeconfig 中以完整路径指定的插件会从该路径运行。
- 它输出的令牌或证书**只保存在内存中**，直到即将过期为止；届时插件会再次运行。它在每个令牌的有效期内只运行一次，而不是每个请求运行一次。当 API 拒绝某个令牌时，Homeport 会先向插件请求一次新令牌，然后才显示**访问被拒绝**。
- 插件自身的缓存（`~/.kube/cache`、`~/.aws/sso/cache`、gcloud 配置）由它自己管理；Homeport 不会向其中写入任何内容。

## 无法登录时

- **“本机上未安装 …。”**——找不到该程序。请安装它（消息会说明安装方法，如果 kubeconfig 中有 `installHint` 则使用它），在终端中确认它能正常工作，然后选择**重新连接**。
- **“… 需要你登录，但它无法在 Homeport 中完成登录。”**——插件需要浏览器或设备代码（AWS SSO、Azure 设备登录、已过期的 `gcloud` 会话）。请在终端中用该上下文运行一次 `kubectl`，在那里登录，然后选择**重新连接**：插件随后会找到其缓存的登录信息。
- **“… 无法获取 … 的凭据。”**——它运行了但失败了；**详细信息**会显示它最后输出的内容。Homeport 会自行重试。

其他消息参见[故障排除](/docs/reference/troubleshooting/#clusters)。
