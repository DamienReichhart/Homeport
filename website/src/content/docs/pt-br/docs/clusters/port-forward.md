---
title: Encaminhamento de porta
description: Alcance a porta de um Pod ou de um Service a partir deste computador, como o kubectl port-forward, sem expô-la.
---

Um encaminhamento de porta torna uma porta de um Pod, ou de um Service, acessível neste computador: um banco de dados, uma interface de administração ou uma API interna, sem um `LoadBalancer`, um Ingress ou um terminal mantido aberto com `kubectl port-forward`.

## Encaminhar uma porta

- Na página de um **Pod**, os contêineres listam as portas que declaram; **Encaminhar…** fica ao lado de cada porta TCP.
- Na página de um **Service**, **Encaminhar…** fica ao lado de cada uma das portas TCP dele.

Escolha a porta local: ela começa com o mesmo número quando essa porta está livre neste computador, e vazia caso contrário. Deixe-a vazia e o Homeport escolhe uma livre. Uma notificação diz onde ele escuta, com **Abrir** para carregá-lo no seu navegador.

O encaminhamento escuta apenas nos endereços de loopback — `127.0.0.1`, e `[::1]` quando a mesma porta está livre ali —, então nada mais na sua rede consegue alcançá-lo.

### Pods e Services

Um encaminhamento para um **Pod** alcança esse Pod. Se o Pod desaparece (um rollout o substituiu), o encaminhamento termina e o Homeport avisa.

Um encaminhamento para um **Service** passa por um dos Pods prontos dele, como o `kubectl port-forward svc/…` faz, até a porta para a qual o Service envia essa porta (a `targetPort` dele, por número ou pelo nome de uma porta de contêiner). Quando esse Pod desaparece, a próxima conexão escolhe outro pronto, então um encaminhamento para um Service sobrevive aos rollouts. Ele não distribui as conexões entre os Pods.

Só é possível encaminhar TCP. Services sem seletor (que apontam para endpoints externos) não podem ser encaminhados.

## Encaminhamentos de porta

**Encaminhamentos de porta**, na barra lateral de um cluster, lista todos os encaminhamentos em execução, em todos os seus clusters: o endereço local, o Pod ou Service e a porta que ele alcança (e, para um Service, o Pod pelo qual ele passa agora), o Namespace e o cluster, e quantas conexões estão abertas por ele. **Abrir** o carrega no seu navegador; **Parar** o encerra e fecha as conexões dele.

Os encaminhamentos duram apenas enquanto o Homeport estiver aberto: sair dele para todos, e nenhum volta na próxima inicialização. Remover ou desativar um cluster também para os encaminhamentos dele.

## Quando não funciona

- **Porta em uso**: outro programa já escuta nessa porta local. Escolha outra, ou deixe-a vazia.
- **Portas abaixo de 1024**: a maioria dos sistemas não deixa um app escutar nelas; escolha uma porta local mais alta.
- **Fora de execução**, **nenhum Pod pronto**: ainda não há nada para onde encaminhar; a página do Pod diz por quê.
- **Não permitido**: encaminhar exige `create` em `pods/portforward`; o Homeport indica isso, com o comando `kubectl auth can-i` para conferir.
