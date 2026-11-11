---
title: RBAC
description: Mira los Roles, bindings y sujetos de un clúster, qué puede hacer cada sujeto y dónde, y qué pueden hacer tus propias credenciales.
---

**RBAC**, en la barra lateral de un clúster de Kubernetes, muestra quién puede hacer qué en el clúster. Es de solo lectura: Homeport no crea, cambia ni elimina objetos RBAC (aún puedes editarlos como YAML desde [Recursos de la API](/docs/clusters/api-resources/)).

Como las demás páginas, sigue el selector de namespace: con un namespace elegido, muestra los Roles y RoleBindings de ese namespace, además de los ClusterRoles y ClusterRoleBindings, que se aplican en todas partes.

## Roles

**Roles** lista los Roles (cada uno en su namespace) y los ClusterRoles, con su número de reglas. Abre uno para ver su página:

- sus **reglas**, tal como las escribe Kubernetes: recursos, grupos de API (`core` para el grupo core), nombres de recursos y verbos, o URL que no son recursos, como `/healthz`;
- los **bindings que lo usan**, cada uno enlazado a sus sujetos.

Un ClusterRole marcado como **agregado** tiene una `aggregationRule`: su página muestra sus selectores de etiquetas, los ClusterRoles que coinciden con ellos, y sus reglas incluyen las de esos. Los `admin`, `edit` y `view` propios de Kubernetes se construyen así, de modo que el ClusterRole de una CRD con la etiqueta `rbac.authorization.k8s.io/aggregate-to-edit: "true"` aparece en `edit`.

## Bindings

**Bindings** lista los RoleBindings y ClusterRoleBindings, cada uno con el rol que concede, enlazado, y los sujetos que nombra: Users, Groups y ServiceAccounts. Un binding cuyo rol no existe se marca como **no encontrado**; no concede nada.

## Sujetos

**Sujetos** lista a todos los que nombra algún binding. Abre un User, un Group o un ServiceAccount para ver qué puede hacer, como una matriz de recursos y verbos agrupada por dónde se aplica: primero **Todo el clúster**, luego cada namespace. Debajo, los bindings que se lo conceden, y cómo: nombrándolo directamente o a través de un grupo.

Homeport combina las reglas de cada binding que se aplica:

- Un ClusterRoleBinding concede su ClusterRole en todas partes: en cada namespace, en los recursos de alcance de clúster como los Nodes y en las URL que no son recursos.
- Un RoleBinding concede su rol solo en su propio namespace, incluso cuando vincula un ClusterRole. Un ServiceAccount vinculado a `edit` por un RoleBinding en `prod` puede editar en `prod` y en ningún otro sitio.
- También cuentan los bindings a los grupos en los que Kubernetes pone a cada sujeto: un ServiceAccount está en `system:serviceaccounts`, `system:serviceaccounts:<su namespace>` y `system:authenticated`, y además es el usuario `system:serviceaccount:<namespace>:<nombre>`; un User está en `system:authenticated`.
- `*` representa cualquier verbo, grupo de API o recurso, `*/scale` ese subrecurso de cualquier recurso, y una regla con nombres de recursos solo se aplica a esos objetos (la matriz los muestra en **Solo**). Se omite una fila que otra más amplia ya cubre.

**Es una vista aproximada.** Los permisos de un sujeto se calculan a partir de los bindings que tus credenciales pueden leer, no se preguntan al servidor de la API, así que no necesitan permiso para suplantar a nadie. Pueden no ver lo que permiten otros autorizadores, como un webhook (habitual en clústeres gestionados) o ABAC, ni los grupos que un autenticador da a un usuario (de los claims de OIDC o de la organización de un certificado de cliente), que solo conoce el servidor de la API. Para estar seguro de tus propias credenciales, usa **¿Qué puedo hacer aquí?**.

Con un namespace elegido, solo se leen los RoleBindings de ese namespace; elige **Todos los namespaces** para ver los permisos de un sujeto en todas partes.

## ¿Qué puedo hacer aquí?

**¿Qué puedo hacer aquí?** es la respuesta del propio servidor de la API para las credenciales que usa Homeport, en el namespace elegido (el predeterminado del clúster con **Todos los namespaces**), como la muestra `kubectl auth can-i --list`. Indica con qué identidad inician sesión y en qué grupos, en los clústeres que lo dicen (Kubernetes 1.28 y posteriores), con un enlace a la página de ese sujeto.

Cuando el servidor de la API indica que su lista puede estar incompleta —un autorizador como un webhook puede permitir más de lo que sabe listar—, la página lo dice.

**¿Puedo…?** hace una pregunta, como `kubectl auth can-i`: un verbo, un recurso y un namespace (vacío para todos los namespaces) y, opcionalmente, un grupo de API, un subrecurso y el nombre de un objeto. El recurso puede ser su plural (`secrets`), un nombre corto (`deploy`) o su plural con su grupo (`deployments.apps`); Homeport lo resuelve como kubectl. La respuesta viene con la misma pregunta como comando para copiar, como `kubectl auth can-i delete secrets --namespace prod`, que da la misma respuesta.

Cuando tus credenciales no pueden listar ningún objeto RBAC, la página muestra solo **¿Qué puedo hacer aquí?**, bajo un mensaje que nombra lo que se rechazó. Cuando solo se pueden listar algunos, como los RoleBindings pero no los ClusterRoles, la página dice cuáles, y lo que muestra los deja fuera.
