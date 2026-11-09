---
title: Releases de Helm
description: Lista, instala, actualiza, revierte y desinstala los releases de Helm de un clúster, y gestiona los repositorios de charts, mediante la CLI helm de este equipo.
---

**Helm**, en la barra lateral de un clúster, muestra los releases de [Helm](https://helm.sh) que hay en él: los mismos releases que muestra `helm list` en un terminal, se hayan instalado desde Homeport o no.

## Qué necesita Homeport

Homeport ejecuta el comando `helm`, versión 3 o posterior, en **este** equipo. Instálalo con `brew install helm` en macOS, `winget install Helm.Helm` (o Chocolatey, o Scoop) en Windows, y `snap install helm --classic`, el paquete de tu distribución o el script de helm.sh en Linux. **Ajustes → Este equipo** muestra el helm que encontró Homeport; sin él, la página Helm indica qué instalar en tu sistema, y todas las demás páginas del clúster funcionan.

Homeport no le da tu kubeconfig a helm. Para cada comando de helm, abre una conexión privada con el clúster en la dirección de loopback de este equipo y le da a helm un kubeconfig temporal que apunta a ella, con un token de un solo uso y ninguna credencial del clúster. Homeport añade por el camino las propias credenciales del clúster (certificados, tokens, tokens de los plugins exec) a cada solicitud, como en todas las demás páginas, y elimina el kubeconfig cuando termina el comando. Así, helm alcanza todos los clústeres que alcanza Homeport, incluidos los clústeres de la nube con plugins exec, y no se escribe ninguna credencial en el disco.

Helm guarda sus releases como Secrets en el Namespace de cada release: Homeport no instala nada en el clúster.

## Releases

La lista muestra el nombre de cada release, su Namespace, su chart y la versión del chart, la versión de la app, la revisión, el estado y cuándo se modificó por última vez. Elige un Namespace arriba a la derecha, o todos. helm no tiene actualizaciones en directo, así que la lista vuelve a preguntar cada 30 segundos mientras está abierta, y **Actualizar** pregunta al instante.

Un release **failed** se marca en rojo, y uno **pending** de una instalación, una actualización o una reversión (`pending-install`, `pending-upgrade`, `pending-rollback`), en ámbar: un release pendiente queda bloqueado hasta que termina esa operación.

Si tus credenciales no pueden listar los Secrets de todos los Namespaces, Homeport lista en su lugar los releases del Namespace predeterminado del clúster y lo indica; elige otro Namespace para ver sus releases.

## Instalar un chart

**Instalar chart** (o <kbd>n</kbd>) en la pestaña Releases instala un chart en el clúster seleccionado:

1. **Elige el chart.** Busca entre los charts de los [repositorios](#repositorios-de-charts) de helm y haz clic en **Usar** o, en **Desde un registro OCI**, indica una referencia como `oci://registry-1.docker.io/bitnamicharts/nginx`, que se instala sin añadir un repositorio.
2. **Elige su versión, su nombre y su Namespace.** Las versiones de un chart de repositorio se listan, la más reciente primero; la versión de un chart OCI se escribe, o se deja vacía para la más reciente. Al principio, el release recibe el nombre del chart: como máximo 53 letras minúsculas, dígitos, guiones y puntos. El Namespace es el que muestra la página (o el predeterminado del clúster), y se crea si no existe, salvo que desmarques esa opción.
3. **Edita sus valores.** El editor empieza con los valores predeterminados del chart (`helm show values`); cambia lo que necesites, y **Restablecer los valores predeterminados del chart** los recupera. Los valores se comprueban como YAML antes de que se ejecute helm, y llegan a helm por su entrada estándar, nunca en un archivo.
4. **Previsualizar**, opcionalmente: helm renderiza el chart con tus valores y con la ayuda del clúster (`--dry-run=server`), y Homeport muestra el manifiesto que aplicaría, hooks incluidos, y las notas del chart. No cambia nada en el clúster.
5. **Instalar.** La salida de helm aparece en directo, tal como la escribe `helm install`. Con **Esperar a que sus recursos estén listos**, helm espera a que los Deployments, Pods y Services del release estén listos, durante un máximo de los minutos indicados (5 de forma predeterminada), y marca el release como fallido si no lo están. **Abrir release** abre entonces su página.

Los charts de una carpeta local o de un archivo `.tgz` no se instalan desde Homeport; tampoco los charts de registros OCI que requieren inicio de sesión, salvo que hayas iniciado sesión con `helm registry login` en un terminal.

## La página de un release

Haz clic en un release para abrirlo:

- **Historial**: todas las revisiones, la más reciente primero, con su fecha, su estado, su chart, su versión de la app y la descripción de helm de lo que pasó (`Install complete`, `Upgrade "web" failed: …`).
- **Valores**: los valores indicados al instalar o actualizar (**Proporcionados por el usuario**), o todos los valores con los que se renderizó el chart, incluidos sus valores predeterminados (**Calculados**).
- **Notas**: lo que escribe el chart después de una instalación (su `NOTES.txt`).
- **Manifiesto**: los objetos de Kubernetes que renderizó la revisión, como YAML.
- **Recursos**: esos objetos por kind, nombre y Namespace, y cada uno abre su página en Homeport: su propia página para los Deployments, los Pods, los Services y similares, y su página de [Recursos de la API](/docs/clusters/api-resources/) para el resto.

Valores, Notas, Manifiesto y Recursos muestran la revisión actual; elige una anterior encima de ellos para ver con qué se instaló.

## Actualizar un release

**Actualizar**, en la página de un release, cambia la versión de su chart, sus valores o ambas cosas:

- **Chart**: el chart desde el que actualizar. helm no registra de dónde vino un chart, así que Homeport busca un chart con ese nombre en tus repositorios, primero los que tienen la versión actual del release. Elige otro con **Otro chart…**, como `repository/chart` o como una referencia `oci://`, que es lo que necesita un release instalado desde un registro OCI.
- **Versión del chart**: las versiones de ese chart, la más reciente primero, o escrita para un chart OCI.
- **Valores**: el editor empieza con los valores proporcionados por el usuario del release. Sustituyen a los del release: los valores predeterminados del chart completan el resto. **Cambios en los valores** los muestra junto a los actuales, y **Restablecer los valores actuales** los recupera.
- **Esperar a que sus recursos estén listos**, durante un máximo de los minutos indicados.

**Previsualizar** renderiza la actualización con la ayuda del clúster sin cambiar nada, y muestra el nuevo manifiesto y lo que cambia en él respecto al actual. **Actualizar** ejecuta `helm upgrade`, con la salida de helm en directo. Cada actualización es una nueva revisión en **Historial**.

## Revertir

En **Historial**, **Revertir a la revisión N** en cualquier revisión anterior pregunta antes, indicando las versiones del chart y de la app ahora y después, y luego ejecuta `helm rollback`: helm restaura el chart y los valores de esa revisión como una nueva revisión, así que una reversión también se puede revertir. Marca **Esperar a que sus recursos estén listos** para que helm los espere.

Un release que se queda en **pending** tras una instalación, una actualización o una reversión interrumpida está bloqueado: helm se niega a cambiarlo hasta que termina. Revertirlo a su última revisión desplegada lo desbloquea.

## Desinstalar

**Desinstalar**, en la página de un release, indica el release, su Namespace y su clúster, y lista los objetos que se eliminarán con él, antes de que se ejecute `helm uninstall`. Los objetos que un chart marca para conservar (`helm.sh/resource-policy: keep`) y los PersistentVolumeClaims de los StatefulSets se quedan en el clúster. **Conservar su historial** deja el release en la lista como `uninstalled`, para poder revertirlo; sin esa opción, el release desaparece de la lista. **Esperar a que se eliminen sus objetos** hace que helm los espere.

## Repositorios de charts

**Repositorios de charts**, la segunda pestaña de la página Helm, lista los repositorios que conoce helm en este equipo: la misma lista que `helm repo list` en un terminal, compartida por todos los clústeres. Homeport no tiene una lista propia, así que un repositorio añadido en un terminal aparece aquí, y uno añadido aquí aparece en `helm repo list`.

- **Añadir repositorio** pide un nombre (letras, dígitos, puntos, guiones y guiones bajos) y la URL `https://` del repositorio, como `https://charts.bitnami.com/bitnami`, y descarga su índice.
- **Actualizar todos**, o **Actualizar** en un repositorio, descarga los índices más recientes, como hace `helm repo update`, con la salida de helm en directo. Los charts publicados desde la última actualización solo aparecen después de una.
- **Quitar** olvida un repositorio, después de que lo confirmes. Los releases instalados desde él siguen ejecutándose.

**Buscar charts** busca entre los charts de estos repositorios por nombre o descripción, como hace `helm search repo`: escribe `postgres` después de añadir el repositorio de Bitnami para ver sus charts de PostgreSQL, con sus versiones más recientes del chart y de la app.

Homeport solo añade repositorios públicos. Uno que requiere inicio de sesión se añade en un terminal, con `helm repo add --username …`, donde helm guarda su contraseña como siempre; después aparece aquí y funciona como los demás. Un chart de un registro OCI (`oci://…`) no necesita ningún repositorio.

## Cuando no funciona

- **La CLI helm no está instalada**: instala helm 3 como se indica arriba y vuelve a abrir la página.
- **Helm 2**: Homeport solo maneja Helm 3 y posteriores. Instala un helm actual.
- **No permitido**: helm lee los releases de los Secrets; listarlos requiere `list` sobre `secrets` en el Namespace. Detalles indica lo que se rechazó.
- **Hay otra instalación, actualización o reversión en curso**: el release está bloqueado por una operación pendiente. Espera a que termine, o revierte a su última revisión desplegada.
- **Ya existe un release con este nombre**: los nombres de release son únicos en un Namespace. Elige otro, o actualiza el release existente.
- **No se encontró el chart o la versión**: comprueba el nombre y la versión; un chart de repositorio necesita que su repositorio esté añadido y [actualizado](#repositorios-de-charts). Para actualizar un release cuyo chart no está en ninguno de tus repositorios, añade primero ese repositorio.
- **Los valores no son YAML válido**: el mensaje indica la línea; corrígela y vuelve a intentarlo.
- **Se dejó de esperar**: con **Esperar**, un recurso no estuvo listo a tiempo. Mira los Pods y los Events del release para ver qué está atascado.
- **No es un repositorio de charts o no se puede alcanzar**: la URL no sirve ningún `index.yaml`, o este equipo no puede alcanzarla. Detalles muestra lo que dijo helm.
- **No conectado**: helm pasa por la conexión del clúster en Homeport; la página del clúster dice por qué no está conectado.
