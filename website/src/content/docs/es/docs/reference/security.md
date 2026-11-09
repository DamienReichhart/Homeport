---
title: Seguridad y tus datos
description: Qué guarda Homeport, dónde, cómo se cifran los secretos y qué no hace nunca.
---

Homeport guarda credenciales que abren tus servidores. Esta página explica exactamente qué guarda y cómo.

## Qué guarda Homeport

Todo vive en una base de datos SQLite, `homeport.db`, en la carpeta de datos de aplicación de tu usuario:

| Sistema | Carpeta                                           |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

Contiene:

| Datos                                                             | Se guarda        |
| ----------------------------------------------------------------- | ---------------- |
| Nombres de host, direcciones, usuarios SSH y rutas de socket      | Texto sin cifrar |
| Claves públicas SSH y las claves de servidor que aceptaste        | Texto sin cifrar |
| Claves privadas SSH y sus frases de contraseña                    | **Cifrado**      |
| Certificados de CA TLS, certificados y claves de cliente          | **Cifrado**      |
| Claves de cliente y certificados fijados de los hosts de Incus    | **Cifrado**      |
| Archivos compose de los stacks gestionados                        | Texto sin cifrar |
| Entornos de los stacks gestionados                                | **Cifrado**      |
| Carpetas y rutas de archivos compose de los stacks vinculados     | Texto sin cifrar |
| Historial de despliegues: archivos compose y perfiles             | Texto sin cifrar |
| Historial de despliegues: entornos                                | **Cifrado**      |
| Direcciones de registros y nombres de usuario                     | Texto sin cifrar |
| Contraseñas y tokens de acceso de registros                       | **Cifrado**      |
| URL de repositorios, mitades públicas de las claves de despliegue | Texto sin cifrar |
| Claves de servidor de repositorios que aceptaste                  | Texto sin cifrar |
| Claves de despliegue y tokens de acceso de repositorios           | **Cifrado**      |
| Nombres de clústeres, URL de servidores de API y namespaces       | Texto sin cifrar |
| Cómo inicia sesión cada clúster, su contexto de kubeconfig        | Texto sin cifrar |
| CA, certificados y claves de cliente de los clústeres             | **Cifrado**      |
| Tokens, contraseñas y tokens OIDC de los clústeres                | **Cifrado**      |
| Comandos, argumentos y entorno de los plugins exec                | **Cifrado**      |
| Tu tema, la última selección y el namespace de cada clúster       | Texto sin cifrar |
| Los eventos del Engine de cada host, para la página Eventos       | Texto sin cifrar |

