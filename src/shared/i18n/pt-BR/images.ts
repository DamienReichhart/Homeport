/** Brazilian Portuguese text of the `images` namespace. */
import type en from '../en/images';
import type { Translation } from '../types';

export default {
  'images.nav': 'Imagens',
  'images.title': 'Imagens',
  'images.summary': '{count, plural, one {# imagem} other {# imagens}} · {size}',
  'images.cleanUp': 'Limpar',
  'images.cleanUp.dangling': 'Imagens órfãs',
  'images.cleanUp.all': 'Todas as imagens não usadas',
  'images.saveSelected': '{count, plural, one {Salvar imagem…} other {Salvar # imagens…}}',
  'images.load': 'Carregar…',
  'images.importTar': 'Importar de tar…',
  'images.build': 'Build',
  'images.pull': 'Baixar imagem',
  'images.search': 'Pesquisar imagens',
  'images.empty.title': 'Nenhuma imagem',
  'images.empty.description': 'Baixe uma de um registro para começar.',

  'images.column.name': 'Nome',
  'images.column.id': 'ID',
  'images.column.created': 'Criada',
  'images.column.size': 'Tamanho',
  'images.column.status': 'Status',
  'images.moreTags': '{count, plural, one {+# tag a mais} other {+# tags a mais}}',
  'images.inUse': 'Em uso · {count}',
  'images.unused': 'Não usada',
  'images.runOne': 'Executar {name}',
  'images.action.inspect': 'Inspecionar',
  'images.action.tag': 'Adicionar tag',
  'images.action.push': 'Enviar',
  'images.action.save': 'Salvar…',

  'images.remove.title': 'Remover {name}?',
  'images.remove.everyName':
    'Isso remove todos os nomes da imagem ({names}) e as camadas dela que nada mais usa.',
  'images.remove.description':
    'A imagem e as camadas que nenhuma outra imagem usa são excluídas. Ela pode ser baixada novamente depois.',
  'images.prune.allTitle': 'Remover todas as imagens não usadas?',
  'images.prune.danglingTitle': 'Remover imagens órfãs?',
  'images.prune.allDescription':
    'Todas as imagens que nenhum contêiner usa são excluídas, inclusive as com tag.',
  'images.prune.danglingDescription':
    'Imagens sem tag deixadas por builds e downloads são excluídas.',

  'images.removed': '{name} removida',
  'images.pruned':
    '{count, plural, one {# imagem removida} other {# imagens removidas}}, {space} recuperados',
  'images.pulled': '{reference} baixada',
  'images.pushed': '{reference} enviada',
  'images.built': 'Build de {name} concluída',
  'images.builtUnnamed': 'Build da imagem concluída',
  'images.saved': '{path} salvo',
  'images.loadedOne': '{name} carregada',
  'images.loaded': '{count, plural, one {# imagem carregada} other {# imagens carregadas}}',
  'images.imported': '{name} importada',

  'images.files.save': 'Salvar imagens como',
  'images.files.load': 'Carregar imagens de',
  'images.files.import': 'Importar um sistema de arquivos de',
  'images.files.imageArchives': 'Arquivos de imagem',
  'images.files.imageArchive': 'Arquivo de imagem',
  'images.files.filesystemArchives': 'Arquivos de sistema de arquivos',

  'images.buildDialog.title': 'Fazer build de uma imagem',
  'images.buildDialog.description':
    'A partir de uma pasta neste computador. O Homeport a envia ao host, deixando de fora o que o .dockerignore dela exclui, e o Engine do host faz a build.',
  'images.buildDialog.built': '✓ Build concluída',
  'images.buildDialog.folder': 'Pasta',
  'images.buildDialog.chooseFolder': 'Escolha a pasta para a build',
  'images.buildDialog.choose': 'Escolher…',
  'images.buildDialog.dockerfile': 'Dockerfile',
  'images.buildDialog.target': 'Estágio de destino',
  'images.buildDialog.targetPlaceholder': 'O último estágio',
  'images.buildDialog.names': 'Nomes',
  'images.buildDialog.buildArgs': 'Argumentos de build',
  'images.buildDialog.buildArgsPlaceholder': 'VERSION=1.0, um por linha',
  'images.buildDialog.noCache': 'Não usar o cache de build',
  'images.buildDialog.pull': 'Baixar imagens base mais recentes',
  'images.buildForm.noFolder': 'Escolha a pasta para a build.',
  'images.buildForm.argNeedsName': '“{line}” precisa de um nome antes de “=”.',

  'images.hub.unavailable':
    'Não é possível pesquisar no Docker Hub agora: {error} Um nome completo como nginx:1.27 ainda pode ser baixado.',
  'images.hub.noMatch': 'Nada no Docker Hub corresponde a “{term}”.',
  'images.hub.results': 'Resultados do Docker Hub',
  'images.hub.result': '{name}, {stars} estrelas',
  'images.hub.resultOfficial': '{name}, imagem oficial, {stars} estrelas',
  'images.hub.official': 'Oficial',
  'images.hub.backToSearch': 'Pesquisar',
  'images.hub.tagCount': '{count, plural, one {# tag} other {# tags}}',
  'images.hub.filterTags': 'Filtrar tags',
  'images.hub.tags': 'Tags',
  'images.hub.noTagMatch': 'Nenhuma tag corresponde a “{filter}”.',
  'images.hub.moreTags': 'Mais tags',

  'images.importDialog.title': 'Importar um sistema de arquivos como imagem',
  'images.importDialog.description':
    '{path} vira uma imagem de uma camada, sem histórico nem configurações próprias; informe um comando abaixo para torná-la executável.',
  'images.importDialog.name': 'Nome da imagem',
  'images.importDialog.changes': 'Alterações',
  'images.importDialog.changesHint':
    'Instruções de Dockerfile, uma por linha: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD, USER, VOLUME ou WORKDIR.',
  'images.importDialog.import': 'Importar',

  'images.inspect.layers': 'Camadas',
  'images.inspect.raw': 'JSON bruto',
  'images.inspect.created': 'Criada',
  'images.inspect.instruction': 'Instrução',
  'images.inspect.size': 'Tamanho',

  'images.pullDialog.title': 'Baixar uma imagem',
  'images.pullDialog.description':
    'Pesquise no Docker Hub pelo nome ou digite qualquer referência que o host consiga acessar, por exemplo nginx:1.27 ou ghcr.io/org/app:v2.',
  'images.pullDialog.placeholder': 'postgres, ou nginx:1.27',
  'images.pullDialog.reference': 'Referência da imagem',
  'images.pullDialog.pull': 'Baixar',
  'images.done': 'Concluído.',

  'images.pushDialog.title': 'Enviar {name}',
  'images.pushDialog.description':
    'Para o registro com o qual o nome dela começa, por exemplo ghcr.io/org/app:v2, ou para o Docker Hub em nomes como org/app:v2.',
  'images.pushDialog.target': 'Nome a enviar',
  'images.pushDialog.push': 'Enviar',
  'images.pushDialog.tagsFirst': 'A imagem recebe esse nome antes, como faz Adicionar tag.',
  'images.pushDialog.uploading': 'Enviando {current} / {total}',
  'images.push.pushed': '{pushed, plural, one {# camada enviada} other {# camadas enviadas}}',
  'images.push.pushedAndExisting':
    '{pushed, plural, one {# camada enviada} other {# camadas enviadas}}, {existing} já no registro',

  'images.tagDialog.title': 'Adicionar tag a {name}',
  'images.tagDialog.newName': 'Novo nome',
  'images.tagDialog.add': 'Adicionar tag',

  'images.transfer.savingOne': 'Salvando {name}',
  'images.transfer.saving': 'Salvando {count, plural, one {# imagem} other {# imagens}}',
  'images.transfer.to': 'Para {path}',
  'images.transfer.from': 'De {path}',
  'images.transfer.written': 'Gravado',
  'images.transfer.sent': 'Enviado',
  'images.transfer.loading': 'Carregando imagens',
  'images.transfer.importing': 'Importando {reference}',
  'images.transfer.doneWith': 'Concluído: {images}',
  'images.palette.pull': 'Baixar uma imagem',
} satisfies Translation<typeof en>;
