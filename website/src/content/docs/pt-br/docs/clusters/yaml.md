---
title: Aplicar e editar YAML
description: Aplique manifestos a um cluster ou edite o YAML de um objeto, vendo o que muda antes de qualquer alteração, com server-side apply.
---

Duas formas de alterar um cluster com YAML, para tudo o que as ações dedicadas não cobrem: **Aplicar YAML** para manifestos, como o `kubectl apply -f`, e **Editar** na aba **YAML** de qualquer objeto, como o `kubectl edit`. Ambas mostram o que mudaria antes de qualquer alteração.

## Aplicar YAML

**Aplicar YAML**, na barra lateral de um cluster, recebe um ou mais manifestos: cole-os, ou use **Abrir arquivo…** para carregar um arquivo `.yaml`, `.yml` ou `.json` deste computador. Vários documentos separados por `---` são aplicados juntos, em ordem.

Um documento sem namespace vai para o namespace escolhido no canto superior direito (ou, com **Todos os namespaces**, para o namespace padrão do cluster). Os objetos que não ficam em um namespace, como um Namespace ou um ClusterRole, o ignoram.

1. **Pré-visualizar alterações** pergunta ao cluster o que cada documento faria, com um dry run no servidor: nada muda ainda. Cada documento aparece como **Criado**, **Alterado** ou **Sem alterações**, com as linhas que ele mudaria.
2. **Aplicar** pede a sua confirmação, indicando o cluster, e então aplica.

Aplicar os mesmos manifestos duas vezes informa **Sem alterações** na segunda vez. Editar o YAML depois de uma pré-visualização exige uma nova antes.

## Editar um objeto

A aba **YAML** de todo objeto tem **Editar**: o YAML dele como o cluster o guarda, sem o que o servidor gerencia (`managedFields`, `status`, o `resourceVersion`, o uid e a data de criação). Altere-o, use **Pré-visualizar alterações** para ver apenas as linhas que você mudou, e depois **Aplicar**.

A pré-visualização compara o seu YAML com o objeto como ele está no momento da pré-visualização, então uma alteração feita nesse meio-tempo aparece no diff. Se a edição muda um campo que outra ferramenta gerencia, a pré-visualização o lista como um conflito em vez de sobrescrevê-lo.

Os Secrets não podem ser editados no Homeport, já que ele nunca mostra todos os valores deles de uma vez; e os Events são gravados pelo próprio Kubernetes.

## Server-side apply

O Homeport aplica com o server-side apply do Kubernetes, com o field manager `homeport`: o cluster registra quais campos o Homeport definiu, ao lado dos definidos pelo `kubectl`, pelo Helm, por controladores ou por outras ferramentas, o que você vai ver nos `managedFields` de um objeto.

Quando um documento muda um campo que outra ferramenta gerencia — `.spec.replicas` definido pelo `kubectl apply` ou gerenciado por um autoscaler —, a pré-visualização diz quais campos e quais managers. Deixe esses campos de fora, ou use **Aplicar mesmo assim** depois de uma segunda confirmação: o Homeport assume esses campos, e a outra ferramenta pode redefini-los.

## Quando um documento não pode ser aplicado

A pré-visualização mostra o problema de cada documento no lugar, e **Aplicar** fica desativado até que todos os documentos possam ser aplicados:

- **YAML inválido**, com a linha em que ele quebra.
- **Kind desconhecido**: o cluster não serve esse `apiVersion` e `kind`. Um recurso personalizado precisa que o CRD dele esteja instalado antes.
- **Recusado pelo cluster**: a resposta da validação dele para cada campo, como um campo obrigatório ausente ou um valor do tipo errado.
- **Não permitido**: aplicar exige `patch` no recurso (e `create` para novos objetos), com o comando `kubectl auth can-i` para conferir.
