---
title: Escalar, reiniciar e excluir
description: Escale Deployments e StatefulSets, reinicie um rollout, suspenda e execute CronJobs e exclua qualquer objeto de um cluster, sempre depois de uma confirmação que o indica.
---

As alterações do dia a dia em um cluster estão a um clique nas páginas dele, cada uma enviada à API do Kubernetes com as próprias credenciais do cluster, como o `kubectl` faria. As listas acompanham a alteração em tempo real pelo watch delas.

Toda ação que remove ou substitui algo pede confirmação antes, e a confirmação indica o cluster, o namespace, o kind e o nome, para que uma alteração nunca caia no cluster ou no namespace errado por acidente.

## Cargas de trabalho

A página de uma carga de trabalho tem as ações no cabeçalho; em **Cargas de trabalho**, cada linha as tem no menu **⋯**.

| Ação               | Em                                    | Faz, como                                                          |
| ------------------ | ------------------------------------- | ------------------------------------------------------------------ |
| **Escalar**        | Deployments, StatefulSets             | `kubectl scale --replicas=N`                                       |
| **Reiniciar**      | Deployments, StatefulSets, DaemonSets | `kubectl rollout restart`                                          |
| **Suspender**      | CronJobs                              | `kubectl patch cronjob … -p '{"spec":{"suspend":true}}'`           |
| **Retomar**        | CronJobs suspensos                    | o mesmo, com `false`                                               |
| **Executar agora** | CronJobs                              | `kubectl create job --from=cronjob/…`                              |
| **Excluir**        | Todos eles                            | `kubectl delete` (os Pods e Jobs dele vão junto, em segundo plano) |

### Escalar

**Escalar** pergunta quantos Pods executar, mostrando quantos ele executa agora. Escalar para `0` para a carga de trabalho até que você a escale de novo. Se um HorizontalPodAutoscaler gerencia a carga de trabalho, a caixa de diálogo avisa: o autoscaler vai redefinir o número na próxima verificação, então altere o mínimo e o máximo do autoscaler em vez disso.

### Reiniciar

**Reiniciar** adiciona a anotação `kubectl.kubernetes.io/restartedAt` ao template de Pod, exatamente como o `kubectl rollout restart` faz: a carga de trabalho faz o rollout de um novo conjunto de Pods seguindo a estratégia dela (um novo ReplicaSet para um Deployment), e os Pods antigos saem quando os novos estão prontos.

### CronJobs

**Suspender** impede um CronJob de iniciar novos Jobs (os que já estão em execução continuam) e **Retomar** o deixa iniciá-los de novo; um CronJob suspenso aparece como **Pausado**. **Executar agora** inicia na hora um Job a partir do template dele, chamado `<cronjob>-manual-<id>`, pertencente ao CronJob e marcado com `cronjob.kubernetes.io/instantiate: manual` como os do kubectl. Ele aparece na aba **Jobs** do CronJob.

## Excluir

A página de qualquer objeto — uma carga de trabalho, um Pod, um Service, um Ingress, um ConfigMap, um Secret, um Node ou qualquer objeto em **Recursos da API** — tem **Excluir** no cabeçalho. Em uma lista, marque linhas e **Excluir N** remove todas elas depois de uma única confirmação que as lista.

Os objetos são excluídos com propagação `Background`, como o `kubectl delete` faz: o objeto sai na hora e o coletor de lixo remove o que pertence a ele (os ReplicaSets e Pods de um Deployment, os Jobs de um CronJob).

A confirmação diz o que acontece em seguida:

- **Um Pod que pertence a um controlador** — um ReplicaSet, StatefulSet, DaemonSet ou Job — volta: o controlador inicia um novo no lugar dele. Exclua ou escale a carga de trabalho para se livrar dele.
- **Um Namespace** leva tudo o que está nele: as cargas de trabalho, Pods, Services, ConfigMaps e Secrets.
- **Um Node** é removido da lista do cluster sem ser drenado antes.

## Quando não funciona

- **Não permitido**: uma alteração que as suas credenciais não podem fazer diz isso com o verbo e o recurso — "Suas credenciais não permitem “patch” em deployments no namespace shop." — e o comando `kubectl auth can-i` para conferir.
- **Alterado nesse meio-tempo**: quando outra pessoa alterou o objeto entre a sua consulta e a sua ação, o Homeport avisa; consulte-o de novo e tente outra vez.
- **Já não existe**: excluir algo já excluído diz que ele não existe mais.
