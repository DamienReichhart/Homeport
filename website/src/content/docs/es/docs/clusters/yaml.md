---
title: Aplicar y editar YAML
description: Aplica manifiestos a un clúster o edita el YAML de un objeto, viendo lo que cambia antes de que cambie nada, con server-side apply.
---

Dos formas de cambiar un clúster con YAML, para todo lo que no cubren las acciones dedicadas: **Aplicar YAML** para manifiestos, como `kubectl apply -f`, y **Editar** en la pestaña **YAML** de cualquier objeto, como `kubectl edit`. Las dos muestran lo que cambiaría antes de que cambie nada.

## Aplicar YAML

**Aplicar YAML**, en la barra lateral de un clúster, acepta uno o varios manifiestos: pégalos, o usa **Abrir archivo…** para cargar un archivo `.yaml`, `.yml` o `.json` de este equipo. Varios documentos separados por `---` se aplican juntos, en orden.

Un documento sin namespace va al namespace elegido arriba a la derecha (o, con **Todos los namespaces**, al namespace predeterminado del clúster). Los objetos que no están en un namespace, como un Namespace o un ClusterRole, lo ignoran.

1. **Previsualizar cambios** pregunta al clúster qué haría cada documento, con un dry run del lado del servidor: aún no cambia nada. Cada documento aparece como **Creado**, **Modificado** o **Sin cambios**, con las líneas que cambiaría.
2. **Aplicar** te pide que confirmes, indicando el clúster, y luego aplica.

Aplicar los mismos manifiestos dos veces informa de **Sin cambios** la segunda vez. Editar el YAML después de una previsualización pide primero una nueva.

## Editar un objeto

La pestaña **YAML** de cada objeto tiene **Editar**: su YAML tal como lo tiene el clúster, sin lo que gestiona el servidor (`managedFields`, `status`, su `resourceVersion`, su uid y su fecha de creación). Cámbialo, usa **Previsualizar cambios** para ver solo las líneas que cambiaste y luego **Aplicar**.

La previsualización compara tu YAML con el objeto tal como está en el momento de previsualizar, así que un cambio hecho mientras tanto aparece en el diff. Si la edición cambia un campo que gestiona otra herramienta, la previsualización lo muestra como un conflicto en lugar de sobrescribirlo.

Los Secrets no se pueden editar en Homeport, ya que nunca muestra todos sus valores a la vez; y los Events los escribe el propio Kubernetes.

## Server-side apply

Homeport aplica con el server-side apply de Kubernetes, con el field manager `homeport`: el clúster registra qué campos estableció Homeport, junto a los establecidos por `kubectl`, Helm, los controladores u otras herramientas, que verás en los `managedFields` de un objeto.

Cuando un documento cambia un campo que gestiona otra herramienta (`.spec.replicas` establecido por `kubectl apply` o gestionado por un autoescalador), la previsualización indica qué campos y qué managers. Deja fuera esos campos, o usa **Aplicar de todos modos** tras una segunda confirmación: Homeport toma el control de ellos y la otra herramienta puede volver a cambiarlos.

## Cuando un documento no se puede aplicar

La previsualización muestra el problema de cada documento en su lugar, y **Aplicar** sigue desactivado hasta que todos los documentos se pueden aplicar:

- **YAML no válido**, con la línea en la que falla.
- **Kind desconocido**: el clúster no sirve ese `apiVersion` y ese `kind`. Un recurso personalizado necesita antes que su CRD esté instalada.
- **Rechazado por el clúster**: la respuesta de su validación para cada campo, como un campo obligatorio que falta o un valor del tipo equivocado.
- **No permitido**: aplicar requiere `patch` sobre el recurso (y `create` para los objetos nuevos), con el comando `kubectl auth can-i` para comprobarlo.
