/** The home page, in Brazilian Portuguese. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — Um só app para todos os Engines de contêiner',
  'home.hero.title': 'Um só app para todos os Engines de contêiner.',
  'home.hero.lead':
    'O Homeport roda na sua máquina e não instala nada nos seus servidores. Gratuito e de código aberto.',
  'home.hero.docs': 'Ler a documentação',
  'home.map.label':
    'Uma frota de exemplo desenhada como um mapa de metrô. Esta máquina fica no fim de cinco linhas: o socket local chega a This Mac, o SSH chega a prod-eu-1, nas e edge-1, o TLS chega a staging e vps-fra-1, o TCP chega a lan-box e a API do cluster chega a k8s-prod, jobs e lab. Uma stack em vps-fra-1 precisa de você.',
  'home.map.thisMachine': 'Esta máquina',
  'home.map.needsYou': '1 stack precisa de você',
  'home.map.keyNeedsYou': 'Precisa de você',
  'home.map.example': 'Uma frota de exemplo',

  // Home: the questions people open it for.
  'home.questions.title': 'Uma pergunta, um lugar.',
  'home.questions.lead':
    'Você abre o Homeport para descobrir algo e depois volta ao trabalho. Cada resposta está a um passo do Quadro.',
  'home.questions.up.title': 'Está no ar?',
  'home.questions.up.body':
    'O Quadro mostra todos os hosts, Swarms e clusters de uma vez e lista o que precisa de você antes de tudo o que está bem.',
  'home.questions.up.link': 'O Quadro e a Visão geral',
  'home.questions.up.screenshot':
    'O Quadro: um bloco para cada um de três hosts, um contêiner que precisa de você e depois os contêineres de todos os hosts em uma só tabela.',
  'home.questions.died.title': 'Por que ele caiu?',
  'home.questions.died.body':
    'Abra um contêiner e os logs dele já estão chegando. Um terminal, os arquivos dele, gráficos ao vivo e o documento de inspect estão a uma aba de distância.',
  'home.questions.died.link': 'Trabalhar com contêineres',
  'home.questions.died.screenshot':
    'A aba Logs de um contêiner, transmitindo a saída com pesquisa, carimbos de data/hora e filtros de fluxo.',
  'home.questions.changed.title': 'O que mudou?',
  'home.questions.changed.body':
    'Cada host mantém um registro contínuo do que o Engine dele fez: o contêiner que reiniciou, a imagem que foi baixada, o volume que foi montado.',
  'home.questions.changed.link': 'Eventos',
  'home.questions.changed.screenshot':
    'A página Eventos de um host: verificações de integridade, reinícios, downloads e montagens, os mais recentes primeiro, com a stack a que cada um pertence.',
  'home.questions.act.title': 'Depois mude, e veja a mudança antes.',
  'home.questions.act.body':
    'Uma reimplantação mostra o que vai mudar antes de ser executada. Cada implantação é guardada para que você possa reverter, e Atualizar baixa imagens mais novas em um só passo.',
  'home.questions.act.link': 'Gerenciar stacks',
  'home.questions.act.screenshot':
    'A definição de uma stack gerenciada: o editor do compose.yaml, o ambiente dela e o botão Implantar.',
  'home.questions.background.title': 'De olho em segundo plano',
  'home.questions.background.body':
    'O status de cada host na bandeja do sistema, notificações quando um contêiner falha ou um host cai, e o Homeport iniciando com o login, oculto até você precisar dele.',
  'home.questions.keyboard.title': 'Teclado em primeiro lugar',
  'home.questions.keyboard.body':
    'Uma paleta de comandos para ir a qualquer lugar, trocar de host ou executar um comando, e para encontrar qualquer contêiner, imagem ou volume em qualquer host que o tenha. Atalhos em todas as páginas, listados com ?.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Nada se instala lá.',
  'home.lines.lead':
    'O Homeport fica na sua máquina e visita seus servidores. Nenhum agente para instalar, nenhum painel web para expor, nenhuma conta para criar e nenhuma telemetria.',
  'home.lines.line': 'Linha',
  'home.lines.reaches': 'Alcança',
  'home.lines.needs': 'O que ele precisa no servidor',
  'home.lines.socket.name': 'Socket local',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop e máquinas do Podman neste computador, encontrados para você.',
  'home.lines.socket.needs': 'Nada. O Engine já está nesta máquina.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd e Incus em um servidor, passando por bastions se for preciso.',
  'home.lines.ssh.needs':
    'A porta que você já tem aberta. Com a chave que o Homeport cria, uma linha em {file}.',
  'home.lines.tls.reaches':
    'A API de um Engine compatível com Docker, com a sua CA e o seu certificado de cliente.',
  'home.lines.tls.needs':
    'A própria porta da API do Engine (geralmente {port}), que você mesmo expõe.',
  'home.lines.tcp.reaches': 'A API de um Engine compatível com Docker, sem criptografia.',
  'home.lines.tcp.needs':
    'A porta da API do Engine (geralmente {port}). Só em uma rede em que você confia.',
  'home.lines.cluster.name': 'API do cluster',
  'home.lines.cluster.reaches':
    'Kubernetes do seu kubeconfig, Nomad pelo endereço do agente dele, Incus por HTTPS com um token de confiança.',
  'home.lines.cluster.needs': 'A API que o cluster já oferece. Nada é instalado em nenhum nó.',
  'home.lines.advice':
    'Quando puder, use SSH: ele não acrescenta nada ao servidor que você já não esteja executando.',
  'home.lines.import':
    'Já descreveu seus servidores? Importe-os de {file}, incluindo bastions, ou dos seus contextos do Docker.',
  'home.lines.link': 'Como as conexões funcionam',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Todos os Engines, com as mesmas palavras.',
  'home.engines.lead':
    'Um host é um host e uma stack é uma stack, seja qual for o Engine que a executa. Onde um Engine tem algo próprio, o Homeport mostra isso também.',
  'home.engines.docker.body':
    'Contêineres, imagens, volumes, redes e stacks Compose: guardadas no Homeport, vinculadas a uma pasta ou sincronizadas de um repositório git.',
  'home.engines.swarm.body':
    'Nós, serviços, tarefas, secrets e configs, encontrados pelos hosts manager dele. Implante stacks no Swarm.',
  'home.engines.podman.body':
    'Pods e unidades Quadlet instaladas e iniciadas sob o systemd, pela própria API do Podman.',
  'home.engines.containerd.body':
    'Namespaces, tarefas, logs, um terminal e os snapshots que estão ocupando seu disco, sem Docker.',
  'home.engines.incus.body':
    'Contêineres de sistema e máquinas virtuais: o console, os snapshots, as imagens, os perfis, o armazenamento e as redes deles.',
  'home.engines.kubernetes.body':
    'Cargas de trabalho, Pods, Services e qualquer recurso da API, ao vivo. Logs, terminais, encaminhamentos de porta, YAML com prévia, releases do Helm.',
  'home.engines.nomad.body':
    'Jobs executados a partir de HCL com o plano deles, suas alocações e logs, um shell em uma tarefa e a drenagem de nós.',

  // Home: security.
  'home.security.title': 'Suas chaves ficam lacradas.',
  'home.security.lead':
    'O Homeport lida com as credenciais que abrem seus servidores, então as trata como tal.',
  'home.security.rest.title': 'Criptografado em repouso',
  'home.security.rest.body':
    'Chaves SSH, certificados TLS, ambientes de stacks e tokens de repositórios são lacrados com XChaCha20-Poly1305. A chave fica no chaveiro do sistema, não ao lado dos dados.',
  'home.security.disk.title': 'Nunca em disco às claras',
  'home.security.disk.body':
    'O Homeport fala SSH por conta própria, então as chaves descriptografadas ficam na memória. Nenhum arquivo de chave, nenhuma mudança no seu {dir}.',
  'home.security.home.title': 'Nada é enviado para casa',
  'home.security.home.body':
    'Sem conta, sem telemetria, sem rastreamento. Cada versão é gerada publicamente, com checksums e procedência da build.',
  'home.security.link': 'A segurança e os seus dados',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Comece por esta máquina.',
  'home.close.body':
    'Baixe o Homeport no computador em que você trabalha e depois adicione seus hosts. Ele é gratuito, de código aberto sob a GNU AGPL e desenvolvido publicamente no GitHub.',
  'home.close.github': 'Ver no GitHub',
  'home.desktops.windows.versions': '10 e 11, x64 e ARM64',
  'home.desktops.windows.formats': 'Instalador (.exe) ou MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Imagem de disco (.dmg)',
  'home.desktops.linux.versions': 'x64 e ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper ou AppImage',
};
