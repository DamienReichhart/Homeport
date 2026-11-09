/** Spanish text of the `board` namespace. */
import type en from '../en/board';
import type { Translation } from '../types';

export default {
  'board.nav': 'Tablero',
  'board.title': 'Tablero',
  'board.summary':
    '{running} de {count, plural, one {# contenedor} other {# contenedores}} en ejecución en {hosts, plural, one {# host} other {# hosts}}',
  'board.column.host': 'Host',
  'board.column.stack': 'Stack',
  'board.sortBy': 'Ordenar por {column}',
  'board.filter.hosts': 'Hosts',
  'board.filter.allHosts': 'Todos los hosts',
  'board.filter.someHosts': '{count, plural, one {# host} other {# hosts}}',
  'board.filter.state': 'Estado',
  'board.filter.stack': 'Stack',
  'board.filter.anyStack': 'Cualquier stack',
  'board.filter.clear': 'Borrar filtros',
  'board.state.all': 'Cualquier estado',
  'board.state.running': 'En ejecución',
  'board.state.stopped': 'Detenidos',
  'board.state.unhealthy': 'No saludables',
  'board.search': 'Buscar en todos los hosts',
  'board.skipped': '{name} ({status}) no se muestra.',
  'board.openHost': 'Abrir host',
  'board.listFailed': 'No se pudieron listar los contenedores de {name}.',
  'board.noHosts.title': 'No hay ningún host conectado',
  'board.noHosts.description': 'El Tablero lista los contenedores de todos los hosts conectados.',
  'board.empty.title': 'Aún no hay contenedores',
  'board.empty.description': 'Ninguno de tus hosts conectados ejecuta un contenedor.',
  'board.noMatch': 'Ningún contenedor coincide con estos filtros.',
  'board.selection': 'Contenedores seleccionados',
  'board.selectAll': 'Seleccionar todos los contenedores mostrados',
  'board.selectOne': 'Seleccionar {name} en {host}',
  'board.remove.title': '¿Eliminar {count, plural, one {# contenedor} other {# contenedores}}?',
  'board.remove.description':
    'Se eliminan junto con sus volúmenes anónimos; los volúmenes con nombre se conservan. Los que están en ejecución se detienen antes.',
  'board.remove.onHost': 'En {host}: {names}',
} satisfies Translation<typeof en>;
