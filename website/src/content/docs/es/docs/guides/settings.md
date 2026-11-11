---
title: Ajustes
description: Tema e idioma, la bandeja del sistema, el inicio al iniciar sesión, las notificaciones, las actualizaciones, los inicios de sesión de registros, los repositorios, cada cuánto se comprueban los stacks sincronizados, lo que admite este equipo, la versión y las licencias.
---

**Ajustes** es la última página de la barra lateral, encima de los enlaces **Documentación** y **GitHub**, que se abren en tu navegador.

## Apariencia

**Tema** es **Igual que el sistema** por defecto, y sigue el modo claro u oscuro de tu sistema operativo a medida que cambia. Elige **Claro** u **Oscuro** para fijarlo.

**Idioma** es **Igual que el sistema** por defecto: Homeport se muestra en el idioma de tu sistema operativo cuando lo tiene, y en inglés si no. La opción indica el idioma que encontró, como **Igual que el sistema (Español)**. Elige un idioma para usar ese en su lugar; toda la ventana cambia al momento, sin reiniciar, y también el menú de la bandeja y las notificaciones.

Homeport habla inglés, francés (Français), español (Español), alemán (Deutsch), italiano (Italiano), portugués de Brasil (Português), ruso (Русский), chino simplificado (简体中文), japonés (日本語), coreano (한국어), hindi (हिन्दी) y árabe (العربية), que muestra de derecha a izquierda. Las variantes regionales siguen a su idioma: el francés de Canadá muestra francés y el portugués europeo muestra portugués de Brasil. El chino tradicional aún no está disponible y muestra inglés.

Las fechas, los números y los tamaños también siguen al idioma, con el estilo regional de tu sistema cuando tiene uno para él: _1.5 GB_ y _4 Nov 2026_ en inglés británico, _1,5 Go_ y _4 nov. 2026_ en francés. Los nombres, los ID, los comandos, los logs y lo que dicen los Engines y los clústeres se quedan como están, y los **Detalles** de un error conservan el texto original del sistema o la biblioteca que falló, para que un informe de error se lea igual en todos los idiomas.

## Escritorio

Cómo se ejecuta Homeport en este equipo.

