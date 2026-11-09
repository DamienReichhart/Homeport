---
title: Escalar, reiniciar y eliminar
description: Escala Deployments y StatefulSets, reinicia un despliegue, suspende y ejecuta CronJobs, y elimina cualquier objeto de un clúster, cada vez tras una confirmación que lo nombra.
---

Los cambios de todos los días en un clúster están a un clic en sus páginas, y cada uno se envía a la API de Kubernetes con las propias credenciales del clúster, como haría `kubectl`. Las listas siguen el cambio en directo mediante su watch.

Toda acción que elimina o sustituye algo pregunta antes, y la confirmación indica el clúster, el namespace, el kind y el nombre, para que un cambio nunca acabe por accidente en el clúster o el namespace equivocado.

## Cargas de trabajo

La página de una carga de trabajo tiene sus acciones en la cabecera; en **Cargas de trabajo**, cada fila las tiene en su menú **⋯**.

| Acción             | Sobre                                 | Hace, como                                                              |
| ------------------ | ------------------------------------- | ----------------------------------------------------------------------- |
| **Escalar**        | Deployments, StatefulSets             | `kubectl scale --replicas=N`                                            |
| **Reiniciar**      | Deployments, StatefulSets, DaemonSets | `kubectl rollout restart`                                               |
| **Suspender**      | CronJobs                              | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`                |
| **Reanudar**       | CronJobs suspendidos                  | lo mismo, con `false`                                                   |
| **Ejecutar ahora** | CronJobs                              | `kubectl create job --from=cronjob/…`                                   |
| **Eliminar**       | Todos ellos                           | `kubectl delete` (sus Pods y Jobs se eliminan con él, en segundo plano) |

### Escalar

**Escalar** pregunta cuántos Pods ejecutar y muestra cuántos ejecuta ahora. Escalar a `0` detiene la carga de trabajo hasta que vuelvas a escalarla. Si un HorizontalPodAutoscaler gestiona la carga de trabajo, el diálogo lo indica: el autoescalador restablecerá el número en su próxima comprobación, así que cambia en su lugar el mínimo y el máximo del autoescalador.

### Reiniciar

**Reiniciar** añade la anotación `kubectl.kubernetes.io/restartedAt` a la plantilla de Pod, exactamente como hace `kubectl rollout restart`: la carga de trabajo despliega un nuevo conjunto de Pods según su estrategia (un nuevo ReplicaSet para un Deployment), y los Pods antiguos desaparecen cuando los nuevos están listos.

### CronJobs

**Suspender** impide que un CronJob inicie Jobs nuevos (los que ya se están ejecutando continúan) y **Reanudar** le permite volver a iniciarlos; un CronJob suspendido se muestra como **En pausa**. **Ejecutar ahora** inicia al instante un Job a partir de su plantilla, llamado `<cronjob>-manual-<id>`, propiedad del CronJob y marcado con `cronjob.kubernetes.io/instantiate: manual` como los de kubectl. Aparece en la pestaña **Jobs** del CronJob.

## Eliminar

La página de cualquier objeto (una carga de trabajo, un Pod, un Service, un Ingress, un ConfigMap, un Secret, un Node o cualquier objeto de **Recursos de la API**) tiene **Eliminar** en su cabecera. En una lista, marca filas y **Eliminar N** las elimina todas tras una única confirmación que las enumera.

Los objetos se eliminan con la propagación `Background`, como hace `kubectl delete`: el objeto desaparece al instante y el recolector de basura elimina lo que es de su propiedad (los ReplicaSets y Pods de un Deployment, los Jobs de un CronJob).

La confirmación dice lo que pasará después:

- **Un Pod que es propiedad de un controlador** (un ReplicaSet, StatefulSet, DaemonSet o Job) vuelve: el controlador inicia uno nuevo en su lugar. Elimina o escala la carga de trabajo para deshacerte de él.
- **Un Namespace** se lleva todo lo que contiene: sus cargas de trabajo, Pods, Services, ConfigMaps y Secrets.
- **Un Node** se quita de la lista del clúster sin drenarlo antes.

## Cuando no funciona

- **No permitido**: un cambio que tus credenciales no pueden hacer lo indica con su verbo y su recurso («Tus credenciales no permiten «patch» sobre deployments en shop.») y con el comando `kubectl auth can-i` para comprobarlo.
- **Modificado mientras tanto**: cuando otra persona cambió el objeto entre que lo miraste y tu acción, Homeport lo indica; vuelve a mirarlo y reinténtalo.
- **Ya no existe**: eliminar algo que ya se eliminó indica que ya no existe.
