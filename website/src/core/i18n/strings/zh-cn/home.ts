/** The home page, in Simplified Chinese. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport：一个应用，管理所有容器引擎',
  'home.hero.title': '一个应用，管理所有容器引擎。',
  'home.hero.lead': 'Homeport 运行在你的电脑上，不在服务器上安装任何东西。免费、开源。',
  'home.hero.docs': '阅读文档',
  'home.map.label':
    '以线路图形式画出的一组示例服务器。这台电脑位于五条线路的终点：本地套接字通往 This Mac，SSH 通往 prod-eu-1、nas 和 edge-1，TLS 通往 staging 和 vps-fra-1，TCP 通往 lan-box，集群 API 通往 k8s-prod、jobs 和 lab。vps-fra-1 上有 1 个堆栈需要你处理。',
  'home.map.thisMachine': '这台电脑',
  'home.map.needsYou': '1 个堆栈需要你处理',
  'home.map.keyNeedsYou': '待处理',
  'home.map.example': '一组示例服务器',

  // Home: the questions people open it for.
  'home.questions.title': '一个问题，一个去处。',
  'home.questions.lead':
    '你打开 Homeport 是为了弄清某件事，弄清之后就回去继续工作。每个答案都离看板只有一步。',
  'home.questions.up.title': '它在运行吗？',
  'home.questions.up.body':
    '看板同时显示每个主机、Swarm 和集群，并把待处理的事项排在一切正常的之前。',
  'home.questions.up.link': '看板与概览',
  'home.questions.up.screenshot':
    '看板：三个主机各一个磁贴，一个需要你处理的容器，下面是一张汇总所有主机容器的表。',
  'home.questions.died.title': '它为什么停了？',
  'home.questions.died.body':
    '打开一个容器，它的日志已经在实时输出。终端、文件、实时图表和 inspect 文档，切换一个标签页就能看到。',
  'home.questions.died.link': '使用容器',
  'home.questions.died.screenshot': '容器的“日志”标签页，实时输出，带搜索、时间戳和输出流筛选。',
  'home.questions.changed.title': '什么变了？',
  'home.questions.changed.body':
    '每个主机都会持续记录其引擎做过的事：重启的容器、拉取的镜像、挂载的卷。',
  'home.questions.changed.link': '事件',
  'home.questions.changed.screenshot':
    '主机的“事件”页面：健康检查、重启、拉取和挂载，最新的在前，并注明各自所属的堆栈。',
  'home.questions.act.title': '然后再修改，并先看到变更。',
  'home.questions.act.body':
    '重新部署会在执行前显示将发生的变更。每次部署都会保留，可以回滚；“更新”一步拉取较新的镜像。',
  'home.questions.act.link': '管理堆栈',
  'home.questions.act.screenshot':
    '托管堆栈的定义：compose.yaml 编辑器、它的环境变量和“部署”按钮。',
  'home.questions.background.title': '在后台留意',
  'home.questions.background.body':
    '系统托盘中显示每台主机的状态，容器崩溃或主机掉线时发出通知；Homeport 可在登录时启动，隐藏在后台，直到你需要它。',
  'home.questions.keyboard.title': '键盘优先',
  'home.questions.keyboard.body':
    '用命令面板前往任意位置、切换主机或运行命令，并在任何主机上找到任意容器、镜像或卷。每个页面都有快捷键，按 ? 查看列表。',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': '什么也不入驻。',
  'home.lines.lead':
    'Homeport 留在你的电脑上，只是到访你的服务器。无需安装代理程序，没有需要暴露的 Web 面板，无需创建账户，也没有遥测。',
  'home.lines.line': '线路',
  'home.lines.reaches': '连接到',
  'home.lines.needs': '服务器上需要什么',
  'home.lines.socket.name': '本地套接字',
  'home.lines.socket.reaches':
    '这台电脑上的 Docker Desktop、OrbStack、Colima、Rancher Desktop 和 Podman machine，自动发现。',
  'home.lines.socket.needs': '无。引擎已在这台电脑上。',
  'home.lines.ssh.reaches':
    '服务器上的 Docker Engine、Podman、containerd 和 Incus，必要时经由堡垒机。',
  'home.lines.ssh.needs': '你已经开放的端口。使用 Homeport 生成的密钥时，只需在 {file} 中加一行。',
  'home.lines.tls.reaches': 'Docker 兼容引擎的 API，使用你的 CA 和客户端证书。',
  'home.lines.tls.needs': '引擎自己的 API 端口（通常为 {port}），需由你自行开放。',
  'home.lines.tcp.reaches': 'Docker 兼容引擎的 API，不加密。',
  'home.lines.tcp.needs': '引擎的 API 端口（通常为 {port}）。仅限在你信任的网络中使用。',
  'home.lines.cluster.name': '集群 API',
  'home.lines.cluster.reaches':
    '来自 kubeconfig 的 Kubernetes，按代理地址连接的 Nomad，以及通过 HTTPS 和信任令牌连接的 Incus。',
  'home.lines.cluster.needs': '集群本来就提供的 API。节点上不安装任何东西。',
  'home.lines.advice': '条件允许时请使用 SSH：它不会给服务器增加任何你尚未运行的东西。',
  'home.lines.import':
    '已经写好服务器配置了？可以从 {file} 导入（包括堡垒机），或从你的 Docker 上下文导入。',
  'home.lines.link': '连接的工作原理',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': '每种引擎，同一套说法。',
  'home.engines.lead':
    '无论由哪种引擎运行，主机就是主机，堆栈就是堆栈。某种引擎有独有的功能时，Homeport 也会显示出来。',
  'home.engines.docker.body':
    '容器、镜像、卷、网络和 Compose 堆栈：保存在 Homeport 中、关联到文件夹，或从 git 仓库同步。',
  'home.engines.swarm.body':
    '节点、服务、任务、机密和配置，通过其管理节点主机发现。可将堆栈部署到 Swarm。',
  'home.engines.podman.body':
    'Podman Pod，以及在 systemd 下安装并启动的 Quadlet 单元，全部通过 Podman 自己的 API 完成。',
  'home.engines.containerd.body':
    'containerd 命名空间、任务、日志、终端，以及占用磁盘的快照，无需 Docker。',
  'home.engines.incus.body': '系统容器和虚拟机：它们的控制台、快照、镜像、配置文件、存储和网络。',
  'home.engines.kubernetes.body':
    '实时查看工作负载、Pod、Service 及任意 API 资源。日志、终端、端口转发、带预览的 YAML、Helm Release。',
  'home.engines.nomad.body':
    '根据 HCL 及其执行计划运行的 job、它们的 allocation 和日志、task 中的 shell，以及节点腾空。',

  // Home: security.
  'home.security.title': '你的密钥始终加密保存。',
  'home.security.lead': 'Homeport 保管着打开你服务器的凭据，因此会以相应的方式对待它们。',
  'home.security.rest.title': '静态加密',
  'home.security.rest.body':
    'SSH 密钥、TLS 证书、堆栈环境变量和仓库令牌都使用 XChaCha20-Poly1305 加密。密钥保存在系统钥匙串中，而不是放在数据旁边。',
  'home.security.disk.title': '从不以明文写入磁盘',
  'home.security.disk.body':
    'Homeport 自己实现 SSH，因此解密后的密钥只留在内存中。没有密钥文件，也不会改动你的 {dir}。',
  'home.security.home.title': '不向外发送任何数据',
  'home.security.home.body':
    '无需账户，没有遥测，没有跟踪。每个版本都公开构建，附带校验和与构建来源证明。',
  'home.security.link': '安全与你的数据',

  // Home: the close, where the visitor starts.
  'home.close.title': '从这台电脑开始。',
  'home.close.body':
    '在你工作用的电脑上下载 Homeport，然后添加你的主机。它免费，依据 GNU AGPL 开源，并在 GitHub 上公开开发。',
  'home.close.github': '在 GitHub 上查看',
  'home.desktops.windows.versions': '10 和 11，x64 和 ARM64',
  'home.desktops.windows.formats': '安装程序（.exe）或 MSI',
  'home.desktops.macos.versions': 'Apple 芯片和 Intel',
  'home.desktops.macos.formats': '磁盘映像（.dmg）',
  'home.desktops.linux.versions': 'x64 和 ARM64',
  'home.desktops.linux.formats': 'apt、dnf、zypper 或 AppImage',
};
