import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Simplified Chinese. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport：所有 Docker Engine，尽在一块看板。',
  'home.hero.new': '新',
  'home.hero.release': 'Homeport {version} 已发布',
  'home.hero.title': '所有 Docker Engine，尽在一块看板。',
  'home.hero.lead':
    'Homeport 运行在你的电脑上，通过 SSH、TLS 或 TCP 连接你的主机。服务器上无需代理程序。免费、开源，支持 Windows、macOS、Linux。',
  'home.hero.source': '阅读源代码',
  'home.board.caption':
    '一块示例看板：五台主机、它们的引擎，以及 Homeport 连接它们的方式。其中一台需要处理。',
  'home.board.host': '主机',
  'home.board.engine': '引擎',
  'home.board.via': '连接方式',
  'home.board.stacks': '堆栈',
  'home.board.status': '状态',
  'home.board.allRunning': '全部运行中',
  'home.board.needsYou': '1 个堆栈需要你处理',
  'home.board.note': '健康状态不着色。只有需要你处理的才会变红。',
  'home.runtimes.label': '适用于',
  'home.runtimes.title': '适用于你已经在运行的环境',

  // Home: where it lives.
  'home.visits.eyebrow': '它在哪里运行',
  'home.visits.title': 'Homeport 留在你的电脑上，只是到访你的服务器，从不入驻。',
  'home.visits.agent.title': '无需代理程序',
  'home.visits.agent.body':
    '服务器上无需安装、更新或加固任何东西。Homeport 直接与服务器上已有的引擎通信。',
  'home.visits.panel.title': '没有 Web 面板',
  'home.visits.panel.body': '服务器上不部署仪表盘，也没有等着被人发现的登录页面。',
  'home.visits.account.title': '无需账户',
  'home.visits.account.body':
    '无需登录，不按席位收费，没有订阅，也没有遥测。它只显示你的容器，仅此而已。',
  'home.visits.ports.title': '它在服务器上需要什么，如实说明',
  'home.visits.ports.ssh': '你已经开放的端口。使用 Homeport 生成的密钥时，只需在 {file} 中加一行。',
  'home.visits.ports.tls': '引擎自己的 API 端口（通常为 {port}），需由你自行开放。',
  'home.visits.ports.tcp': '引擎的 API 端口（通常为 {port}），不加密。仅限在你信任的网络中使用。',
  'home.visits.ports.advice': '条件允许时请使用 SSH：它不会给服务器增加任何你尚未运行的东西。',
  'home.visits.link': '连接的工作原理',

  // Home: the features.
  'home.features.eyebrow': '功能',
  'home.features.title': '引擎上运行的一切，尽在一个窗口',
  'home.features.lead':
    'Docker 的日常：什么在运行、它输出了什么、占用了多少资源，无需记住它在哪台服务器上。',
  'home.features.hosts.title': '主机，无论远近',
  'home.features.hosts.body':
    '本地引擎和 Podman machine 会被自动发现。远程引擎可通过 SSH（必要时经由堡垒机）、双向 TLS 或 TCP 加入，也可直接从你的 ~/.ssh/config 和 Docker 上下文导入。网络短暂中断时，每台主机都会自行重新连接。',
  'home.features.containers.title': '容器一目了然',
  'home.features.containers.body':
    '按堆栈分组，显示实时 CPU 和内存，以及容器不健康的原因。可设置资源限制、设备和健康检查来运行容器，之后还可编辑、复制、提交或导出容器。',
  'home.features.stacks.title': 'Compose 堆栈',
  'home.features.stacks.body':
    '从模板开始，关联这台电脑上的项目文件夹，或跟踪一个 git 仓库，然后部署到任意主机，包括构建。每次重新部署或同步前都能看到将发生的变更，可以回滚，并一步更新到较新的镜像。',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Swarm 通过其管理节点主机显示。管理节点、服务、任务、机密和配置，将堆栈部署到 Swarm，并在某台主机上创建、加入或退出 Swarm，任何节点上都无需安装任何东西。',
  'home.features.podman.title': 'Podman，按 Podman 的方式',
  'home.features.podman.body':
    'Homeport 在连接时识别 Podman，并补充只有 Podman 才有的功能：Podman Pod（创建时带上端口，并在其中运行容器）和 Quadlet 单元（在 systemd 下安装、编辑和启动），全部通过 Podman 自己的 API 完成。',
  'home.features.containerd.title': '直接使用 containerd',
  'home.features.containerd.body':
    '通过套接字连接 containerd，本地或经由 SSH 均可，无需 Docker：选择 containerd 命名空间，启动、停止其容器的任务并向其发送信号，读取日志，打开终端，使用你的镜像仓库登录拉取镜像，并找出占用磁盘的快照。',
  'home.features.incus.title': 'Incus 实例',
  'home.features.incus.body':
    '通过套接字、SSH 或使用信任令牌的 HTTPS 添加 Incus，运行其系统容器和虚拟机：从远程镜像创建它们，启动、冻结和停止它们，打开其控制台或 shell，创建和恢复快照，并管理镜像、配置文件、存储和网络。',
  'home.features.clusters.title': 'Kubernetes 和 Nomad 集群',
  'home.features.clusters.body':
    '从你的 kubeconfig 添加 Kubernetes 集群（包括 EKS、GKE 和 AKS），或一键找到本地集群。实时浏览工作负载、Pod、Service、配置以及任意 API 资源；跟踪日志、打开终端、转发端口、扩缩，并在预览后应用 YAML；安装、升级和回滚 Helm Chart。按地址添加 Nomad 集群，可根据 HCL 及其执行计划运行 job，扩缩和停止 job，跟踪日志，打开 shell，并腾空节点。集群上不会安装任何东西。',
  'home.features.images.title': '镜像',
  'home.features.images.body':
    '搜索 Docker Hub，使用你的镜像仓库登录拉取和推送，从你电脑上的文件夹构建，保存和加载归档文件，并在有较新镜像发布时得知。',
  'home.features.volumes.title': '卷和网络',
  'home.features.volumes.body':
    '查看哪些容器在使用什么。创建、连接和断开连接；删除时会准确告诉你将失去什么。',
  'home.features.board.title': '一块看板，涵盖所有主机',
  'home.features.board.body':
    '所有主机的所有容器都在一张表中，可筛选，可批量操作。每台主机也有自己的页面：引擎详情、磁盘用量（在一个对话框中完成清理），以及引擎当前操作的实时流。',
  'home.features.keyboard.title': '键盘优先',
  'home.features.keyboard.body':
    '用命令面板前往任意位置、切换主机或运行命令，并在任何主机上找到任意容器、镜像或卷。每个页面都有快捷键，按 ? 查看列表。',
  'home.features.background.title': '在后台留意',
  'home.features.background.body':
    '系统托盘中显示每台主机的状态，容器崩溃或主机掉线时发出通知；Homeport 可在登录时启动，隐藏在后台，直到你需要它。',

  // Home: Hosts.
  'home.hosts.eyebrow': '主机',
  'home.hosts.title': '远程引擎，无需繁琐步骤',
  'home.hosts.body':
    '服务器只需添加一次，就会一直留在看板上。对于 SSH，Homeport 会为该主机单独生成一个密钥，并给你一条用于授权它的命令。首次连接时它会检查服务器的指纹，并在指纹发生变化时提醒你。',
  'home.hosts.import':
    '已经写好配置了？可以从 {file} 导入你的服务器（包括堡垒机），或导入你的 Docker 上下文（包括证书）。换新电脑时，可将它们一起放进一个用口令加密的文件带走。',
  'home.hosts.socket.name': '本地套接字',
  'home.hosts.socket.detail': 'Unix 套接字和 Windows 命名管道，自动检测。',
  'home.hosts.ssh.detail':
    '每台主机一个专用 Ed25519 密钥，或使用你自己的密钥、ssh-agent 或 Pageant；也支持经由堡垒机连接。',
  'home.hosts.tls.detail': '使用你的 CA、客户端证书和密钥的双向 TLS。',
  'home.hosts.tcp.detail': '普通、不加密的 TCP，仅适用于你信任的网络中的引擎。',
  'home.hosts.link': '连接的工作原理',
  'home.hosts.screenshot':
    '选中 SSH 的“添加主机”页面：服务器、端口、用户、远程套接字路径，以及为此主机生成的新密钥。',

  // Home: containers.
  'home.containers.eyebrow': '容器',
  'home.containers.title': '容器的一切，点一下就能看到',
  'home.containers.body':
    '打开一个容器，它的日志已经在实时输出。切换标签页即可使用交互式终端、查看其文件、实时图表、进程和完整的 inspect 文档；位于地球另一端的主机，用起来和你的笔记本电脑一样方便。',
  'home.containers.logs.title': '日志',
  'home.containers.logs.body': '跟踪、搜索、按输出流和时间筛选、下载。',
  'home.containers.terminal.title': '终端',
  'home.containers.terminal.body': '交互式 shell，需要多少开多少。',
  'home.containers.files.title': '文件',
  'home.containers.files.body': '浏览、上传和下载，文件夹以 .tar 形式下载。',
  'home.containers.stats.title': '统计',
  'home.containers.stats.body': 'CPU、内存、网络和磁盘 I/O 图表。',
  'home.containers.link': '使用容器',
  'home.containers.screenshot': '容器的“日志”标签页，实时输出，带搜索、时间戳和输出流筛选。',

  // Home: Stacks.
  'home.stacks.eyebrow': '堆栈',
  'home.stacks.title': 'Compose 堆栈：托管、关联、同步或发现',
  'home.stacks.body':
    '将 compose 文件及其环境变量保存在 Homeport 中，并部署到任意主机：从模板开始，组合多个文件，选择 profile；变量加密存储，从不写入磁盘。或者关联这台电脑上的项目文件夹：Homeport 直接编辑其中的文件并从中部署，包括构建。或者跟踪 git 仓库的某个分支，使用部署密钥或令牌：每次同步都会在部署前显示新的提交及其变更；Homeport 打开期间会检查新提交，如果你要求，还会自行部署。每次重新部署都会先显示将发生的变更，每次部署都会保留以便回滚，“更新”会一步拉取较新的镜像并重新部署。别人手动部署的堆栈也会显示：你可以启动、停止或删除它们，也可以接管它们。',
  'home.stacks.link': '管理堆栈',
  'home.stacks.screenshot':
    '托管堆栈的定义：compose.yaml 编辑器、它的环境变量和“保存并重新部署”按钮。',

  // Home: security.
  'home.security.eyebrow': '安全',
  'home.security.title': '你的密钥始终加密保存',
  'home.security.lead': 'Homeport 掌管着打开你服务器的凭据，因此会以相应的方式对待它们。',
  'home.security.rest.title': '静态加密',
  'home.security.rest.body':
    'SSH 密钥、TLS 证书、堆栈环境变量和仓库令牌都使用 XChaCha20-Poly1305 加密。密钥保存在系统钥匙串中，而不是放在数据旁边。',
  'home.security.disk.title': '从不以明文写入磁盘',
  'home.security.disk.body':
    'Homeport 自己实现 SSH，因此解密后的密钥只留在内存中。没有密钥文件，也不会改动你的 {dir}。',
  'home.security.home.title': '不向外发送任何数据',
  'home.security.home.body':
    '无需账户，没有遥测，没有跟踪。Homeport 只与你的引擎通信。每个版本都公开构建，附带校验和与构建来源证明。',
  'home.security.link': '安全与你的数据',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': '下载',
  'home.desktops.title': '在每种桌面系统上原生运行',
  'home.desktops.lead':
    '一款用 Tauri 和 Rust 构建的小巧、快速的应用，而不是伪装成应用的浏览器标签页。提供 x64 和 ARM 安装程序，并让 Homeport 保持最新。',
  'home.desktops.windows.versions': '10 和 11 · x64 和 ARM64',
  'home.desktops.windows.formats': '安装程序（.exe）· MSI',
  'home.desktops.macos.versions': 'Apple 芯片和 Intel',
  'home.desktops.macos.formats': '磁盘映像（.dmg）',
  'home.desktops.linux.versions': 'x64 和 ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': '服务器上无需安装任何东西。',
  'home.cta.body':
    '在你工作用的电脑上下载 Homeport，然后添加你的主机。它免费，依据 GNU AGPL 开源，并在 GitHub 上公开开发。',
  'home.cta.github': '在 GitHub 上查看',
} satisfies Translation<typeof en>;
