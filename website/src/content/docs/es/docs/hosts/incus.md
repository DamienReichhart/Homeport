---
title: Incus
description: Conéctate a Incus en este equipo, por SSH o por HTTPS con un token de confianza, y elige el proyecto que muestran sus páginas.
---

**Incus** ejecuta contenedores de sistema y máquinas virtuales, las **instancias** de [la guía](/docs/guides/instances/). Homeport se conecta a él a través de su propia API REST, como decide el [ADR 0021](https://github.com/DamienReichhart/Homeport/blob/main/docs/adr/0021-incus-over-its-rest-api.md). Un host de Incus tiene sus propias páginas en la barra lateral: **Instancias**, **Imágenes**, **Perfiles**, **Almacenamiento** y **Redes**. Las páginas de Docker, los stacks y el Tablero no se ofrecen en él.

## Añadir un host de Incus

Elige **Incus** en el campo **Engine** del formulario y luego cómo acceder a él:

| Conexión         | Cuándo                                                          | Qué necesita                                                                 |
| ---------------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| **Socket local** | Incus se ejecuta en este equipo (Linux)                         | tu usuario en el grupo `incus-admin`, o `incus` (ver abajo)                  |
| **SSH**          | Incus se ejecuta en un servidor en el que puedes iniciar sesión | el usuario SSH en el grupo `incus-admin`, o `incus`                          |
| **TLS** (HTTPS)  | Incus escucha en la red, o SSH no está permitido                | un token de confianza, o un certificado de cliente en el que Incus ya confía |

**TCP** sin cifrar aparece atenuado: Incus no sirve su API sin TLS. Homeport se conecta a **Incus 6.0** (la versión de soporte a largo plazo) y posteriores.

### Sus sockets

| Socket                            | Quién puede usarlo              | A qué llega                                                  |
| --------------------------------- | ------------------------------- | ------------------------------------------------------------ |
| `/var/lib/incus/unix.socket`      | el grupo `incus-admin`          | a todo: todos los proyectos, pools de almacenamiento y redes |
| `/var/lib/incus/unix.socket.user` | el grupo `incus` (`incus-user`) | solo al proyecto propio del usuario, creado en el primer uso |

En Linux, **Encontrados en este equipo** lista los dos cuando responden, el restringido como **Incus (restricted to your project)**. Un socket que todavía no puedes usar se lista con el comando que lo soluciona:

```bash
sudo usermod -aG incus-admin "$USER"
```

Los cambios de grupo se aplican a los nuevos inicios de sesión: cierra la sesión y vuelve a iniciarla (o vuelve a conectarte por SSH) antes de añadir el host. Un Incus detenido indica qué unidades lo inician:

```bash
sudo systemctl enable --now incus.socket incus.service
```

### Por HTTPS con un token de confianza

Incus escucha en la red una vez que se define `core.https_address`. En el servidor de Incus:

```bash
incus config set core.https_address :8443
incus config trust add homeport
```

El segundo comando muestra un **token de confianza**: una línea de letras y dígitos. En Homeport, elige **Incus** y **TLS**, deja **Token de confianza** y pégalo. Homeport muestra lo que lee (el nombre para el que es, las direcciones del servidor y la huella de su certificado, y cuándo caduca) y luego, al elegir **Añadir**:

1. crea su propio certificado de cliente, con una clave ECDSA P-384 creada en memoria;
2. prueba las direcciones del token por orden y se queda con la primera cuyo certificado tiene la huella que indica el token; cualquier otra se rechaza, mostrando las dos huellas;
3. envía el token con su certificado y comprueba que Incus ahora confía en él.

El host aparece entonces en `incus config trust list` con el nombre del token. Un token sirve una sola vez y caduca; uno usado, revocado o caducado se rechaza, junto con el comando que crea uno nuevo. Para revocar más adelante a Homeport, elimina allí su certificado:

```bash
incus config trust list
incus config trust remove <fingerprint>
```

**Certificado existente** acepta un certificado y una clave de cliente en los que Incus ya confía (`incus config trust add-certificate`), junto con el propio certificado de Incus en el que confiar (`/var/lib/incus/server.crt`, o `cluster.crt` en un clúster).

### El certificado del servidor queda fijado

El certificado de Incus está autofirmado, así que no hay ninguna CA con la que comprobarlo. Homeport guarda con el host el certificado en el que confió y después acepta exactamente ese, comparado por su huella SHA-256, sea cual sea la dirección por la que se acceda a él. Si Incus se reinstala o se renueva su certificado, el host se rechaza mostrando la huella que esperaba y la que recibió: comprueba la nueva en el servidor con `incus info` y luego vuelve a añadir el host con un token nuevo. La clave de cliente y los dos certificados se guardan cifrados como cualquier otro secreto; consulta [seguridad](/docs/reference/security/#incus-certificates).

## Proyectos

Incus guarda las instancias, imágenes, perfiles, volúmenes personalizados y redes en **proyectos**. Las páginas muestran un proyecto a la vez, elegido bajo el selector de hosts, que lista cada uno con cuántas instancias e imágenes contiene. Homeport recuerda la elección para cada host entre sesiones. Hasta que eliges uno, un host muestra `default`, o su único proyecto en el socket restringido. Los pools de almacenamiento pertenecen a todo el servidor; sus volúmenes se muestran para el proyecto seleccionado.

## Eventos

Las páginas se actualizan a partir de los eventos de ciclo de vida de Incus, en todos los proyectos: una instancia que se inicia o se detiene, una instantánea tomada, una imagen añadida, un perfil o una red modificados. También se conservan en la página **Eventos**. Incus no guarda historial de ellos, así que lo que pasó mientras Homeport estaba cerrado no se recupera.

Mientras está conectado, Homeport le pide a Incus `GET /1.0` cada 15 segundos. Un Incus detenido deja el host como **Inaccesible**, y Homeport vuelve a conectarse solo cuando responde de nuevo.

## Lo que hace Homeport en el host

En un host de Incus no se ejecuta nada aparte del propio Incus: las consolas y los terminales pasan por los websockets de Incus, y las imágenes remotas las descarga Incus. Homeport lee desde este equipo la lista de imágenes de un servidor de imágenes remoto, para los selectores de imagen.
