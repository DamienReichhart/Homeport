---
title: Logs y terminales
description: Sigue los logs de un Pod o de toda una carga de trabajo, averigua por qué falló un contenedor y abre un terminal en un contenedor de un Pod.
---

La página de un Pod tiene las pestañas **Logs** y **Terminal**, y la página de una carga de trabajo, una pestaña **Logs** para todos sus Pods: el `kubectl logs` y el `kubectl exec -it` de todos los días, a través de la API de Kubernetes con las propias credenciales del clúster.

## Logs

**Logs** transmite los logs de un contenedor a medida que se escriben. Es el mismo visor que el de un contenedor de un host:

- **Historial**: empieza por las últimas 100, 1000 o 10 000 líneas, o por todas.
- **Desde**: solo las líneas de los últimos minutos, horas o días.
- **Marcas de tiempo**: la hora a la que Kubernetes registró cada línea.
- **Seguir**: mantiene a la vista la línea más reciente; desactívalo para leer hacia atrás mientras siguen llegando líneas nuevas.
- **Buscar en los logs**, **Borrar** y **Descargar logs** (como archivo de texto, las líneas mostradas hasta el momento).

Un Pod con varios contenedores tiene un selector de **Contenedor**, con los contenedores init marcados. Se abre en el contenedor que elegiría `kubectl`: el que indica la anotación `kubectl.kubernetes.io/default-container` del Pod o, si no, su primer contenedor de aplicación.

Kubernetes guarda juntas la salida estándar y la salida de error de un contenedor, así que no hay filtro stdout/stderr como en los contenedores de un host.

La transmisión se detiene cuando sales de la pestaña, y lo indica cuando el contenedor se detiene.

### ¿Por qué falló?

Cuando un contenedor se reinició, **Instancia anterior** muestra los logs de la ejecución anterior al último reinicio (para un Pod en `CrashLoopBackOff`, la ejecución que falló), como `kubectl logs --previous`. Está desactivado para un contenedor que nunca se reinició, ya que no hay nada que mostrar.

Cuando un contenedor aún no se ha iniciado (`ContainerCreating`, `CrashLoopBackOff` mientras espera para volver a iniciarse), los logs lo indican con el motivo, en su lugar. La página de su Pod dice por qué espera.

### Los logs de una carga de trabajo

La pestaña **Logs** de un Deployment, StatefulSet, DaemonSet o Job intercala los logs de todos sus Pods a medida que se escriben, con cada línea etiquetada con su Pod en su propio color, como `kubectl logs -l app=api --prefix`. Elige un Pod para leerlo solo, y el contenedor cuando su plantilla tiene varios.

## Terminal

**Terminal** abre un shell en uno de los contenedores en ejecución del Pod: `bash` cuando la imagen lo tiene, `sh` si no. Escribe en su lugar un comando en el campo (`psql -U app shop`, `top`) y **Conectar** lo ejecuta, entrecomillado como lo haría un shell.

Es un TTY real: `vi`, `top`, los colores y las teclas de flecha funcionan, y redimensionar la ventana redimensiona el terminal en el contenedor. Cuando el proceso termina, el terminal indica con qué código; **Volver a conectar** inicia otro. Salir de la pestaña termina el proceso en el contenedor.

### Cuando no puede abrirse

- **Sin shell**: las imágenes mínimas (distroless) no tienen ningún shell. Escribe un comando que tenga la imagen, o adjunta un contenedor de depuración desde un terminal con `kubectl debug -it <pod> --image=busybox --target=<container>`; Homeport no crea contenedores de depuración.
- **No se está ejecutando**: un contenedor que espera o que terminó no puede ejecutar nada. Los Pods de un Job completado se han detenido para siempre.
- **No permitido**: abrir un terminal requiere `create` sobre `pods/exec`, y leer los logs, `get` sobre `pods/log`, algo de lo que suelen carecer los roles de solo lectura. Homeport indica cuál, con el comando `kubectl auth can-i` para comprobarlo.

Homeport no copia archivos hacia dentro ni hacia fuera de un Pod (`kubectl cp`).
