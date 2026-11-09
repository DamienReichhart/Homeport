---
title: Recursos de la API
description: Explora todos los tipos de objeto que sirve un clúster, recursos personalizados incluidos, con las columnas que muestra kubectl get.
---

Homeport tiene páginas propias para las cargas de trabajo, los Pods, los Services, los Ingresses, los ConfigMaps, los Secrets, los Nodes y los Events. **Recursos de la API**, en la barra lateral de un clúster, cubre todo lo demás: los PersistentVolumes y sus claims, las NetworkPolicies, las ServiceAccounts y RBAC, y los recursos que añaden los operadores con sus CRD (los Certificates de cert-manager, las Applications de Argo CD, los ServiceMonitors de Prometheus).

## Encontrar un tipo

La columna de la izquierda lista todos los tipos que sirve el clúster, agrupados por grupo de API (`core` primero), en la versión preferida de cada grupo, como hace `kubectl api-resources`. Busca por kind, por plural o por grupo, o escribe un nombre corto: `pvc`, `netpol`, `sa`, `cert`.

Un candado marca los tipos que no puedes listar: los que tus credenciales no tienen permiso para listar en el namespace elegido (según le indica el clúster a Homeport mediante un `SelfSubjectRulesReview`) y los pocos cuya API no permite listar en absoluto. Al abrir uno se explica por qué. Algunos clústeres, como aquellos cuyo autorizador es un webhook, no pueden indicarlo; entonces no se marca nada y una lista rechazada lo dice.

Instalar o eliminar una CRD actualiza la lista por sí sola; **Actualizar** la vuelve a leer.

## Listar objetos

Elige un tipo y sus objetos se listan con las mismas columnas que `kubectl get`: el servidor de la API indica cuáles, incluidas las `additionalPrinterColumns` de una CRD, así que los Certificates de cert-manager muestran **Ready**, **Secret** y **Age** como en un terminal. La lista sigue los cambios en directo, mediante un watch.

Los tipos con namespace tienen el selector de namespace, como las demás páginas de Homeport; con **Todos los namespaces** se añade una columna **Namespace**.

## La página de un objeto

Haz clic en un objeto para abrir su página:

- **Resumen**: su namespace, los objetos de los que es propiedad (con un enlace a su página), su antigüedad, sus etiquetas y sus anotaciones.
- **Condiciones**, cuando su `status` las tiene, como en las páginas propias de Homeport.
- Los **Eventos** sobre él, y su **YAML**, sin `managedFields`.

Cuando su tipo lo permite, **Editar** abre su YAML para cambiarlo mediante una previsualización y server-side apply (consulta [Aplicar y editar YAML](/docs/clusters/yaml/)), y **Eliminar** lo elimina tras una confirmación que indica su nombre y el del clúster (consulta [Eliminar](/docs/clusters/actions/#eliminar)). En la lista, marca varios objetos para eliminarlos a la vez.
