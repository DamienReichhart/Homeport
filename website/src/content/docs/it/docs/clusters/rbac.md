---
title: RBAC
description: Guarda i Role, i binding e i soggetti di un cluster, cosa può fare ogni soggetto e dove, e cosa possono fare le tue credenziali.
---

**RBAC**, nella barra laterale di un cluster Kubernetes, mostra chi può fare cosa sul cluster. È in sola lettura: Homeport non crea, modifica né elimina oggetti RBAC (puoi comunque modificarli come YAML da [Risorse dell’API](/docs/clusters/api-resources/)).

Come le altre pagine, segue il selettore di namespace: con un namespace scelto, mostra i Role e i RoleBinding di quel namespace, oltre ai ClusterRole e ai ClusterRoleBinding, che valgono ovunque.

## Roles

**Roles** elenca i Role (ciascuno nel suo namespace) e i ClusterRole, con il numero delle loro regole. Aprine uno per la sua pagina:

- le sue **regole**, come le scrive Kubernetes: risorse, gruppi API (`core` per il gruppo core), nomi delle risorse e verbi, oppure URL non di risorse come `/healthz`;
- i **binding che lo usano**, ciascuno con i link ai suoi soggetti.

Un ClusterRole segnato come **aggregato** ha una `aggregationRule`: la sua pagina mostra i suoi selettori di etichette, i ClusterRole che corrispondono, e le sue regole includono le loro. Gli `admin`, `edit` e `view` di Kubernetes sono costruiti così: il ClusterRole di una CRD con l’etichetta `rbac.authorization.k8s.io/aggregate-to-edit: "true"` compare in `edit`.

## Binding

**Binding** elenca i RoleBinding e i ClusterRoleBinding, ciascuno con il ruolo che concede, con il suo link, e i soggetti che nomina: Users, Groups e ServiceAccounts. Un binding il cui ruolo non esiste è segnato come **non trovato**; non concede nulla.

## Soggetti

**Soggetti** elenca tutti quelli che un binding nomina. Apri un User, un Group o un ServiceAccount per vedere cosa può fare, come una matrice di risorse e verbi raggruppata per dove vale: prima **Tutto il cluster**, poi ogni namespace. Sotto, i binding che glielo concedono, e come: nominandolo direttamente o tramite un gruppo.

Homeport unisce le regole di ogni binding che si applica:

- Un ClusterRoleBinding concede il suo ClusterRole ovunque: in ogni namespace, sulle risorse a livello di cluster come i Node e sugli URL non di risorse.
- Un RoleBinding concede il suo ruolo solo nel proprio namespace, anche quando lega un ClusterRole. Un ServiceAccount legato a `edit` da un RoleBinding in `prod` può modificare in `prod` e da nessun’altra parte.
- Contano anche i binding ai gruppi in cui Kubernetes mette ogni soggetto: un ServiceAccount è in `system:serviceaccounts`, `system:serviceaccounts:<il suo namespace>` e `system:authenticated`, ed è anche l’utente `system:serviceaccount:<namespace>:<nome>`; uno User è in `system:authenticated`.
- `*` sta per ogni verbo, gruppo API o risorsa, `*/scale` per quella sottorisorsa di ogni risorsa, e una regola con nomi di risorse vale solo per quegli oggetti (la matrice li mostra sotto **Solo**). Una riga già coperta da una più ampia è omessa.

**È una vista approssimativa.** I permessi di un soggetto sono calcolati dai binding che le tue credenziali possono leggere, non chiesti al server API, quindi non richiedono il permesso di impersonare nessuno. Possono non vedere ciò che permettono altri autorizzatori, come un webhook (comune nei cluster gestiti) o ABAC, né i gruppi che un autenticatore dà a un utente (dai claim OIDC o dall’organizzazione di un certificato client), che conosce solo il server API. Per essere sicuro delle tue credenziali, usa **Cosa posso fare qui?**.

Con un namespace scelto, sono letti solo i RoleBinding di quel namespace; scegli **Tutti i namespace** per vedere ovunque i permessi di un soggetto.

## Cosa posso fare qui?

**Cosa posso fare qui?** è la risposta del server API stesso per le credenziali che Homeport usa, nel namespace scelto (quello predefinito del cluster con **Tutti i namespace**), come la mostra `kubectl auth can-i --list`. Dice con quale identità accedono e in quali gruppi, sui cluster che lo dicono (Kubernetes 1.28 e successivi), con un link alla pagina di quel soggetto.

Quando il server API indica che il suo elenco può essere incompleto — un autorizzatore come un webhook può permettere più di quanto sa elencare — la pagina lo dice.

**Posso…?** pone una domanda, come `kubectl auth can-i`: un verbo, una risorsa e un namespace (vuoto per tutti i namespace) e, se vuoi, un gruppo API, una sottorisorsa e il nome di un oggetto. La risorsa può essere il suo plurale (`secrets`), un nome breve (`deploy`) o il suo plurale con il gruppo (`deployments.apps`); Homeport la risolve come kubectl. La risposta arriva con la stessa domanda come comando da copiare, ad esempio `kubectl auth can-i delete secrets --namespace prod`, che dà la stessa risposta.

Quando le tue credenziali non possono elencare alcun oggetto RBAC, la pagina mostra solo **Cosa posso fare qui?**, sotto un messaggio che dice cosa è stato rifiutato. Quando se ne possono elencare solo alcuni, ad esempio i RoleBinding ma non i ClusterRole, la pagina dice quali, e ciò che mostra li esclude.
