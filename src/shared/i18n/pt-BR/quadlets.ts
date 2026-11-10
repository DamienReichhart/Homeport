/** Brazilian Portuguese text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Unidades Quadlet',
  'quadlets.summary': '{count, plural, one {# unidade} other {# unidades}}',
  'quadlets.search': 'Pesquisar unidades',
  'quadlets.empty': 'Nenhuma unidade Quadlet',
  'quadlets.emptyDescription':
    'Unidades Quadlet são arquivos que o Podman transforma em serviços do systemd, para que os contêineres iniciem com o host e o systemd os mantenha em execução. Instale uma para vê-la aqui.',
  'quadlets.new': 'Nova unidade',
  'quadlets.edit': 'Editar',
  'quadlets.notGenerated': 'ainda não gerado',
  'quadlets.gone': '{name} não está mais instalada',
  'quadlets.column.name': 'Unidade',
  'quadlets.column.kind': 'Tipo',
  'quadlets.column.service': 'Serviço',
  'quadlets.column.status': 'Status',
  'quadlets.column.containers': 'Contêineres',
  'quadlets.status.running': 'Em execução',
  'quadlets.status.starting': 'Iniciando',
  'quadlets.status.stopping': 'Parando',
  'quadlets.status.stopped': 'Parado',
  'quadlets.status.failed': 'Falhou',
  'quadlets.status.notLoaded': 'Não carregado pelo systemd',
  'quadlets.status.template': 'Modelo',
  'quadlets.action.start': 'Iniciar',
  'quadlets.action.stop': 'Parar',
  'quadlets.action.restart': 'Reiniciar',
  'quadlets.actionOne.start': 'Iniciar {name}',
  'quadlets.actionOne.stop': 'Parar {name}',
  'quadlets.actionOne.restart': 'Reiniciar {name}',
  'quadlets.done.start': '{name} iniciada',
  'quadlets.done.stop': '{name} parada',
  'quadlets.done.restart': '{name} reiniciada',
  'quadlets.installed': '{name} instalada',
  'quadlets.saved': '{name} salva',
  'quadlets.removed': '{name} removida',
  'quadlets.blocked.noCommands':
    'O Homeport inicia e para unidades com o systemctl, que ele só consegue executar via SSH ou para um Podman neste computador.',
  'quadlets.blocked.notLoaded':
    'O systemd não gerou o serviço desta unidade, então não há nada para iniciar.',
  'quadlets.blocked.podmanTooOld':
    'Ler, instalar, editar e remover unidades requer o Podman 5.8 ou posterior; este host executa o Podman {version}.',
  'quadlets.blocked.noFile':
    'Esta unidade só é conhecida pelos seus contêineres, então o arquivo dela não pode ser alterado.',
  'quadlets.note.labels':
    'O Podman {version} não consegue listar unidades Quadlet, então estas vêm dos contêineres que elas criaram. Os arquivos delas, e as unidades que não criaram nenhum contêiner, aparecem no Podman 5.7 e posterior.',
  'quadlets.note.readOnly':
    'O Podman {version} lista unidades, mas não consegue lê-las, instalá-las, editá-las ou removê-las; isso requer o Podman 5.8 ou posterior.',
  'quadlets.note.noCommands':
    'Iniciar e parar unidades requer que este host seja acessado via SSH ou um Podman neste computador.',
  'quadlets.detail.service': 'Serviço',
  'quadlets.detail.unit': 'Unidade systemd',
  'quadlets.detail.scope': 'Executa como',
  'quadlets.detail.path': 'Arquivo',
  'quadlets.detail.app': 'Aplicação',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {Contêineres} one {# contêiner} other {# contêineres}}',
  'quadlets.detail.noContainers': 'Ela não criou nenhum contêiner, ou o serviço dela está parado.',
  'quadlets.scope.user': 'Uma unidade de usuário (systemctl --user)',
  'quadlets.scope.system': 'Uma unidade de sistema',
  'quadlets.file.title': 'Arquivo da unidade',
  'quadlets.file.unknown': 'Esta unidade só é conhecida pelo rótulo dos seus contêineres.',
  'quadlets.file.tooOld':
    'O Podman {version} não consegue ler arquivos de unidade; mostrá-los requer o Podman 5.8 ou posterior.',
  'quadlets.form.newTitle': 'Nova unidade Quadlet',
  'quadlets.form.newDescription':
    'O Podman grava o arquivo onde ficam as unidades do seu usuário e recarrega o systemd. Inicie a unidade depois que ela for instalada.',
  'quadlets.form.editTitle': 'Editar {name}',
  'quadlets.form.editDescription':
    'Salvar substitui o arquivo e recarrega o systemd; reinicie a unidade para que um serviço em execução o use. Os arquivos instalados ao lado dela permanecem.',
  'quadlets.form.name': 'Nome do arquivo',
  'quadlets.form.kind': 'Tipo',
  'quadlets.form.openFile': 'Abrir arquivo…',
  'quadlets.form.openTitle': 'Abrir um arquivo de unidade Quadlet',
  'quadlets.form.unitFiles': 'Arquivos de unidade Quadlet',
  'quadlets.form.content': 'Arquivo da unidade',
  'quadlets.form.extra': 'Arquivos ao lado',
  'quadlets.form.extraHint':
    'Um Containerfile, YAML kube ou arquivo env que a unidade usa, instalado na pasta da aplicação dela.',
  'quadlets.form.extraTitle': 'Escolher arquivos para instalar ao lado da unidade',
  'quadlets.form.addExtra': 'Adicionar arquivos…',
  'quadlets.form.removeExtra': 'Remover {name}',
  'quadlets.form.install': 'Instalar',
  'quadlets.form.save': 'Salvar',
  'quadlets.form.overwrite': 'Sobrescrever',
  'quadlets.form.nameInvalid':
    'Dê um nome como web.container, terminando em .container, .pod, .volume, .network, .kube, .image, .build ou .artifact.',
  'quadlets.form.sectionMissing':
    'O arquivo precisa de uma seção {section} para o Quadlet poder lê-lo.',
  'quadlets.form.extraInvalid':
    'Cada arquivo ao lado da unidade precisa de um nome simples próprio, sem pastas.',
  'quadlets.remove.title': 'Remover {name}?',
  'quadlets.remove.description':
    'O Podman exclui o arquivo da unidade e recarrega o systemd. Contêineres, volumes e imagens que ela criou permanecem.',
  'quadlets.remove.stop': 'Parar o serviço dela se estiver em execução',
  'quadlets.remove.stopRunning': 'Parar o serviço em execução dela antes',
} satisfies Translation<typeof en>;
