---
title: CPU e memória
description: Veja quanto de CPU e memória os Pods, seus contêineres e os Nodes usam, em relação às solicitações, aos limites e ao alocável, com o metrics-server.
---

Quando um cluster executa o [metrics-server](https://github.com/kubernetes-sigs/metrics-server), o Homeport mostra quanto de CPU e memória cada Pod e cada Node usa: nas listas e em gráficos ao vivo nas páginas deles. Ele lê a Metrics API (`metrics.k8s.io`) que o metrics-server oferece, como o `kubectl top` faz.

## O que é preciso

O metrics-server já precisa estar em execução no cluster: o Homeport não instala nada em um cluster. Se ele está lá depende de como o cluster foi criado:

| Cluster                    | metrics-server                                                                   |
| -------------------------- | -------------------------------------------------------------------------------- |
| GKE                        | Instalado                                                                        |
| AKS                        | Instalado                                                                        |
| k3s e k3d                  | Instalado (a menos que o k3s tenha sido iniciado com `--disable metrics-server`) |
| Docker Desktop             | Instalado com o Kubernetes dele                                                  |
| kind                       | Não instalado                                                                    |
| EKS                        | Não instalado: adicione o complemento metrics-server ou instale-o você mesmo     |
| minikube                   | Não instalado: `minikube addons enable metrics-server`                           |
| Outros clusters (kubeadm…) | Em geral não instalado                                                           |

Para instalá-lo você mesmo, siga [as instruções dele](https://github.com/kubernetes-sigs/metrics-server#installation); em um cluster local cujos kubelets têm certificados autoassinados (kind), o metrics-server também precisa de `--kubelet-insecure-tls`.

Sem o metrics-server nada quebra: as colunas de CPU e memória não aparecem, e a aba **Métricas** de um Pod ou de um Node diz que o metrics-server não está instalado, com um link para ele. Logo depois de instalado, o metrics-server precisa de cerca de um minuto até ter amostras; enquanto isso, a aba diz que a Metrics API ainda não responde.

Suas credenciais também precisam poder ler métricas: `list` e `get` em `pods.metrics.k8s.io` no Namespace (ou em todos), e em `nodes.metrics.k8s.io` para os Nodes. Quando não podem, a aba **Métricas** diz qual permissão falta, e todas as outras páginas continuam funcionando.

## Nas listas

Em **Pods**, as colunas **CPU** e **Memória** mostram quanto cada Pod usa, somando seus contêineres, e que fração isso é do limite dele, ou da solicitação quando não há limite. Um Pod só tem limite quando todos os contêineres dele têm um: um contêiner sem limite pode usar o Node inteiro. A CPU fica em milicores, como o `kubectl top` escreve (`250m` é um quarto de core), e a memória é o working set, pelo qual o kubelet despeja Pods.

Em **Nodes**, **CPU usada** e **Memória usada** mostram quanto cada Node usa e a fração dele do que é **alocável** aos Pods.

Clique no cabeçalho de uma coluna para ordenar por ela: os Pods ou Nodes mais carregados primeiro.

## Na página de um Pod ou de um Node

A aba **Métricas** de um Pod traça a CPU e a memória de cada contêiner, com uma linha tracejada na solicitação e outra no limite, quando definidos. A de um Node traça a CPU e a memória dele com uma linha no alocável.

O metrics-server lê os kubelets a cada 15 segundos, e o Homeport pede uma amostra nova no mesmo ritmo, então uma mudança de carga aparece no gráfico em até 30 segundos. As amostras ficam guardadas enquanto a página está aberta, por no máximo 15 minutos, qualquer que seja a aba, e só na memória: fechar a página para as leituras e as esquece. O Homeport não guarda histórico de métricas; para isso, use um sistema de monitoramento como o Prometheus.
