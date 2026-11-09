---
title: Plugins exec
description: Como o Homeport faz login em clusters EKS, GKE e AKS com o plugin de credenciais exec que o kubeconfig deles indica, e o que é executado onde.
---

Os kubeconfigs da nuvem não contêm uma senha nem um certificado: o usuário deles indica um **plugin exec**, um programa no seu computador que imprime um token de curta duração quando solicitado. O Homeport executa o mesmo programa que o `kubectl` executaria, neste computador, seguindo o protocolo `ExecCredential` do Kubernetes (`client.authentication.k8s.io/v1` e `v1beta1`).

## Provedores comuns

| Provedor | O usuário do kubeconfig executa      | Instalar                                                                      |
| -------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| EKS      | `aws eks get-token --cluster-name …` | a AWS CLI, versão 2                                                           |
| GKE      | `gke-gcloud-auth-plugin`             | a Google Cloud CLI, depois `gcloud components install gke-gcloud-auth-plugin` |
| AKS      | `kubelogin get-token …`              | `az aks install-cli`, ou o kubelogin a partir das releases dele               |

Escreva o kubeconfig como de costume — `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` seguido de `kubelogin convert-kubeconfig` — e depois [adicione os contextos dele](/docs/clusters/).

## O que é executado onde

- O plugin é executado **neste computador**, como você, com os argumentos e as variáveis de ambiente que o kubeconfig dá a ele (`AWS_PROFILE`, por exemplo). O Homeport nunca pede que ele seja interativo e não lhe passa nenhuma entrada.
- Um inicializador da área de trabalho inicia o Homeport com um `PATH` curto, então o Homeport também procura nos locais de instalação habituais das CLIs da AWS, do Google Cloud e do Azure e do kubelogin (Homebrew, a própria pasta do SDK, `~/.local/bin`, `~/.krew/bin`, Program Files…). Um plugin indicado por caminho completo no kubeconfig é executado a partir dali.
- O token ou o certificado que ele imprime é mantido **apenas na memória**, até pouco antes de expirar; o plugin então é executado de novo. Ele é executado uma vez por validade de token, não uma vez por requisição. Quando a API recusa um token, o Homeport pede um novo ao plugin uma vez antes de mostrar **Acesso recusado**.
- Os caches do próprio plugin (`~/.kube/cache`, `~/.aws/sso/cache`, a configuração do gcloud) são assunto dele; o Homeport não grava nada ali.

## Quando ele não consegue fazer login

- **"… não está instalado neste computador"** — o programa não foi encontrado. Instale-o (a mensagem diz como, usando o `installHint` do kubeconfig quando ele tem um), confira se ele funciona em um terminal e depois **Reconectar**.
- **"… precisa que você faça login, o que ele não consegue fazer dentro do Homeport"** — o plugin quer um navegador ou um código de dispositivo (AWS SSO, login de dispositivo do Azure, uma sessão do `gcloud` expirada). Execute o `kubectl` uma vez em um terminal com esse contexto, faça login ali e depois escolha **Reconectar**: o plugin então encontra o login em cache.
- **"… não conseguiu obter credenciais para …"** — ele foi executado mas falhou; **Detalhes** mostra o final do que ele imprimiu. O Homeport tenta de novo sozinho.

Veja [Solução de problemas](/docs/reference/troubleshooting/#clusters) para as outras mensagens.