Homeport no copia nada más de tus Engines: los contenedores, las imágenes, los volúmenes y los logs se leen en directo y nunca se guardan, salvo cuando tú mismo descargas logs o archivos, o haces una copia de seguridad de un volumen. Los eventos del Engine que guarda —qué le pasó a qué contenedor, imagen, volumen o red, con sus etiquetas— se conservan 30 días por defecto ([Ajustes → Datos](/docs/guides/settings/#datos)), como máximo 10 000 por host, y se eliminan con su host o con **Borrar historial**.

### Carpetas de los stacks vinculados

Para un [stack vinculado](/docs/guides/stacks/#stacks-vinculados), Homeport guarda solo dónde están su carpeta y sus archivos compose, y una huella de los archivos para saber cuándo cambian, nunca una copia de ellos. Lee los archivos compose cuando abres el stack, y solo escribe en ellos cuando eliges **Guardar** en su pestaña **Definición**, sustituyendo cada archivo editado de una sola vez y conservando sus permisos; no sobrescribe un archivo que haya cambiado desde que lo abriste salvo que se lo indiques. Homeport nunca lee ni escribe el `.env` de la carpeta: Compose lo lee cuando se ejecuta. Olvidar un stack vinculado nunca toca la carpeta.

### Repositorios

Para los [stacks sincronizados](/docs/guides/synced-stacks/), Homeport guarda la URL de cada repositorio, y o bien una clave de despliegue que generó para ese repositorio (con su mitad privada cifrada), o bien tu nombre de usuario y un token de acceso (cifrado). El contenido de un repositorio no es un secreto: su clon está en una carpeta `repositories` junto a `homeport.db`.

### Swarms

Homeport no guarda nada sobre un [Swarm](/docs/guides/swarm/): lo encuentra a través de sus hosts manager cada vez que se conecta. Los valores de los secretos de Swarm van al Swarm y Homeport no los conserva: un valor que escribes o cargas desde un archivo se envía una vez al Engine, nunca se escribe en `homeport.db`, en un archivo ni en el log, y se descarta de la memoria cuando se cierra el diálogo. El Engine nunca devuelve el valor de un secreto, así que Homeport tampoco puede mostrarlo. Los tokens de unión se obtienen de un manager cuando los necesitas, y nunca se guardan ni se registran en el log.

### Clústeres

Las credenciales de un [clúster](/docs/clusters/) vienen de tu kubeconfig y solo se guardan cifradas, como las claves SSH; los archivos a los que hace referencia el kubeconfig se leen una vez y se integran, un kubeconfig pegado nunca se escribe en un archivo, y tu kubeconfig nunca se modifica. Un [plugin exec](/docs/clusters/exec-plugins/) se ejecuta en este equipo, como tú, y el token o el certificado que devuelve se guarda solo en memoria, hasta que caduca. Homeport no guarda nada más sobre lo que se ejecuta en un clúster: los Pods, las cargas de trabajo y lo demás se leen en directo. Los [reenvíos de puerto](/docs/clusters/port-forward/) escuchan solo en el loopback de este equipo (`127.0.0.1` y `::1`), así que nada más de tu red puede usarlos, y terminan cuando Homeport se cierra. Los cambios hechos con [Aplicar YAML o Editar](/docs/clusters/yaml/) quedan registrados en el clúster con el field manager `homeport`.

[Helm](/docs/clusters/helm/) ejecuta tu CLI `helm` contra un clúster sin tu kubeconfig y sin escribir ninguna credencial del clúster en el disco: para cada comando, Homeport abre una conexión privada en el loopback de este equipo (`127.0.0.1`, en un puerto aleatorio) que solo acepta peticiones con un token aleatorio, y le da a helm un kubeconfig temporal, legible solo por ti, que contiene ese token y nada más. Homeport añade las credenciales propias del clúster a cada petición por el camino, y borra el kubeconfig y cierra la conexión cuando termina el comando. Los valores de los charts llegan a helm por su entrada estándar, nunca en un archivo. Los repositorios de charts, los inicios de sesión de registros y las cachés propios de helm siguen siendo de helm, como en un terminal; Homeport solo añade repositorios públicos y no guarda ninguna contraseña de repositorio.

El token ACL, el certificado de CA y el certificado y la clave de cliente de un [clúster de Nomad](/docs/clusters/nomad/) solo se guardan cifrados, como las claves SSH; su dirección, su región y su nombre de servidor TLS se guardan sin cifrar. El token viaja en la cabecera `X-Nomad-Token` de cada petición, nunca en una URL, ni siquiera en el WebSocket de un shell. Homeport no guarda ninguna copia de la especificación de un job: Nomad la guarda con el job.

**Los valores de los Secrets de Kubernetes** se obtienen solo cuando eliges **Mostrar** para una clave, de una en una: las listas, la página de un Secret y su YAML llevan sus claves y tamaños, nunca sus valores. Un valor mostrado se queda en la memoria de esa página —no en una caché, no en `homeport.db`, no en el log— y se vuelve a ocultar cuando sales de la página.

## Cómo se cifran los secretos

- Cada secreto se sella por separado con **XChaCha20-Poly1305**, un cifrado autenticado, así que se detecta cualquier manipulación además de impedir su lectura.
- La clave de 256 bits es aleatoria, se crea en el primer arranque y se guarda en el **llavero de tu sistema** —el Llavero de macOS, el Administrador de credenciales de Windows o el Secret Service en Linux— con el servicio `com.homeport.app`. Nunca se guarda junto a los datos.
- No hay contraseña maestra: el llavero ya vincula la clave a tu cuenta de usuario.

Alguien con una copia de `homeport.db` y nada más puede leer los nombres y las direcciones de los hosts y los clústeres, pero no tus claves, certificados, tokens, entornos de stacks ni contraseñas de registros.

### Sin llavero

Homeport muestra un aviso y no guarda secretos. Los hosts de socket local, TCP y ssh-agent siguen funcionando. Consulta los [requisitos](/docs/requirements/#opcional-un-llavero-del-sistema).

### Si se pierde la clave del llavero

Restablecer el llavero, o mover `homeport.db` a otro equipo o a otro usuario, deja secretos que no se pueden descifrar. Homeport no borra nada: los hosts afectados muestran **Credenciales perdidas** hasta que vuelvas a introducir sus credenciales. Consulta [solución de problemas](/docs/reference/troubleshooting/#credenciales-perdidas). Para mover tus datos, haz una copia de seguridad o exporta tus hosts.

### Pasar a otro equipo

**Exportar**, en la página Hosts, escribe los hosts que elijas en un archivo, con sus claves, certificados y claves de servidor aceptadas, cifrado con una frase de contraseña que tú eliges:

- Todo el archivo está cifrado (XChaCha20-Poly1305, con la clave derivada de la frase de contraseña mediante Argon2id), así que sin la frase de contraseña no revela nada, ni siquiera los nombres o las direcciones de los hosts. Cualquier cambio en el archivo lo vuelve ilegible.
- La frase de contraseña necesita al menos 12 caracteres. Homeport nunca la guarda ni la registra en el log; si la pierdes, el archivo no se puede abrir.
- **Importar**, en el otro equipo, pide la frase de contraseña y luego guarda los secretos de cada host cifrados con la clave del llavero de ese equipo, como cualquier otro.
- Un host cuyas credenciales ya se han perdido en este equipo se exporta sin ellas, y la exportación indica cuáles.

Trata el archivo como las claves que contiene: bórralo una vez importado. El formato se describe en el [ADR 0013](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0013-passphrase-protected-export-bundles.md).

### Copias de seguridad

**Ajustes → Datos → Hacer copia de seguridad…** escribe todo lo anterior —hosts, clústeres, stacks, inicios de sesión de registros, repositorios y ajustes, con sus secretos descifrados— en un archivo con el mismo formato cifrado, con una frase de contraseña que escribes dos veces. Los eventos del Engine y el historial de despliegues no se incluyen. Un secreto que no se puede descifrar en este equipo se deja fuera, y la copia de seguridad indica cuál.

**Restaurar…** pide la frase de contraseña y lista lo que contiene el archivo antes de cambiar nada. A continuación, la restauración:

1. copia la base de datos actual en `homeport.db.before-restore-<date and time>`, junto a `homeport.db`;
2. sustituye todos los hosts, clústeres, stacks, inicios de sesión, repositorios y ajustes de una sola vez, de modo que un fallo deja tus datos como estaban;
3. vuelve a cifrar cada secreto con la clave del llavero de este equipo.

Una copia de seguridad hecha por un Homeport más reciente se rechaza; una hecha por un Homeport más antiguo se restaura. La copia de seguridad contiene todos los secretos que tiene Homeport: guárdala, junto con su frase de contraseña, con el mismo cuidado que las propias claves.

## Las claves nunca tocan el disco sin cifrar

Homeport tiene su propio cliente SSH, así que nunca escribe una clave privada en un archivo para que la lea `ssh`, y nunca modifica `~/.ssh`. Las claves descifradas se quedan en memoria.

Cuando `docker compose` despliega un stack en un host SSH o TLS, Homeport abre en este equipo un socket privado de corta duración —legible solo por tu usuario, o una tubería con nombre que rechaza clientes remotos en Windows— y lo reenvía por su propia conexión, para ese único comando.

Las claves y los tokens de los repositorios tampoco llegan nunca al disco. Para cada comando `git`, Homeport sirve un agente SSH privado que contiene solo la clave de despliegue de ese repositorio —en un socket que solo puede abrir tu usuario, o en una tubería con nombre que rechaza clientes remotos en Windows— o pasa el token en una cabecera HTTPS a través del entorno del comando. Tu propia configuración de git, tus asistentes de credenciales y tus claves SSH se dejan fuera, así que nada queda en caché en tu llavero ni en `~/.git-credentials`.

Los inicios de sesión de los registros tampoco llegan nunca a `docker compose`: Homeport descarga él mismo las imágenes de un stack, a través de la API del Engine, antes de ejecutar `docker compose up`, así que ni el entorno del comando ni los archivos que recibe contienen ninguna contraseña ni token.

## Confiar en los servidores

La primera vez que Homeport se conecta a un servidor SSH, te pide que confirmes la huella de la clave del servidor, y se niega a conectarse si esa clave cambia después, hasta que vuelvas a confirmarla. Los hosts TLS verifican el certificado del servidor con tu CA y con el nombre del servidor; los hosts de Incus por HTTPS solo aceptan el certificado con el que se añadieron. Consulta [Hosts SSH](/docs/hosts/ssh/#primera-conexión).

### Certificados de Incus

Añadir un [host de Incus por HTTPS](/docs/hosts/incus/#por-https-con-un-token-de-confianza) crea en memoria un certificado de cliente y su clave privada, una clave ECDSA P-384, y los guarda cifrados con los demás secretos del host, nunca sin cifrar en el disco. El propio certificado de Incus se guarda de la misma forma y queda fijado: Homeport acepta exactamente ese certificado del servidor, comparado por su huella SHA-256, y rechaza uno distinto, mostrando las dos huellas. El token de confianza se usa una vez y no se conserva. Elimina el certificado de Homeport en el servidor con `incus config trust remove` para revocarlo.

## A qué se conecta Homeport

- **A los hosts que añades.** Cuando descargas, subes o compilas una imagen, es el Engine quien contacta con el registro, no Homeport. Homeport le pasa con la petición el inicio de sesión guardado para ese registro, por la conexión del host; el Engine no lo conserva.
- **A los registros de tus imágenes, a través del Engine de cada host**, para saber si se ha publicado una [imagen más reciente](/docs/guides/containers/#imágenes-más-recientes): cuando se carga la página Contenedores, Imágenes o Stacks (cada respuesta se guarda durante 6 horas) y cuando eliges **Buscar imágenes más recientes**. El Engine le pide al registro el digest al que apunta una etiqueta, con el inicio de sesión guardado para los registros privados, y no descarga nada. Para un host local, el Engine se ejecuta en tu equipo, así que la petición sale de él.
- **A Docker Hub, cuando exploras etiquetas** en el diálogo de descarga: Homeport lista las etiquetas de un repositorio desde `hub.docker.com`, desde este equipo, con tu inicio de sesión de Docker Hub si hay uno guardado. La búsqueda de repositorios pasa en cambio por el Engine del host.
- **A un registro, cuando compruebas su inicio de sesión** en **Ajustes → Registros**: Homeport consulta el endpoint `/v2/` del registro, y su servicio de tokens, desde este equipo, enviando el nombre de usuario y la contraseña o el token por HTTPS (HTTP sin cifrar solo para un registro de este equipo, como `localhost:5000`).
- **A los servidores de tus repositorios**, para los [stacks sincronizados](/docs/guides/synced-stacks/): cuando pruebas el acceso, creas o sincronizas un stack sincronizado, y a intervalos mientras Homeport está abierto si el sondeo está activado. Homeport ejecuta `git` desde este equipo con la clave de despliegue o el token de ese repositorio.
- **A los clústeres que añades**, a través del servidor de API de Kubernetes que nombra su kubeconfig (y su proxy, cuando tiene uno), con las credenciales que da, comprobando el certificado del servidor con la CA del kubeconfig. Los plugins exec contactan con su propia nube (AWS, Google Cloud, Azure) desde este equipo, como hacen para `kubectl`.
- **A los clústeres de Nomad que añades**, a través de la dirección de agente que indicas, y nada más: los logs y los shells de un job pasan por ese agente, que los reenvía al nodo cliente, así que Homeport nunca se conecta a los propios nodos. El certificado del agente se comprueba con la CA que indicas, o con las CA de este equipo.
- **A los servidores de imágenes de Incus**, para los selectores de imagen de un [host de Incus](/docs/guides/instances/#crear-una-instancia): Homeport descarga la lista de imágenes de un servidor (`streams/v1/images.json`, de `images.linuxcontainers.org` salvo que elijas otro) desde este equipo, y la guarda diez minutos. Las imágenes en sí las descarga Incus.
- **A los repositorios de charts y registros OCI, a través de helm**, cuando añades o actualizas un repositorio, buscas charts, o instalas o actualizas un chart: `helm` descarga índices y charts desde este equipo, como hace en un terminal.
- **A este sitio web, para buscar actualizaciones**: al arrancar y cada 6 horas, Homeport descarga `updates/latest.json`, un archivo pequeño que indica la última versión. La petición no lleva nada sobre ti, tus hosts ni tus contenedores, pero, como cualquier petición web, llega a GitHub Pages, que aloja este sitio, desde tu dirección IP. Desactívalo en **Ajustes → Actualizaciones**. Las actualizaciones en sí se descargan de GitHub Releases, y Homeport solo instala una si está firmada con la clave de actualización de Homeport. Consulta [actualizar](/docs/reference/updating/).

## Lo que Homeport ejecuta en tus hosts

Homeport habla con el Engine de un host a través de su API, y solo ejecuta comandos en el servidor de un host para lo que esa API no puede hacer:

| Cuándo                                                                                             | Dónde                                                        | Comando                                                                                                                                         |
| -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| El servidor de un host SSH se niega a abrir el socket de su Engine                                 | el servidor SSH                                              | una comprobación en shell POSIX de si el socket existe y es legible, y de si hay un Engine instalado                                            |
| **Iniciar**, **Detener** o **Reiniciar** una [unidad Quadlet](/docs/guides/quadlet/)               | el servidor SSH, o este equipo para un socket local en Linux | `systemctl [--user] start\|stop\|restart '<unit>.service'`                                                                                      |
| Los [logs de un contenedor de containerd](/docs/hosts/containerd/#logs)                            | el servidor SSH, o este equipo para un socket local en Linux | `tail -n <lines> [-F] -- '<log file>'`, después de un `ls` de la carpeta de logs de un contenedor de Kubernetes cuando CRI no nombra el archivo |
| Un [terminal en un contenedor de containerd](/docs/hosts/containerd/#un-terminal-en-un-contenedor) | el servidor SSH, o este equipo para un socket local en Linux | `mktemp -d`, `mkfifo -m 600` para dos FIFO en esa carpeta, `cat` a través de ellos, y luego `rm -rf` de la carpeta                              |

Se ejecutan con los permisos del usuario SSH (o con los tuyos, en este equipo) y nunca con `sudo`. Los valores se comprueban y se entrecomillan antes de entrar en un comando. Instalar, editar y eliminar unidades Quadlet pasa por la API de Podman, que escribe los archivos por sí misma. En un host de Incus no se ejecuta nada: su consola y sus terminales pasan por la propia API de Incus. Los únicos archivos que Homeport crea en un host son los dos FIFO de un terminal de containerd, en una carpeta temporal privada que se elimina cuando se cierra el terminal.

## Lo que Homeport no hace nunca

- **Ninguna cuenta.** No hay nada en lo que registrarse.
- **Ni telemetría, ni analíticas, ni informes de fallos.**
- **Nada instalado en tus clústeres.** Homeport solo usa la API de Kubernetes, con los permisos de tus credenciales, y no cambia nada sin que se lo pidas: todo lo que elimina o sustituye algo pregunta antes, nombrando el clúster.
- **Nada instalado en tus hosts.** Los únicos comandos que Homeport ejecuta en un host están [listados arriba](#lo-que-homeport-ejecuta-en-tus-hosts). El único contenedor que Homeport inicia por su cuenta es un [contenedor auxiliar](/docs/guides/volumes-networks/#el-contenedor-auxiliar) de corta duración cuando abres, haces una copia de seguridad, restauras o clonas un volumen; no tiene red y se elimina cuando termina. Su imagen solo se descarga cuando das tu aprobación.
- **Ninguna instalación silenciosa.** Una actualización solo se instala cuando eliges **Instalar y reiniciar**; los paquetes de Linux solo los actualiza tu gestor de paquetes.

La [política de privacidad](/privacy/) explica lo que esto significa en términos legales: quién es responsable de tus datos, con qué base y cómo ejercer tus derechos.

## Logs

Homeport escribe un archivo de log pequeño —unas pocas decenas de kilobytes, que se sustituye cuando se llena— para diagnosticar problemas. Contiene eventos de conexión y errores, no tus credenciales ni la salida de los contenedores.

| Sistema | Carpeta                                 |
| ------- | --------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` |
| macOS   | `~/Library/Logs/com.homeport.app/`      |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` |

## Confiar en una descarga

Las releases las compila GitHub Actions a partir del código fuente público. Cada archivo viene con una suma de comprobación SHA-256 y una atestación firmada de la procedencia de la compilación que lo vincula al commit y al workflow exactos que lo compilaron. Consulta [verificar descargas](/docs/reference/verifying/).

## Notificar una vulnerabilidad

Notifica los problemas de seguridad de forma privada a través de la [notificación privada de vulnerabilidades de GitHub](https://github.com/DamienReichhart/Homeport/security/advisories/new), en lugar de en una issue pública.