- **Cerrar a la bandeja del sistema** — desactivado por defecto, así que cerrar la ventana cierra Homeport. Actívalo para ocultar la ventana en la [bandeja del sistema](/docs/guides/overview/#bandeja-del-sistema) en su lugar: Homeport sigue en ejecución con sus conexiones a los hosts abiertas. Recupéralo desde el icono de la bandeja, o abriendo Homeport de nuevo; sal con **Salir** en la bandeja. El interruptor aparece atenuado, con una explicación, en un escritorio sin bandeja.
- **Iniciar al iniciar sesión** — desactivado por defecto. Actívalo y Homeport se inicia cuando inicias sesión, así vigila tus hosts desde el principio. El interruptor muestra lo que tiene tu sistema: si quitas tú mismo Homeport de los elementos de inicio, aparece desactivado. En Windows aparece en **Task Manager → Startup apps**, en macOS en **System Settings → General → Login Items** y en Linux en `~/.config/autostart/`. La entrada de una AppImage sigue al archivo cuando lo mueves, a partir de la siguiente vez que lo abres.
- **Iniciar oculto en la bandeja** — desactivado por defecto, y solo disponible mientras **Iniciar al iniciar sesión** está activado. Cuando Homeport se inicia al iniciar sesión, se queda en la [bandeja del sistema](/docs/guides/overview/#bandeja-del-sistema) sin abrir su ventana. Si abres Homeport tú mismo, siempre muestra la ventana, y sin bandeja siempre se abre.

## Notificaciones

Homeport muestra una notificación de tu sistema operativo cuando ocurre algo en un host, tanto si su ventana está abierta, minimizada o [cerrada a la bandeja](#escritorio). Cada tipo tiene su propio interruptor, todos activados por defecto, y un interruptor surte efecto al momento:

| Interruptor                                                      | Ejemplo                                                                                 |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Un contenedor se detiene con un error o se queda sin memoria** | _web-1 se detuvo en prod-server (código de salida 137)_, _db se quedó sin memoria en …_ |
| **Un contenedor pasa a no saludable**                            | _api no está saludable en prod-server_                                                  |
| **Un host conectado pasa a ser inaccesible**                     | _prod-server es inaccesible: …_                                                         |
| **Un host inaccesible vuelve a estar disponible**                | _prod-server vuelve a estar conectado._                                                 |

Detener un contenedor — con **Detener** en Homeport, `docker stop`, `docker kill` o `docker compose down` — no es un fallo, así que no muestra nada; tampoco un contenedor que termina con el código 0. Un host que no logra conectarse cuando se inicia Homeport tampoco es una caída: solo lo es uno que estaba conectado. Un contenedor atascado en reinicios, o un host que se desconecta una y otra vez, avisa como máximo una vez cada 5 minutos.

Las notificaciones salen de las conexiones que Homeport ya mantiene, así que no se instala nada en tus hosts, y se detienen cuando cierras Homeport. Tu sistema operativo decide su aspecto y si se muestran: revisa sus ajustes de notificaciones para Homeport si no aparece ninguna.

## Actualizaciones

- **Buscar actualizaciones automáticamente** — activado por defecto: Homeport busca una versión nueva al iniciarse y cada 6 horas, y muestra una barra en la parte superior de la ventana cuando la hay.
- **Comprobar ahora** — comprueba al momento, incluso con las comprobaciones automáticas desactivadas, e indica si tienes la versión más reciente.

Cuando hay una actualización disponible, la tarjeta muestra lo que puedes hacer, que depende de cómo instalaste Homeport:

- **Windows, macOS y la AppImage** — **Instalar y reiniciar** descarga la actualización, comprueba su firma, la instala y vuelve a abrir Homeport.
- **Paquetes `.deb` y `.rpm`** — tu gestor de paquetes instala las actualizaciones, así que la tarjeta muestra el comando que hay que ejecutar, con un botón **Copiar comando**.

**Novedades** abre el [registro de cambios](/changelog/). Consulta [Actualizar](/docs/reference/updating/) para saber más.

## Registros

Los inicios de sesión de los registros de contenedores — Docker Hub, GitHub Container Registry (GHCR) o el tuyo propio — se guardan una vez en Homeport y se usan en todos los hosts: cuando descargas una imagen, ejecutas un contenedor a partir de una que aún no está, subes, compilas o despliegas un stack gestionado, vinculado o sincronizado. Hay un inicio de sesión por dirección de registro.

- **Añadir inicio de sesión** pide el **Registro** (`docker.io`, `ghcr.io`, `registry.example.com:5000`), el **Nombre de usuario** y la **Contraseña o token de acceso**. GHCR, y las cuentas de Docker Hub con autenticación en dos pasos, necesitan un token de acceso en lugar de la contraseña.
- **Comprobar inicio de sesión** pregunta al registro, desde este equipo, si acepta el inicio de sesión, como hace `docker login`, sin guardar nada. Un inicio de sesión rechazado lo indica con una pista; un registro al que Homeport no puede acceder indica eso en su lugar.
- **Editar** cambia la dirección o el nombre de usuario; deja la contraseña vacía para conservar la guardada. **Quitar** olvida el inicio de sesión: las imágenes de ese registro vuelven a descargarse de forma anónima.

Docker Hub se puede introducir como `docker.io`, `index.docker.io` o `https://index.docker.io/v1/`; son un mismo registro. Una imagen sin registro en su nombre, como `nginx` o `bitnami/redis`, usa el inicio de sesión de Docker Hub.

Las contraseñas y los tokens se cifran con la clave del llavero del sistema y nunca se vuelven a mostrar, ni se escriben en los logs ni en los detalles de los errores. Sin llavero, la tarjeta explica que no se pueden guardar inicios de sesión, y las imágenes se descargan de forma anónima. Consulta [Seguridad](/docs/reference/security/).

Homeport no lee `~/.docker/config.json` ni sus asistentes de credenciales.

## Repositorios

Los repositorios git desde los que se despliegan los [stacks sincronizados](/docs/guides/synced-stacks/), guardados una vez y utilizables en todos los hosts.

- **Añadir repositorio** pide la URL del repositorio y cómo accede Homeport a él: una **Clave de despliegue** que crea para ese repositorio (URL SSH), un **Token** con su nombre de usuario (URL HTTPS) o **Público** para uno que cualquiera puede leer.
- El botón del engranaje muestra la clave de despliegue de un repositorio, con dónde añadirla en GitHub, GitLab, Gitea y Forgejo, y **Nueva clave** para sustituirla; o **Sustituir el token**. **Probar acceso** lista sus ramas y etiquetas desde este equipo. Cuando Homeport ya confía en la clave de un servidor SSH, su huella aparece ahí, con **Restablecer** para cuando la forja la haya cambiado de verdad.
- **Quitar** olvida el repositorio y borra su clon en este equipo; se rechaza mientras lo use un stack sincronizado.

Las claves y los tokens se cifran con la clave del llavero del sistema y nunca se vuelven a mostrar, ni se escriben en los logs ni en los detalles de los errores. Tu propia configuración de git, tus asistentes de credenciales y tus claves SSH nunca se usan. Consulta [Repositorios](/docs/guides/synced-stacks/#repositorios).

## Stacks sincronizados

**Comprobar los repositorios cada** fija cuántos minutos espera Homeport entre comprobaciones de los repositorios de los stacks sincronizados en busca de commits nuevos: 5 por defecto, 1 como mínimo. Homeport solo comprueba mientras está abierto; nada se sincroniza mientras está cerrado. Consulta [Sondeo y despliegue automático](/docs/guides/synced-stacks/#sondeo-y-despliegue-automático).

## Datos

Lo que Homeport guarda en este equipo.

### Hacer y restaurar copias de seguridad

**Hacer copia de seguridad…** guarda todos los hosts, clústeres, stacks, inicios de sesión de registros, repositorios y ajustes, con sus claves, certificados, tokens, contraseñas y entornos, en un único archivo `.homeport`. Escribe dos veces una frase de contraseña de al menos 12 caracteres: todo el archivo se cifra con ella, y no se puede recuperar. Los eventos del Engine y el historial de despliegues no se incluyen, ni nada de lo que hay en tus hosts o clústeres: los contenedores, las imágenes, los volúmenes y los recursos de Kubernetes se quedan donde están.

**Restaurar…** abre un archivo así con su frase de contraseña y lista lo que contiene: sus hosts y clústeres por nombre, y cuántos stacks, inicios de sesión de registros y repositorios. **Sustituirlo todo** sustituye entonces todos los datos de Homeport por los de la copia de seguridad, en este u otro equipo:

- Todo se conecta y se despliega al momento: los hosts SSH y TLS, los registros y los repositorios, sin volver a pedir una clave o una contraseña, ni confiar en la clave de un servidor.
- Un [stack vinculado](/docs/guides/stacks/#stacks-vinculados) cuya carpeta no está en este equipo aparece como externo hasta que la carpeta esté ahí, en la misma ruta.
- Los [stacks sincronizados](/docs/guides/synced-stacks/) vuelven a clonar su repositorio en su siguiente sincronización.
- Antes, los datos actuales se copian en `homeport.db.before-restore-<date>`, en la carpeta de datos de Homeport. Restaurar un archivo equivocado, uno de un Homeport más reciente o uno que falla a mitad no cambia nada.

Para añadir algunos hosts a los que ya tienes, usa **Exportar** e **Importar** en la página Hosts. [Tus datos y copias de seguridad](/docs/reference/data-and-backups/) explica dónde están los archivos de Homeport, qué incluye una copia de seguridad y cómo copiar tú mismo la base de datos.

### Conservar los eventos del Engine

**Conservar los eventos del Engine durante** fija cuántos días se conservan los [eventos del Engine](/docs/guides/overview/#eventos) de cada host: 30 por defecto, de 1 a 365. Cada host conserva como máximo 10.000 eventos, diga lo que diga este ajuste. Los más antiguos se eliminan cuando se inicia Homeport y cada hora, y al momento cuando bajas el número.

## Este equipo

Lo que Homeport puede usar en este equipo:

- **Llavero del sistema** — **Disponible** cuando las claves SSH, los certificados, los entornos de los stacks y los inicios de sesión de registros se pueden guardar cifrados; **No disponible** si no. Consulta [Requisitos](/docs/requirements/#opcional-un-llavero-del-sistema).
- **CLI de Compose** — la versión de Compose encontrada, como `docker compose v2.29.1`, o **No encontrado** si los stacks se pueden iniciar y detener, pero no desplegar. Consulta [Stacks](/docs/guides/stacks/#qué-necesita-el-despliegue).
- **git** — el git encontrado, como `/usr/bin/git 2.43.0`, o qué instalar si no se pueden usar los stacks sincronizados. Consulta [Requisitos](/docs/requirements/#opcional-git).
- **CLI de Helm** — el helm encontrado, como `/usr/local/bin/helm v3.16.2`, o qué instalar para este sistema si no se pueden listar ni instalar [releases de Helm](/docs/clusters/helm/). Helm 2 se indica como no compatible. Consulta [Requisitos](/docs/requirements/#opcional-helm).

Homeport comprueba las CLI cada vez que se abre esta página, así que una instalada después del inicio aparece; después de instalar un llavero, cierra y vuelve a abrir Homeport.

## Acerca de

Muestra la versión de Homeport y la plataforma para la que se compiló, por ejemplo `v1.0.0 · macos/aarch64`.

**Abrir carpeta de logs** muestra dónde escribe Homeport su [log](/docs/reference/security/#logs).

**Exportar diagnóstico…** guarda un zip para adjuntar cuando [informes de un problema](https://github.com/DamienReichhart/Homeport/issues/new/choose): un resumen de las versiones y de los tipos de hosts y clústeres que usas, y los logs sin sus secretos. El diálogo muestra antes lo que contiene, y no se envía nada a ninguna parte. Consulta [Exportar el diagnóstico](/docs/reference/troubleshooting/#exportar-el-diagnóstico-para-un-informe-de-error).

**Licencias** muestra la licencia de Homeport (la GNU AGPL, versión 3), un enlace al código fuente de la versión que ejecutas y los avisos de todos los paquetes de código abierto con los que está creado.
