---
title: Reenvío de puertos
description: Alcanza un puerto de un Pod o de un Service desde este equipo, como kubectl port-forward, sin exponerlo.
---

Un reenvío de puerto hace que un puerto de un Pod, o de un Service, sea accesible en este equipo: una base de datos, una interfaz de administración o una API interna, sin un `LoadBalancer`, un Ingress ni un terminal abierto con `kubectl port-forward`.

## Reenviar un puerto

- En la página de un **Pod**, los contenedores listan los puertos que declaran; **Reenviar…** está junto a cada puerto TCP.
- En la página de un **Service**, **Reenviar…** está junto a cada uno de sus puertos TCP.

Elige el puerto local: empieza con el mismo número cuando ese puerto está libre en este equipo, y vacío si no. Déjalo vacío y Homeport elige uno libre. Un aviso indica dónde escucha, con **Abrir** para cargarlo en tu navegador.

El reenvío solo escucha en las direcciones de loopback (`127.0.0.1`, y `[::1]` cuando el mismo puerto está libre ahí), así que nada más de tu red puede alcanzarlo.

### Pods y Services

Un reenvío a un **Pod** alcanza ese Pod. Si el Pod desaparece (un despliegue lo sustituyó), el reenvío termina y Homeport lo indica.

Un reenvío a un **Service** pasa por uno de sus Pods listos, como hace `kubectl port-forward svc/…`, al puerto al que el Service envía ese puerto (su `targetPort`, por número o por el nombre de un puerto de contenedor). Cuando ese Pod desaparece, la siguiente conexión elige otro que esté listo, así que un reenvío a un Service sobrevive a los despliegues. No reparte las conexiones entre los Pods.

Solo se puede reenviar TCP. Los Services sin selector (que apuntan a endpoints externos) no se pueden reenviar.

## Reenvíos de puerto

**Reenvíos de puerto**, en la barra lateral de un clúster, lista todos los reenvíos activos, de todos tus clústeres: su dirección local, el Pod o el Service y el puerto que alcanza (y, para un Service, el Pod por el que pasa ahora), su Namespace y su clúster, y cuántas conexiones hay abiertas a través de él. **Abrir** lo carga en tu navegador; **Detener** lo termina y cierra sus conexiones.

Los reenvíos solo duran lo que dura Homeport: al salir de Homeport se detienen todos, y ninguno vuelve en el siguiente inicio. Quitar o desactivar un clúster también detiene sus reenvíos.

## Cuando no funciona

- **Puerto en uso**: otro programa ya escucha en ese puerto local. Elige otro o déjalo vacío.
- **Puertos por debajo de 1024**: la mayoría de los sistemas no permiten que una aplicación escuche en ellos; elige un puerto local más alto.
- **No se está ejecutando**, **ningún Pod listo**: aún no hay nada a lo que reenviar; la página del Pod dice por qué.
- **No permitido**: reenviar requiere `create` sobre `pods/portforward`; Homeport lo indica, con el comando `kubectl auth can-i` para comprobarlo.
