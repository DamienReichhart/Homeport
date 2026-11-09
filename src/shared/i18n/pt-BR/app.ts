/** Brazilian Portuguese text of the `app` namespace. */
import type en from '../en/app';
import type { Translation } from '../types';

export default {
  'app.nav.main': 'Principal',
  'app.keychainMissing':
    'Nenhum chaveiro do sistema está disponível, então chaves SSH, certificados TLS e ambientes de stacks não podem ser armazenados. Hosts locais, TCP e via ssh-agent continuam funcionando.',
  'app.crash.title': 'Algo deu errado',
  'app.crash.description':
    'O Homeport encontrou um problema inesperado nesta página. Recarregar costuma resolver; se continuar acontecendo, copie o relatório em Detalhes para uma issue.',
  'app.crash.reload': 'Recarregar',
  'app.crash.backToOverview': 'Voltar à visão geral',
  'app.shortcuts.title': 'Atalhos de teclado',
  'app.shortcuts.description':
    'Os atalhos que funcionam aqui. Teclas como N e / não funcionam enquanto você digita em um campo.',
  'app.shortcuts.general': 'Geral',
  'app.shortcuts.navigation': 'Ir para',
  'app.shortcuts.hosts': 'Hosts',
  'app.shortcuts.help': 'Mostrar atalhos de teclado',
  'app.shortcuts.settings': 'Abrir Configurações',
  'app.shortcuts.goTo': 'Ir para {page}',
  'app.shortcuts.previousHost': 'Host anterior',
  'app.shortcuts.nextHost': 'Próximo host',
  'app.shortcuts.refresh': 'Atualizar as listas deste host',
  'app.palette.title': 'Paleta de comandos',
  'app.palette.description': 'Ir para uma página, trocar de host ou executar um comando.',
  'app.palette.open': 'Abrir a paleta de comandos',
  'app.palette.placeholder': 'Digite uma página, um host ou um comando…',
  'app.palette.empty': 'Nada corresponde.',
  'app.palette.recent': 'Recentes',
  'app.palette.goTo': 'Ir para',
  'app.palette.hosts': 'Hosts',
  'app.palette.actions': 'Ações',
  'app.palette.switchTo': 'Trocar para {name}',
} satisfies Translation<typeof en>;
