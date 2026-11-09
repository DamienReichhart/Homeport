---
title: Plugin exec
description: Come Homeport accede ai cluster EKS, GKE e AKS con il plugin di credenziali exec indicato dal loro kubeconfig, e cosa viene eseguito dove.
---

I kubeconfig cloud non contengono una password né un certificato: il loro utente indica un **plugin exec**, un programma sul tuo computer che, quando richiesto, stampa un token di breve durata. Homeport esegue lo stesso programma che eseguirebbe `kubectl`, su questo computer, seguendo il protocollo `ExecCredential` di Kubernetes (`client.authentication.k8s.io/v1` e `v1beta1`).

## Provider comuni

| Provider | L’utente del kubeconfig esegue       | Installazione                                                               |
| -------- | ------------------------------------ | --------------------------------------------------------------------------- |
| EKS      | `aws eks get-token --cluster-name …` | la AWS CLI, versione 2                                                      |
| GKE      | `gke-gcloud-auth-plugin`             | la Google Cloud CLI, poi `gcloud components install gke-gcloud-auth-plugin` |
| AKS      | `kubelogin get-token …`              | `az aks install-cli`, o kubelogin dalle sue release                         |

Scrivi il kubeconfig come al solito — `aws eks update-kubeconfig`, `gcloud container clusters get-credentials`, `az aks get-credentials` seguito da `kubelogin convert-kubeconfig` — poi [aggiungi i suoi contesti](/docs/clusters/).

## Cosa viene eseguito dove

- Il plugin viene eseguito **su questo computer**, con il tuo utente, con gli argomenti e le variabili d’ambiente che il kubeconfig gli fornisce (`AWS_PROFILE`, per esempio). Homeport non gli chiede mai di essere interattivo e non gli fornisce alcun input.
- Un launcher del desktop avvia Homeport con un `PATH` ridotto, quindi Homeport cerca anche nelle posizioni di installazione abituali delle CLI di AWS, Google Cloud e Azure e di kubelogin (Homebrew, la cartella propria dell’SDK, `~/.local/bin`, `~/.krew/bin`, Program Files…). Un plugin indicato con il percorso completo nel kubeconfig viene eseguito da lì.
- Il token o il certificato che stampa viene conservato **solo in memoria**, fino a poco prima della scadenza; poi il plugin viene eseguito di nuovo. Viene eseguito una volta per ogni durata del token, non una volta per richiesta. Quando l’API rifiuta un token, Homeport chiede una volta al plugin un nuovo token prima di mostrare **Accesso negato**.
- Le cache proprie del plugin (`~/.kube/cache`, `~/.aws/sso/cache`, la configurazione di gcloud) sono affar suo; Homeport non vi scrive nulla.

## Quando non riesce ad accedere

- **«… non è installato su questo computer»** — il programma non è stato trovato. Installalo (il messaggio dice come, usando l’`installHint` del kubeconfig quando ne ha uno), controlla che funzioni in un terminale, poi **Riconnetti**.
- **«… richiede che tu effettui l’accesso, cosa che non può fare all’interno di Homeport»** — il plugin vuole un browser o un codice dispositivo (AWS SSO, accesso con dispositivo di Azure, una sessione `gcloud` scaduta). Esegui una volta `kubectl` in un terminale con quel contesto, accedi lì, poi scegli **Riconnetti**: il plugin troverà allora il suo accesso in cache.
- **«… non è riuscito a ottenere le credenziali per …»** — è stato eseguito ma non è riuscito; **Dettagli** mostra la parte finale di ciò che ha stampato. Homeport riprova da solo.

Vedi [Risoluzione dei problemi](/docs/reference/troubleshooting/#cluster) per gli altri messaggi.
