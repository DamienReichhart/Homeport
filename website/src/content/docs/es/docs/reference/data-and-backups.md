---
title: Tus datos y copias de seguridad
description: Dónde guarda Homeport sus datos en cada sistema, cómo hacer una copia de seguridad y restaurarla, y qué volver a introducir cuando se pierden los secretos.
---

Homeport guarda lo que sabe en tu equipo: cómo acceder a tus hosts y clústeres, las definiciones y los entornos de tus stacks, tus inicios de sesión de registros, tus repositorios y tus ajustes. Esta página indica dónde está todo eso, qué copiar y cómo recuperarlo. Para saber cómo se protege, consulta [Seguridad y tus datos](/docs/reference/security/).

Lo que se ejecuta en tus hosts y clústeres no son datos de Homeport: los contenedores, las imágenes, los volúmenes y los objetos de Kubernetes o de Nomad se quedan donde están, y ninguna copia de seguridad de Homeport los contiene. Para copiar el contenido de un volumen, [haz una copia de seguridad en un archivo](/docs/guides/volumes-networks/#hacer-y-restaurar-copias-de-seguridad) desde la página Volúmenes.

## Qué está dónde

### La carpeta de datos

| Sistema | Carpeta                                           |
| ------- | ------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/`                |
| macOS   | `~/Library/Application Support/com.homeport.app/` |
| Windows | `%APPDATA%\com.homeport.app\`                     |

En Linux, la carpeta está bajo `$XDG_DATA_HOME` cuando esa variable está definida. Si Homeport no puede abrir su base de datos, su ventana muestra la ruta completa del archivo, con **Abrir carpeta**.

| En la carpeta                        | Qué es                                                                                                                                                                                                            |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `homeport.db`                        | La base de datos: todos los hosts, clústeres, stacks, inicios de sesión de registros, repositorios y ajustes, el historial de despliegues y los eventos de los Engines. Los secretos que contiene están cifrados. |
| `homeport.db-wal`, `homeport.db-shm` | El registro de escritura anticipada de SQLite y su índice, presentes mientras Homeport se ejecuta. Los últimos cambios pueden estar solo en el archivo `-wal` hasta que Homeport se cierra.                       |
| `homeport.db.before-restore-<date>`  | Una copia de la base de datos hecha justo antes de cada [restauración](#restaurar-una-copia-de-seguridad). Borra las que ya no quieras.                                                                           |
| `repositories/` y `synced-stacks/`   | Los clones y las copias de trabajo de los repositorios de los [stacks sincronizados](/docs/guides/synced-stacks/). Homeport los vuelve a crear a partir de los propios repositorios.                              |

[Qué guarda Homeport](/docs/reference/security/#qué-guarda-homeport) lista lo que contiene la base de datos y qué partes están cifradas.

### La entrada del llavero

La clave que cifra los secretos no está en la carpeta. Es una entrada del llavero de tu sistema operativo, con el servicio `com.homeport.app` y la cuenta `secrets-key`: en **Acceso a Llaveros** en macOS, en **Administrador de credenciales → Credenciales de Windows** en Windows, y en **Contraseñas y claves** (Seahorse) o KWallet en Linux. Pertenece a tu cuenta de usuario en este equipo.

La base de datos y esa entrada van juntas. Una base de datos sin su clave se abre igualmente, con todos sus nombres, direcciones y archivos compose, pero ninguno de sus secretos se puede descifrar: los hosts y los clústeres muestran **Credenciales perdidas**. Por eso una copia de la carpeta no basta para pasar a otro equipo, y por eso se recomienda la [copia de seguridad desde la app](#hacer-una-copia-de-seguridad-en-homeport).

### Los logs y la posición de la ventana

| Sistema | Logs                                    | Tamaño y posición de la ventana                                     |
| ------- | --------------------------------------- | ------------------------------------------------------------------- |
| Linux   | `~/.local/share/com.homeport.app/logs/` | `~/.config/com.homeport.app/.window-state.json`                     |
| macOS   | `~/Library/Logs/com.homeport.app/`      | `~/Library/Application Support/com.homeport.app/.window-state.json` |
| Windows | `%LOCALAPPDATA%\com.homeport.app\logs\` | `%APPDATA%\com.homeport.app\.window-state.json`                     |

No merece la pena hacer copia de seguridad de ninguno de los dos. **Ajustes → Acerca de → Abrir carpeta de logs** abre los logs.

### Lo que sigue siendo tuyo

- **Las carpetas de los stacks vinculados.** Homeport solo recuerda dónde están la carpeta y los archivos compose de un [stack vinculado](/docs/guides/stacks/#stacks-vinculados). La carpeta, sus archivos compose y su `.env` son tuyos: haz copia de seguridad de ellos como de tus otros proyectos, normalmente con git.
- **Tu kubeconfig, `~/.ssh` y `~/.docker`.** Homeport los lee cuando importas desde ellos y nunca los cambia.
- **Los repositorios de charts y los inicios de sesión de helm.** Son de helm, y los comparte con tu terminal.

## Hacer una copia de seguridad en Homeport

**Ajustes → Datos → Hacer copia de seguridad…** lo escribe todo en un solo archivo `.homeport`. Es la copia de seguridad que conviene usar, porque lleva los propios secretos, protegidos con una frase de contraseña en lugar de con el llavero de este equipo: se restaura en cualquier equipo.

1. Abre **Ajustes** y, en **Datos**, elige **Hacer copia de seguridad…**.
2. Escribe dos veces una frase de contraseña de al menos 12 caracteres. Homeport nunca la guarda, y nada puede abrir el archivo sin ella.
3. Elige dónde guardar el archivo.

Después, Homeport indica lo que guardó y nombra todo lo que tuvo que guardar sin su secreto porque ese secreto no se podía descifrar en este equipo.

| En la copia de seguridad                                                                                        | Fuera de la copia de seguridad                                                             |
| --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| Los hosts, con sus claves, frases de contraseña, certificados, bastiones y las claves de servidor que aceptaste | Los eventos de los Engines                                                                 |
| Los clústeres, con sus certificados, tokens y ajustes de plugins exec                                           | El historial de despliegues                                                                |
| Los archivos compose y los entornos de los stacks gestionados                                                   | Las carpetas y los archivos de los stacks vinculados                                       |
| Las rutas de las carpetas y los archivos de los stacks vinculados                                               | Los clones de los stacks sincronizados (se vuelven a crear en la siguiente sincronización) |
| El repositorio, la rama o la etiqueta, los ajustes de sincronización y el entorno de los stacks sincronizados   | Los logs, y el tamaño y la posición de la ventana                                          |
| Los stacks desplegados en un Swarm                                                                              | Todo lo que hay en tus hosts y clústeres: contenedores, imágenes, volúmenes                |
| Los inicios de sesión de registros, con sus contraseñas y tokens                                                | Los repositorios de charts de helm                                                         |
| Los repositorios, con sus claves de despliegue y tokens                                                         | Los reenvíos de puerto, que terminan cuando Homeport se cierra                             |
| Tus ajustes                                                                                                     |                                                                                            |

Todo el archivo está cifrado con la frase de contraseña, así que sin ella el archivo no revela nada, ni siquiera el nombre de un host. Contiene todos los secretos que tiene Homeport: guárdalo, junto con su frase de contraseña, con el mismo cuidado que las propias claves. [Copias de seguridad](/docs/reference/security/#copias-de-seguridad) describe el cifrado.

Haz una copia de seguridad antes de probar una versión preliminar, antes de reinstalar tu sistema y siempre que hayas añadido hosts que no querrías volver a configurar.

### Restaurar una copia de seguridad

1. En este equipo o en otro, abre **Ajustes** y, en **Datos**, elige **Restaurar…**.
2. Elige el archivo `.homeport` y escribe su frase de contraseña. Homeport lista lo que contiene el archivo, y qué Homeport lo hizo y cuándo, antes de cambiar nada.
3. Elige **Sustituirlo todo**.

Restaurar sustituye todos los datos de Homeport por los de la copia de seguridad; no los combina. Homeport copia antes la base de datos actual en `homeport.db.before-restore-<date>` en la carpeta de datos y después lo sustituye todo en un solo paso, así que un fallo deja tus datos como estaban. Cada secreto se vuelve a cifrar con la clave del llavero de este equipo.

Después:

- Los hosts y los clústeres se vuelven a conectar al momento, sin pedir ninguna clave, contraseña ni clave de servidor que aceptar.
- Un stack vinculado cuya carpeta no está en este equipo, en la misma ruta, aparece como externo hasta que lo esté.
- Los stacks sincronizados vuelven a clonar su repositorio en su siguiente sincronización.
- La página Eventos y el historial de despliegues empiezan vacíos.

Una copia de seguridad hecha por un Homeport más antiguo se restaura en uno más reciente. Una hecha por un Homeport más reciente se rechaza: actualiza Homeport primero.

Para volver a lo que tenías antes de una restauración, sal de Homeport, cambia el nombre de `homeport.db` por otro, cambia el nombre del `homeport.db.before-restore-<date>` que quieras por `homeport.db`, borra los `homeport.db-wal` y `homeport.db-shm` que queden a su lado y vuelve a abrir Homeport.

### Mover solo algunos hosts

**Exportar** e **Importar**, en la página Hosts, mueven los hosts que elijas, con sus secretos, en un archivo del mismo tipo, y los añaden a los hosts que ya tiene el otro equipo. Consulta [pasar a otro equipo](/docs/reference/security/#pasar-a-otro-equipo).

## Copiar tú mismo los archivos

Una copia de la base de datos sirve para una copia de seguridad del sistema de este equipo, como Time Machine, el Historial de archivos o tu propio script. Conserva lo que la copia de seguridad desde la app deja fuera, los eventos de los Engines y el historial de despliegues, pero **no la clave del llavero**: solo se restaura con sus secretos para el mismo usuario en el mismo equipo, o en uno cuyo llavero se restauró con ella. En cualquier otro sitio, vuelven todos los nombres, direcciones y archivos compose, y hay que [volver a introducir](#si-se-pierden-los-secretos) los secretos.

### Con Homeport cerrado

Sal de Homeport, desde **Salir** en el icono de la bandeja si se [cierra a la bandeja del sistema](/docs/guides/settings/#escritorio), y copia después toda la carpeta de datos. Una vez cerrado Homeport, `homeport.db` lo contiene todo y los archivos `-wal` y `-shm` han desaparecido.

### Con Homeport en ejecución

No copies `homeport.db` solo mientras Homeport se ejecuta: los últimos cambios pueden estar todavía en `homeport.db-wal`, y una copia hecha a mitad de una escritura puede ser incoherente. Pide en su lugar a SQLite una copia coherente, con la herramienta de línea de comandos `sqlite3` (incluida en macOS; en Linux y Windows, instálala desde el paquete `sqlite3` de tu distribución o desde [sqlite.org](https://sqlite.org/download.html)):

```sh
# Linux
sqlite3 ~/.local/share/com.homeport.app/homeport.db ".backup '$HOME/homeport-backup.db'"

# macOS
sqlite3 "$HOME/Library/Application Support/com.homeport.app/homeport.db" ".backup '$HOME/homeport-backup.db'"
```

```powershell
# Windows
sqlite3 "$env:APPDATA\com.homeport.app\homeport.db" ".backup '$HOME\homeport-backup.db'"
```

El resultado es un solo archivo completo, sin ningún `-wal` al lado. Para comprobarlo:

```sh
sqlite3 ~/homeport-backup.db "PRAGMA integrity_check;"
```

Muestra `ok`.

### Restaurar una copia

1. Sal de Homeport.
2. En la carpeta de datos, aparta `homeport.db` y borra `homeport.db-wal` y `homeport.db-shm` si están: pertenecen a la base de datos que vas a sustituir.
3. Pon ahí tu copia, con el nombre `homeport.db`. Si copiaste toda la carpeta, vuelve a poner la carpeta en su lugar.
4. Abre Homeport.

Homeport actualiza una copia hecha por una versión más antigua cuando la abre. Una copia usada por última vez por un Homeport más reciente se rechaza, sin tocarla, con [un mensaje que indica la versión que hay que instalar](/docs/reference/troubleshooting/#esta-base-de-datos-la-usó-por-última-vez-homeport--o-una-versión-posterior).

## Si se pierden los secretos

Tras restablecer el llavero, con una cuenta de usuario nueva o al abrir en otro equipo una base de datos copiada, Homeport conserva todo lo que todavía puede leer y marca lo que no. No se borra nada, y lo que no es un secreto (nombres, direcciones, archivos compose, rutas de carpetas) está intacto. Vuelve a introducir:

| Qué                                     | Cómo                                                                                                                                                                        |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Hosts TLS**                           | En la página del host, vuelve a cargar el certificado de la CA, el certificado de cliente y la clave de cliente, y elige **Guardar**.                                       |
| **Hosts SSH con una clave importada**   | Vuelve a pegar la clave privada y su frase de contraseña, y elige **Guardar**.                                                                                              |
| **Hosts SSH con una clave de Homeport** | Elige **Regenerar clave** y después [autoriza la nueva clave](/docs/hosts/ssh/#autoriza-la-clave-de-homeport) en el servidor. Quita allí la antigua línea `homeport@`.      |
| **Hosts Incus por HTTPS**               | Quita el host y [vuelve a añadirlo con un nuevo token de confianza](/docs/hosts/incus/#por-https-con-un-token-de-confianza).                                                |
| **Clústeres de Kubernetes**             | **Actualizar desde el kubeconfig** en la página del clúster.                                                                                                                |
| **Clústeres de Nomad**                  | Pega el token ACL, y los certificados si los usa, en los **Ajustes** del clúster.                                                                                           |
| **Entornos de los stacks**              | Vuelve a escribir el entorno de un stack gestionado o sincronizado en su página. Los stacks vinculados guardan el suyo en el `.env` de su carpeta, que Homeport nunca tuvo. |
| **Inicios de sesión de registros**      | En **Ajustes → Registros**, edita cada inicio de sesión y escribe su contraseña o su token.                                                                                 |
| **Repositorios**                        | En **Ajustes → Repositorios**, crea una **Nueva clave** y añádela al repositorio en tu forja, o elige **Sustituir el token**.                                               |

Los hosts de socket local, TCP y ssh-agent no tienen ningún secreto y siguen funcionando. Los entornos registrados en el historial de despliegues no se pueden volver a introducir; los archivos compose del historial siguen siendo legibles.

Consulta [Credenciales perdidas](/docs/reference/troubleshooting/#credenciales-perdidas) en la solución de problemas.

## Eliminar tus datos

[Eliminar tus datos](/docs/reference/updating/#eliminar-tus-datos) lista las carpetas y la entrada del llavero que hay que borrar, y las claves que hay que quitar de tus servidores.
