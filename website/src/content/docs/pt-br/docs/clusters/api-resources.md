---
title: Recursos da API
description: Navegue por todos os tipos de objeto que um cluster serve, recursos personalizados incluídos, com as colunas que o kubectl get mostra.
---

O Homeport tem páginas próprias para cargas de trabalho, Pods, Services, Ingresses, ConfigMaps, Secrets, Nodes e Events. **Recursos da API**, na barra lateral de um cluster, cobre todo o resto: PersistentVolumes e as claims deles, NetworkPolicies, ServiceAccounts e RBAC, e os recursos que os operators adicionam com os CRDs deles — os Certificates do cert-manager, as Applications do Argo CD, os ServiceMonitors do Prometheus.

## Encontrar um tipo

A coluna da esquerda lista todos os tipos que o cluster serve, agrupados por grupo de API (`core` primeiro), na versão preferida de cada grupo, como o `kubectl api-resources` faz. Pesquise por kind, plural ou grupo, ou digite um nome curto: `pvc`, `netpol`, `sa`, `cert`.

Um cadeado marca os tipos que você não pode listar: aqueles que as suas credenciais não têm permissão para listar no namespace escolhido (como o cluster informa ao Homeport por um `SelfSubjectRulesReview`), e os poucos cuja API não permite listar de forma alguma. Abrir um deles diz por quê. Alguns clusters, como aqueles cujo autorizador é um webhook, não sabem dizer; então nada é marcado e uma lista recusada avisa.

Instalar ou remover um CRD atualiza a lista sozinho; **Atualizar** a lê de novo.

## Listar objetos

Escolha um tipo e os objetos dele são listados com as mesmas colunas que o `kubectl get`: o servidor de API diz quais, incluindo as `additionalPrinterColumns` de um CRD, então os Certificates do cert-manager mostram **Ready**, **Secret** e **Age** como mostram em um terminal. A lista acompanha as alterações em tempo real, por um watch.

Os tipos com namespace têm o seletor de namespace, como as outras páginas do Homeport; com **Todos os namespaces**, uma coluna **Namespace** é adicionada.

## A página de um objeto

Clique em um objeto para abrir a página dele:

- **Visão geral**: o namespace, os objetos aos quais ele pertence — com link para a página deles —, a idade, os rótulos e as anotações.
- **Condições**, quando o `status` dele as tem, como nas páginas próprias do Homeport.
- Os **Eventos** sobre ele e o **YAML** dele, sem `managedFields`.

Quando o tipo permite, **Editar** abre o YAML para alterá-lo com uma pré-visualização e server-side apply (veja [Aplicar e editar YAML](/docs/clusters/yaml/)), e **Excluir** o remove depois de uma confirmação que indica o objeto e o cluster (veja [Excluir](/docs/clusters/actions/#excluir)). Na lista, marque vários objetos para excluí-los juntos.
