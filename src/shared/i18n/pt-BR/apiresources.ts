/** Brazilian Portuguese text of the `apiresources` namespace. */
import type en from '../en/apiresources';
import type { Translation } from '../types';

export default {
  'apiresources.nav': 'Recursos da API',
  'apiresources.title': 'Recursos da API',
  'apiresources.summary':
    '{count, plural, one {# tipo servido} other {# tipos servidos}} por este cluster',
  'apiresources.refresh': 'Atualizar',
  'apiresources.types': 'Tipos de recurso',
  'apiresources.search': 'Kind ou nome curto (pvc)',
  'apiresources.searchObjects': 'Pesquisar',
  'apiresources.noMatch': 'Nenhum tipo corresponde.',
  'apiresources.coreGroup': 'core',
  'apiresources.pick':
    'Escolha um tipo para listar seus objetos: os integrados e os que seus CRDs adicionam.',
  'apiresources.typeGone': 'Este cluster não serve esse tipo (não mais).',
  'apiresources.cantList': 'Não pode ser listado',
  'apiresources.notAllowed': 'Suas credenciais não podem listá-lo',
  'apiresources.cantListWhy':
    'Objetos {kind} não podem ser listados: a API deles só permite criá-los ou lê-los um a um.',
  'apiresources.empty': 'Nenhum aqui.',
  'apiresources.column.name': 'Nome',
  'apiresources.column.namespace': 'Namespace',
  'apiresources.column.age': 'Idade',
  'apiresources.tab.overview': 'Visão geral',
  'apiresources.tab.conditions': 'Condições',
  'apiresources.tab.events': 'Eventos',
  'apiresources.tab.source': 'YAML',
  'apiresources.detail.namespace': 'Namespace',
  'apiresources.detail.owners': 'Pertence a',
  'apiresources.detail.age': 'Idade',
  'apiresources.detail.labels': 'Rótulos',
  'apiresources.detail.annotations': 'Anotações',
  'apiresources.edit': 'Editar',
  'apiresources.copyYaml': 'Copiar YAML',
} satisfies Translation<typeof en>;
