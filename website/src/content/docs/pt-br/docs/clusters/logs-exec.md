---
title: Logs e terminais
description: Acompanhe os logs de um Pod ou de uma carga de trabalho inteira, veja por que um contêiner falhou e abra um terminal em um contêiner de um Pod.
---

A página de um Pod tem as abas **Logs** e **Terminal**, e a página de uma carga de trabalho tem uma aba **Logs** para todos os Pods dela: o `kubectl logs` e o `kubectl exec -it` do dia a dia, pela API do Kubernetes com as próprias credenciais do cluster.

## Logs

**Logs** transmite os logs de um contêiner à medida que são gravados. É o mesmo visualizador que o de um contêiner em um host:

- **Histórico**: começar pelas últimas 100, 1.000 ou 10.000 linhas, ou por todas elas.
- **Desde**: apenas as linhas dos últimos minutos, horas ou dias.
- **Carimbos de data/hora**: o momento em que o Kubernetes registrou cada linha.
- **Acompanhar**: manter a linha mais recente à vista; desative para reler enquanto novas linhas continuam chegando.
- **Localizar**, **Limpar** e **Baixar** (como arquivo de texto, as linhas mostradas até então).

Um Pod com vários contêineres tem um seletor de **Contêiner**, com os contêineres de inicialização marcados. Ele abre no contêiner que o `kubectl` escolheria: o indicado pela anotação `kubectl.kubernetes.io/default-container` do Pod, senão o primeiro contêiner de aplicação dele.

O Kubernetes mantém juntas a saída padrão e a saída de erro de um contêiner, então não há filtro stdout/stderr como há para os contêineres de um host.

A transmissão para quando você sai da aba, e avisa quando o contêiner para.

### Por que ele falhou?

Quando um contêiner reiniciou, **Instância anterior** mostra os logs da execução anterior ao último reinício — para um Pod em `CrashLoopBackOff`, a execução que falhou —, como o `kubectl logs --previous`. Fica desativado para um contêiner que nunca reiniciou, já que não há nada a mostrar.

Quando um contêiner ainda não iniciou (`ContainerCreating`, `CrashLoopBackOff` enquanto aguarda para iniciar de novo), os logs dizem isso com o motivo, no lugar. A página do Pod dele diz por que ele aguarda.

### Os logs de uma carga de trabalho

A aba **Logs** de um Deployment, StatefulSet, DaemonSet ou Job intercala os logs de todos os Pods dele à medida que são gravados, cada linha identificada com o Pod dela na própria cor, como o `kubectl logs -l app=api --prefix`. Escolha um Pod para lê-lo sozinho, e o contêiner quando o template dele tem vários.

## Terminal

**Terminal** abre um shell em um dos contêineres em execução do Pod: `bash` quando a imagem o tem, `sh` caso contrário. Em vez disso, digite um comando no campo (`psql -U app shop`, `top`) e **Conectar** o executa, entre aspas como um shell faria.

É um TTY de verdade: `vi`, `top`, cores e teclas de seta funcionam, e redimensionar a janela redimensiona o terminal no contêiner. Quando o processo termina, o terminal diz com qual código; **Reconectar** inicia outro. Sair da aba encerra o processo no contêiner.

### Quando ele não abre

- **Sem shell**: imagens mínimas (distroless) não têm shell algum. Digite um comando que a imagem tenha, ou anexe um contêiner de depuração a partir de um terminal com `kubectl debug -it <pod> --image=busybox --target=<container>`; o Homeport não cria contêineres de depuração.
- **Fora de execução**: um contêiner que está aguardando ou que terminou não pode executar nada. Os Pods de um Job concluído pararam de vez.
- **Não permitido**: abrir um terminal exige `create` em `pods/exec`, e ler logs exige `get` em `pods/log`, o que as funções somente leitura muitas vezes não têm. O Homeport diz qual, com o comando `kubectl auth can-i` para conferir.

O Homeport não copia arquivos para dentro nem para fora de um Pod (`kubectl cp`).
