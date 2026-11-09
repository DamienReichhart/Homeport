/** Brazilian Portuguese text of the `settings` namespace. */
import type en from '../en/settings';
import type { Translation } from '../types';

export default {
  'settings.nav': 'Configurações',
  'settings.title': 'Configurações',
  'settings.appearance.title': 'Aparência',
  'settings.appearance.theme': 'Tema',
  'settings.theme.system': 'Igual ao sistema',
  'settings.theme.light': 'Claro',
  'settings.theme.dark': 'Escuro',
  'settings.appearance.language': 'Idioma',
  'settings.language.system': 'Igual ao sistema ({language})',
  'settings.desktop.title': 'Área de trabalho',
  'settings.desktop.description': 'Como o Homeport é executado neste computador.',
  'settings.desktop.closeToTray': 'Fechar para a bandeja do sistema',
  'settings.desktop.closeToTrayHint':
    'Fechar a janela mantém o Homeport e as conexões com os hosts em execução na bandeja. Saia pelo ícone da bandeja. Em ambientes de trabalho que ocultam ícones da bandeja, como o GNOME sem uma extensão, o ícone pode não aparecer.',
  'settings.desktop.noTray':
    'Este ambiente de trabalho não tem uma bandeja do sistema que o Homeport possa usar, então fechar a janela encerra o aplicativo.',
  'settings.desktop.startAtLogin': 'Iniciar ao fazer login',
  'settings.desktop.startAtLoginHint':
    'O Homeport inicia quando você faz login neste computador, para acompanhar seus hosts desde o início.',
  'settings.desktop.startHidden': 'Iniciar oculto na bandeja',
  'settings.desktop.startHiddenHint':
    'Quando inicia ao fazer login, o Homeport fica na bandeja do sistema sem abrir a janela.',
  'settings.desktop.startHiddenNoTray':
    'Requer uma bandeja do sistema a partir da qual abrir a janela, e este ambiente de trabalho não tem.',
  'settings.notifications.title': 'Notificações',
  'settings.notifications.description':
    'O Homeport avisa você, mesmo com a janela fechada na bandeja, quando:',
  'settings.notifications.containerDied': 'Um contêiner para com um erro ou fica sem memória',
  'settings.notifications.containerUnhealthy': 'Um contêiner deixa de estar íntegro',
  'settings.notifications.hostUnreachable': 'Um host conectado fica inacessível',
  'settings.notifications.hostBack': 'Um host inacessível volta',
  'settings.machine.title': 'Esta máquina',
  'settings.machine.description': 'O que o Homeport pode usar aqui.',
  'settings.machine.keychain': 'Chaveiro do sistema',
  'settings.machine.keychainAvailable':
    'Disponível — chaves SSH, certificados, ambientes de stacks e logins de registro são armazenados criptografados.',
  'settings.machine.keychainUnavailable':
    'Indisponível — chaves SSH, certificados TLS, ambientes de stacks e logins de registro não podem ser armazenados.',
  'settings.machine.compose': 'CLI do Compose',
  'settings.machine.composeMissing':
    'As stacks podem ser iniciadas e paradas, mas não implantadas.',
  'settings.machine.stackDeploy': 'CLI do Docker, para Swarms',
  'settings.machine.stackDeployMissing':
    'As stacks não podem ser implantadas em um Swarm; todo o resto dos Swarms funciona.',
  'settings.machine.git': 'git',
  'settings.machine.helm': 'CLI do Helm',
  'settings.sync.title': 'Stacks sincronizadas',
  'settings.sync.interval': 'Verificar repositórios a cada',
  'settings.sync.minutes': 'minutos',
  'settings.data.title': 'Dados',
  'settings.data.description': 'O que o Homeport guarda neste computador.',
  'settings.data.backup': 'Fazer backup…',
  'settings.data.restore': 'Restaurar…',
  'settings.data.backupHint':
    'Salve todos os hosts, stacks, logins de registro, repositórios e configurações, com seus segredos, em um único arquivo protegido por senha e restaure-o neste ou em outro computador.',
  'settings.backup.title': 'Fazer backup dos dados do Homeport',
  'settings.backup.description':
    'O arquivo contém todos os hosts, clusters, stacks, logins de registro, repositórios e configurações, e as chaves, certificados, tokens, senhas e ambientes deles. Qualquer pessoa com ele e a senha pode usá-los, então guarde os dois com segurança. Eventos do Engine e o histórico de implantações não são incluídos.',
  'settings.backup.passphrase': 'Senha',
  'settings.backup.passphraseHint': 'Pelo menos {min} caracteres. Ela não pode ser recuperada.',
  'settings.backup.confirmation': 'Senha novamente',
  'settings.backup.problem.short': 'Use pelo menos {min} caracteres.',
  'settings.backup.problem.mismatch': 'As duas senhas são diferentes.',
  'settings.backup.start': 'Fazer backup…',
  'settings.backup.saveTitle': 'Salvar o backup como',
  'settings.backup.fileType': 'Backup do Homeport',
  'settings.backup.done':
    'Backup feito de {hosts, plural, one {# host} other {# hosts}} e {stacks, plural, one {# stack} other {# stacks}}',
  'settings.backup.withoutSecrets':
    'Estes tiveram backup sem os seus segredos, que não puderam ser abertos:',
  'settings.restore.pick': 'Restaurar de',
  'settings.restore.title': 'Restaurar os dados do Homeport',
  'settings.restore.from': 'De {path}',
  'settings.restore.open': 'Abrir',
  'settings.restore.madeBy': 'Backup feito pelo Homeport {version} em {date}. Ele contém:',
  'settings.restore.holds': 'O que o backup contém',
  'settings.restore.hosts': '{count, plural, one {# host} other {# hosts}}',
  'settings.restore.clusters': '{count, plural, one {# cluster} other {# clusters}}',
  'settings.restore.stacks': '{count, plural, one {# stack} other {# stacks}}',
  'settings.restore.registries':
    '{count, plural, one {# login de registro} other {# logins de registro}}',
  'settings.restore.repositories': '{count, plural, one {# repositório} other {# repositórios}}',
  'settings.restore.settings': 'Suas configurações',
  'settings.restore.replaces':
    'Restaurar substitui todos os hosts, stacks, logins de registro, repositórios e configurações que você tem agora, e esquece os eventos do Engine e o histórico de implantações. Uma cópia dos dados atuais é guardada antes na pasta de dados do Homeport.',
  'settings.restore.confirm': 'Substituir tudo',
  'settings.restore.done':
    '{hosts, plural, one {# host} other {# hosts}} e {stacks, plural, one {# stack} other {# stacks}} restaurados',
  'settings.data.keepEvents': 'Manter eventos do Engine por',
  'settings.data.days': 'dias',
  'settings.data.keepEventsHint':
    'Cada host mantém no máximo 10.000 eventos para a página Eventos. Os mais antigos são removidos a cada hora.',
  'settings.machine.gitMissing':
    'As stacks sincronizadas precisam dele; todos os outros tipos de stack funcionam sem ele.',
  'settings.machine.helmMissing':
    'As releases do Helm não podem ser listadas nem instaladas; todo o resto dos clusters funciona.',
  'settings.about.tagline': 'Todos os Docker Engines, em um só quadro.',
  'settings.about.openLogFolder': 'Abrir pasta de logs',
  'settings.about.licenses': 'Licenças',
  'settings.about.exportDiagnostics': 'Exportar diagnóstico…',
  'settings.diagnostics.title': 'Exportar diagnóstico',
  'settings.diagnostics.description':
    'Um arquivo zip para anexar a um relatório de bug: um resumo das versões e dos tipos de hosts e clusters que você usa, e os logs do Homeport sem os segredos. Nada é enviado a lugar nenhum: você decide se compartilha o arquivo.',
  'settings.diagnostics.includeNames': 'Incluir nomes e endereços',
  'settings.diagnostics.includeNamesHelp':
    'Quando desativado, hosts e clusters aparecem como host-1, cluster-1… no resumo e nos logs.',
  'settings.diagnostics.files': 'Arquivos no zip',
  'settings.diagnostics.save': 'Salvar…',
  'settings.diagnostics.saveTitle': 'Salvar diagnóstico',
  'settings.diagnostics.fileType': 'Arquivo zip',
  'settings.diagnostics.done': 'Diagnóstico salvo em {path}.',
  'settings.licenses.title': 'Licenças',
  'settings.licenses.description':
    'O Homeport é software livre sob a GNU Affero General Public License, versão 3, construído com os pacotes de código aberto abaixo.',
  'settings.licenses.source': 'O código-fonte desta versão é publicado sob a mesma licença.',
  'settings.licenses.sourceLink': 'Código-fonte da v{version}',
  'settings.licenses.thirdParty': 'Software de terceiros',
  'settings.licenses.missing.title': 'Não incluído nesta build',
  'settings.licenses.missing.description':
    'As builds de release as incluem; execute <code>{command}</code> para gerá-las em uma build de desenvolvimento.',
  'settings.palette.group': 'Configurações',
  'settings.palette.themeLight': 'Tema: Claro',
  'settings.palette.themeDark': 'Tema: Escuro',
  'settings.palette.themeSystem': 'Tema: Igual ao sistema',
} satisfies Translation<typeof en>;
