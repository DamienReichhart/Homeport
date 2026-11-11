---
title: RBAC
description: Veja os Roles, bindings e sujeitos de um cluster, o que cada sujeito pode fazer e onde, e o que suas próprias credenciais podem fazer.
---

**RBAC**, na barra lateral de um cluster Kubernetes, mostra quem pode fazer o quê no cluster. É somente leitura: o Homeport não cria, altera nem exclui objetos RBAC (você ainda pode editá-los como YAML em [Recursos da API](/docs/clusters/api-resources/)).

Como as outras páginas, ela segue o seletor de namespace: com um namespace escolhido, mostra os Roles e RoleBindings desse namespace, além dos ClusterRoles e ClusterRoleBindings, que valem em todo lugar.

## Roles

**Roles** lista os Roles (cada um em seu namespace) e os ClusterRoles, com o número de regras. Abra um para ver sua página:

- suas **regras**, como o Kubernetes as escreve: recursos, grupos de API (`core` para o grupo core), nomes de recursos e verbos, ou URLs que não são recursos, como `/healthz`;
- os **bindings que a usam**, cada um com links para seus sujeitos.

Um ClusterRole marcado como **agregado** tem uma `aggregationRule`: sua página mostra seus seletores de rótulos, os ClusterRoles que eles selecionam, e suas regras incluem as deles. Os `admin`, `edit` e `view` do próprio Kubernetes são construídos assim, então o ClusterRole de uma CRD com o rótulo `rbac.authorization.k8s.io/aggregate-to-edit: "true"` aparece em `edit`.

## Bindings

**Bindings** lista os RoleBindings e ClusterRoleBindings, cada um com a role que concede, com link, e os sujeitos que nomeia: Users, Groups e ServiceAccounts. Um binding cuja role não existe é marcado como **não encontrado**; ele não concede nada.

## Sujeitos

**Sujeitos** lista todos que algum binding nomeia. Abra um User, um Group ou um ServiceAccount para ver o que ele pode fazer, como uma matriz de recursos e verbos agrupada por onde vale: primeiro **Todo o cluster**, depois cada namespace. Abaixo, os bindings que concedem isso, e como: nomeando-o diretamente ou por um grupo.

O Homeport combina as regras de cada binding que se aplica:

- Um ClusterRoleBinding concede seu ClusterRole em todo lugar: em cada namespace, nos recursos de escopo de cluster como os Nodes e nas URLs que não são recursos.
- Um RoleBinding concede sua role apenas no próprio namespace, mesmo quando vincula um ClusterRole. Um ServiceAccount vinculado a `edit` por um RoleBinding em `prod` pode editar em `prod` e em nenhum outro lugar.
- Também contam os bindings para os grupos em que o Kubernetes coloca cada sujeito: um ServiceAccount está em `system:serviceaccounts`, `system:serviceaccounts:<seu namespace>` e `system:authenticated`, e também é o usuário `system:serviceaccount:<namespace>:<nome>`; um User está em `system:authenticated`.
- `*` representa qualquer verbo, grupo de API ou recurso, `*/scale` esse sub-recurso de qualquer recurso, e uma regra com nomes de recursos vale só para esses objetos (a matriz os mostra em **Somente**). Uma linha já coberta por outra mais ampla é omitida.

**É uma visão aproximada.** As permissões de um sujeito são calculadas a partir dos bindings que suas credenciais conseguem ler, não perguntadas ao servidor da API, então não exigem permissão para personificar ninguém. Elas podem não ver o que outros autorizadores permitem, como um webhook (comum em clusters gerenciados) ou ABAC, nem os grupos que um autenticador dá a um usuário (de claims OIDC ou da organização de um certificado de cliente), que só o servidor da API conhece. Para ter certeza sobre suas próprias credenciais, use **O que posso fazer aqui?**.

Com um namespace escolhido, só os RoleBindings desse namespace são lidos; escolha **Todos os namespaces** para ver as permissões de um sujeito em todo lugar.

## O que posso fazer aqui?

**O que posso fazer aqui?** é a resposta do próprio servidor da API para as credenciais que o Homeport usa, no namespace escolhido (o padrão do cluster com **Todos os namespaces**), como `kubectl auth can-i --list` a mostra. Ela diz com qual identidade elas entram e em quais grupos, nos clusters que informam isso (Kubernetes 1.28 e posteriores), com um link para a página desse sujeito.

Quando o servidor da API diz que sua lista pode estar incompleta — um autorizador como um webhook pode permitir mais do que consegue listar —, a página avisa.

**Posso…?** faz uma pergunta, como `kubectl auth can-i`: um verbo, um recurso e um namespace (vazio para todos os namespaces) e, opcionalmente, um grupo de API, um sub-recurso e o nome de um objeto. O recurso pode ser seu plural (`secrets`), um nome curto (`deploy`) ou seu plural com o grupo (`deployments.apps`); o Homeport o resolve como o kubectl. A resposta vem com a mesma pergunta como comando para copiar, como `kubectl auth can-i delete secrets --namespace prod`, que dá a mesma resposta.

Quando suas credenciais não conseguem listar nenhum objeto RBAC, a página mostra só **O que posso fazer aqui?**, abaixo de uma mensagem que diz o que foi recusado. Quando só alguns podem ser listados, como os RoleBindings mas não os ClusterRoles, a página diz quais, e o que ela mostra os deixa de fora.
