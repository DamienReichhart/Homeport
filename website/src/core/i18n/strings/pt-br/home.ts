import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in Brazilian Portuguese. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — Todos os Docker Engines, em um só quadro.',
  'home.hero.new': 'Novo',
  'home.hero.release': 'O Homeport {version} foi lançado',
  'home.hero.title': 'Todos os Docker Engines, em um só quadro.',
  'home.hero.lead':
    'O Homeport roda na sua máquina e alcança seus hosts por SSH, TLS ou TCP. Nenhum agente no servidor. Gratuito, de código aberto, para Windows, macOS e Linux.',
  'home.hero.source': 'Ler o código-fonte',
  'home.board.caption':
    'Um quadro de exemplo: cinco hosts, seus Engines e como o Homeport os alcança. Um deles precisa de atenção.',
  'home.board.host': 'Host',
  'home.board.engine': 'Engine',
  'home.board.via': 'Via',
  'home.board.stacks': 'Stacks',
  'home.board.status': 'Status',
  'home.board.allRunning': 'Tudo em execução',
  'home.board.needsYou': '1 stack precisa de você',
  'home.board.note':
    'O que está íntegro fica sem destaque. Só o que precisa de você fica vermelho.',
  'home.runtimes.label': 'Funciona com',
  'home.runtimes.title': 'Funciona com o que você já usa',

  // Home: where it lives.
  'home.visits.eyebrow': 'Onde ele fica',
  'home.visits.title':
    'O Homeport fica na sua máquina e visita seus servidores. Ele nunca se instala neles.',
  'home.visits.agent.title': 'Nenhum agente',
  'home.visits.agent.body':
    'Nada a instalar, atualizar ou proteger no servidor. O Homeport conversa com o Engine que já está lá.',
  'home.visits.panel.title': 'Nenhum painel web',
  'home.visits.panel.body':
    'Nenhum dashboard implantado nos seus servidores e nenhuma página de login esperando que alguém a encontre.',
  'home.visits.account.title': 'Nenhuma conta',
  'home.visits.account.body':
    'Sem login, sem licença por usuário, sem assinatura e sem telemetria. Ele mostra seus contêineres, e só isso.',
  'home.visits.ports.title': 'O que ele precisa no servidor, com sinceridade',
  'home.visits.ports.ssh':
    'A porta que você já tem aberta. Com a chave que o Homeport cria, uma linha em {file}.',
  'home.visits.ports.tls':
    'A própria porta da API do Engine (geralmente {port}), que você mesmo expõe.',
  'home.visits.ports.tcp':
    'A porta da API do Engine (geralmente {port}), sem criptografia. Só em uma rede em que você confia.',
  'home.visits.ports.advice':
    'Quando puder, use SSH: ele não acrescenta nada ao servidor que você já não esteja executando.',
  'home.visits.link': 'Como as conexões funcionam',

  // Home: the features.
  'home.features.eyebrow': 'Recursos',
  'home.features.title': 'Tudo o que seus Engines executam, em uma só janela',
  'home.features.lead':
    'O dia a dia do Docker — o que está em execução, o que ele diz, quanto ele consome — sem precisar lembrar em qual servidor ele está.',
  'home.features.hosts.title': 'Hosts, perto e longe',
  'home.features.hosts.body':
    'Engines locais e máquinas do Podman são encontrados para você. Os remotos entram por SSH, passando por bastions se for preciso, por TLS mútuo ou TCP, ou direto do seu ~/.ssh/config e dos seus contextos do Docker. Cada host se reconecta sozinho quando a rede oscila.',
  'home.features.containers.title': 'Contêineres de relance',
  'home.features.containers.body':
    'Agrupados por stack, com CPU e memória ao vivo e o motivo de um deles não estar íntegro. Execute com limites, dispositivos e verificações de integridade e depois edite, duplique, faça commit ou exporte um contêiner.',
  'home.features.stacks.title': 'Stacks Compose',
  'home.features.stacks.body':
    'Comece de um modelo, vincule uma pasta de projeto neste computador ou acompanhe um repositório git e depois implante em qualquer host — builds incluídas. Veja o que muda antes de cada reimplantação ou sincronização, reverta e atualize para imagens mais novas em um só passo.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Os Swarms aparecem pelos hosts manager deles. Gerencie nós, serviços, tarefas, secrets e configs, implante stacks em um Swarm e crie um Swarm, entre nele ou saia dele a partir de um host, sem nada instalado em nenhum nó.',
  'home.features.podman.title': 'Podman, como Podman',
  'home.features.podman.body':
    'O Homeport reconhece o Podman ao se conectar e acrescenta o que só o Podman tem: pods, criados com suas portas e com contêineres executados dentro deles, e unidades Quadlet, instaladas, editadas e iniciadas sob o systemd, pela própria API do Podman.',
  'home.features.containerd.title': 'containerd, diretamente',
  'home.features.containerd.body':
    'Conecte-se ao containerd pelo socket dele, localmente ou por SSH, sem Docker: escolha um namespace, inicie, pare e envie sinais às tarefas dos contêineres dele, leia os logs, abra um terminal, baixe imagens com seus logins de registro e encontre os snapshots que estão ocupando seu disco.',
  'home.features.incus.title': 'Instâncias do Incus',
  'home.features.incus.body':
    'Adicione o Incus pelo socket dele, por SSH ou por HTTPS com um token de confiança e execute seus contêineres de sistema e máquinas virtuais: crie-os a partir de uma imagem remota, inicie, congele e pare, abra o console deles ou um shell, crie e restaure snapshots e gerencie imagens, perfis, armazenamento e redes.',
  'home.features.clusters.title': 'Clusters Kubernetes e Nomad',
  'home.features.clusters.body':
    'Adicione clusters Kubernetes do seu kubeconfig — EKS, GKE e AKS incluídos — ou encontre os locais em um clique. Navegue ao vivo por cargas de trabalho, Pods, Services, configuração e qualquer recurso da API; acompanhe logs, abra um terminal, encaminhe portas, escale e aplique YAML com uma prévia, e instale, atualize e reverta charts do Helm. Adicione clusters Nomad pelo endereço para executar jobs a partir de HCL com o plano deles, escalá-los e pará-los, acompanhar logs, abrir um shell e drenar nós. Nada é instalado em um cluster.',
  'home.features.images.title': 'Imagens',
  'home.features.images.body':
    'Pesquise no Docker Hub, baixe e envie com seus logins de registro, faça build a partir de uma pasta no seu computador, salve imagens em arquivos e carregue-as de volta, e veja quando uma imagem mais nova sai.',
  'home.features.volumes.title': 'Volumes e redes',
  'home.features.volumes.body':
    'Veja quais contêineres usam o quê. Crie, conecte e desconecte, com uma exclusão que diz exatamente o que você está prestes a perder.',
  'home.features.board.title': 'Um quadro para todos os hosts',
  'home.features.board.body':
    'Todos os contêineres de todos os hosts em uma tabela, filtrados e operados juntos. Cada host também em uma página: detalhes do Engine, uso de disco com uma limpeza em uma só caixa de diálogo e um fluxo ao vivo do que o Engine está fazendo.',
  'home.features.keyboard.title': 'Teclado em primeiro lugar',
  'home.features.keyboard.body':
    'Uma paleta de comandos para ir a qualquer lugar, trocar de host ou executar um comando, e para encontrar qualquer contêiner, imagem ou volume em qualquer host que o tenha. Atalhos em todas as páginas, listados com ?.',
  'home.features.background.title': 'De olho em segundo plano',
  'home.features.background.body':
    'O status de cada host na bandeja do sistema, notificações quando um contêiner falha ou um host cai, e o Homeport iniciando com o login, oculto até você precisar dele.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Hosts',
  'home.hosts.title': 'Engines remotos sem cerimônia',
  'home.hosts.body':
    'Adicione um servidor uma vez e ele fica no quadro. Para SSH, o Homeport cria uma chave só para esse host e entrega a você o único comando que a autoriza. Ele confere a impressão digital do servidor no primeiro contato e avisa você se ela mudar algum dia.',
  'home.hosts.import':
    'Já os descreveu? Importe seus servidores de {file}, incluindo servidores intermediários, ou seus contextos do Docker, incluindo certificados. Ao mudar para um novo computador, leve-os em um só arquivo protegido por senha.',
  'home.hosts.socket.name': 'Socket local',
  'home.hosts.socket.detail': 'Sockets Unix e named pipes do Windows, detectados automaticamente.',
  'home.hosts.ssh.detail':
    'Uma chave Ed25519 dedicada por host, sua própria chave, o ssh-agent ou o Pageant; também por bastions.',
  'home.hosts.tls.detail': 'TLS mútuo com a sua CA, o certificado de cliente e a chave.',
  'home.hosts.tcp.detail':
    'TCP simples, sem criptografia, só para um Engine em uma rede em que você confia.',
  'home.hosts.link': 'Como as conexões funcionam',
  'home.hosts.screenshot':
    'A página Adicionar host com SSH selecionado: servidor, porta, usuário, caminho do socket remoto e uma nova chave para este host.',

  // Home: containers.
  'home.containers.eyebrow': 'Contêineres',
  'home.containers.title': 'Tudo sobre um contêiner, a um clique',
  'home.containers.body':
    'Abra um contêiner e os logs dele já estão chegando. Troque de aba para um terminal interativo, os arquivos dele, gráficos ao vivo, processos e o documento de inspect completo — em um host do outro lado do mundo com a mesma facilidade que no seu notebook.',
  'home.containers.logs.title': 'Logs',
  'home.containers.logs.body': 'Acompanhe, pesquise, filtre por fluxo e por horário, baixe.',
  'home.containers.terminal.title': 'Terminal',
  'home.containers.terminal.body': 'Shells interativos, quantos você precisar.',
  'home.containers.files.title': 'Arquivos',
  'home.containers.files.body': 'Navegue, envie e baixe; pastas como .tar.',
  'home.containers.stats.title': 'Estatísticas',
  'home.containers.stats.body': 'Gráficos de CPU, memória, rede e E/S de disco.',
  'home.containers.link': 'Trabalhar com contêineres',
  'home.containers.screenshot':
    'A aba Logs de um contêiner, transmitindo a saída com pesquisa, carimbos de data/hora e filtros de fluxo.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stacks',
  'home.stacks.title': 'Stacks Compose: gerenciadas, vinculadas, sincronizadas ou encontradas',
  'home.stacks.body':
    'Guarde um arquivo compose e o ambiente dele no Homeport e implante-o em qualquer host — comece de um modelo, combine vários arquivos, escolha perfis; as variáveis são armazenadas criptografadas, nunca gravadas em disco. Ou vincule uma pasta de projeto neste computador: o Homeport edita os arquivos dela no lugar e implanta a partir dela, builds incluídas. Ou acompanhe uma branch de um repositório git, com uma chave de deploy ou um token: cada sincronização mostra os novos commits e o que eles mudam antes de implantar e, enquanto o Homeport está aberto, ele os verifica, implantando sozinho se você pedir. Cada reimplantação mostra antes o que muda, cada implantação é guardada para que você possa reverter, e Atualizar baixa imagens mais novas e reimplanta em um só passo. As stacks que alguém implantou à mão também aparecem: inicie, pare ou remova essas stacks, ou adote-as.',
  'home.stacks.link': 'Gerenciar stacks',
  'home.stacks.screenshot':
    'A definição de uma stack gerenciada: o editor do compose.yaml, o ambiente dela e o botão Salvar e reimplantar.',

  // Home: security.
  'home.security.eyebrow': 'Segurança',
  'home.security.title': 'Suas chaves ficam lacradas',
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
    'Sem conta, sem telemetria, sem rastreamento. O Homeport conversa com seus Engines e com mais nada. Cada versão é gerada publicamente, com checksums e procedência da build.',
  'home.security.link': 'A segurança e os seus dados',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Download',
  'home.desktops.title': 'Nativo em todos os desktops',
  'home.desktops.lead':
    'Um app pequeno e rápido feito com Tauri e Rust — não uma aba de navegador disfarçada. Instaladores para x64 e ARM que mantêm o Homeport atualizado.',
  'home.desktops.windows.versions': '10 e 11 · x64 e ARM64',
  'home.desktops.windows.formats': 'Instalador (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon e Intel',
  'home.desktops.macos.formats': 'Imagem de disco (.dmg)',
  'home.desktops.linux.versions': 'x64 e ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Nada a instalar nos seus servidores.',
  'home.cta.body':
    'Baixe o Homeport na máquina em que você trabalha e adicione seus hosts. Ele é gratuito, de código aberto sob a GNU AGPL, e desenvolvido publicamente no GitHub.',
  'home.cta.github': 'Ver no GitHub',
} satisfies Translation<typeof en>;
