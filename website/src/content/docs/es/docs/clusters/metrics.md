---
title: CPU y memoria
description: Mira cuánta CPU y memoria usan los Pods, sus contenedores y los Nodes, frente a sus solicitudes, límites y lo asignable, gracias a metrics-server.
---

Cuando un clúster ejecuta [metrics-server](https://github.com/kubernetes-sigs/metrics-server), Homeport muestra cuánta CPU y memoria usa cada Pod y cada Node: en las listas y en gráficos en directo en sus páginas. Lee la Metrics API (`metrics.k8s.io`) que ofrece metrics-server, como hace `kubectl top`.

## Lo que necesitas

metrics-server ya tiene que estar en ejecución en el clúster: Homeport no instala nada en un clúster. Que esté o no depende de cómo se creó el clúster:

| Clúster                    | metrics-server                                                         |
| -------------------------- | ---------------------------------------------------------------------- |
| GKE                        | Instalado                                                              |
| AKS                        | Instalado                                                              |
| k3s y k3d                  | Instalado (salvo que k3s se iniciara con `--disable metrics-server`)   |
| Docker Desktop             | Instalado con su Kubernetes                                            |
| kind                       | No instalado                                                           |
| EKS                        | No instalado: añade el complemento metrics-server o instálalo tú mismo |
| minikube                   | No instalado: `minikube addons enable metrics-server`                  |
| Otros clústeres (kubeadm…) | Normalmente no instalado                                               |

Para instalarlo tú mismo, sigue [sus instrucciones](https://github.com/kubernetes-sigs/metrics-server#installation); en un clúster local cuyos kubelets tienen certificados autofirmados (kind), metrics-server también necesita `--kubelet-insecure-tls`.

Sin metrics-server no se rompe nada: las columnas de CPU y memoria no se muestran, y la pestaña **Métricas** de un Pod o de un Node dice que metrics-server no está instalado, con un enlace a él. Justo después de instalarlo, metrics-server necesita alrededor de un minuto para tener muestras; mientras tanto, la pestaña dice que la Metrics API aún no responde.

Tus credenciales también tienen que poder leer métricas: `list` y `get` sobre `pods.metrics.k8s.io` en el Namespace (o en todos), y sobre `nodes.metrics.k8s.io` para los Nodes. Si no pueden, la pestaña **Métricas** dice qué permiso falta, y todas las demás páginas siguen funcionando.

## En las listas

En **Pods**, las columnas **CPU** y **Memoria** muestran lo que usa cada Pod, sumando sus contenedores, y qué parte es de su límite, o de su solicitud si no tiene límite. Un Pod solo tiene límite cuando todos sus contenedores lo tienen: un contenedor sin límite puede usar todo el Node. La CPU va en milinúcleos, como la escribe `kubectl top` (`250m` es un cuarto de núcleo), y la memoria es el working set, por el que el kubelet desaloja Pods.

En **Nodes**, **CPU usada** y **Memoria usada** muestran lo que usa cada Node y su parte de lo que es **asignable** a los Pods.

Haz clic en el encabezado de una columna para ordenar por ella: primero los Pods o Nodes más cargados.

## En la página de un Pod o de un Node

La pestaña **Métricas** de un Pod traza la CPU y la memoria de cada contenedor, con una línea discontinua en su solicitud y otra en su límite, cuando están definidos. La de un Node traza su CPU y su memoria con una línea en lo asignable.

metrics-server lee los kubelets cada 15 segundos, y Homeport pide una muestra nueva al mismo ritmo, así que un cambio de carga aparece en el gráfico en menos de 30 segundos. Las muestras se guardan mientras la página está abierta, 15 minutos como máximo, sea cual sea la pestaña, y solo en memoria: cerrar la página detiene las lecturas y las olvida. Homeport no guarda ningún historial de métricas; para eso, usa un sistema de monitorización como Prometheus.
