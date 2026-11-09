---
title: Pod Podman
description: Elenca, avvia, arresta e rimuovi i pod Podman, creane uno con le sue porte pubblicate ed esegui container al suo interno.
---

Un **pod** è il modo in cui Podman raggruppa i container: condividono una rete (oltre al nome host e all’IPC), quindi si raggiungono a vicenda su `localhost`, e si avviano e si arrestano insieme. Ogni pod ha un **container infra** che contiene ciò che condividono e le porte che il pod pubblica. Homeport mostra i pod sugli host il cui Engine è Podman, tramite l’API di Podman stesso sul socket che l’host usa già; sull’host non viene installato nulla.

La pagina **Pod** compare nella barra laterale solo quando è selezionato un host Podman. Su qualsiasi altro Engine compatibile con Docker non c’è, e un collegamento a essa ne spiega il motivo. Vedi [tipi di Engine](/docs/hosts/#tipi-di-engine) per come Homeport riconosce Podman.

## La pagina Pod

| Colonna       | Mostra                                                                                                                                  |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| **Nome**      | Il nome del pod, con il suo stato indicato da un punto.                                                                                 |
| **Stato**     | **In esecuzione**, **In parte in esecuzione** (alcuni container arrestati), **In pausa**, **Arrestato**, **Creato** o **Non riuscito**. |
| **Container** | Quanti dei suoi container sono in esecuzione, senza contare il container infra.                                                         |
| **Reti**      | Le reti a cui è collegato il pod.                                                                                                       |
| **Creato**    | Quando è stato creato.                                                                                                                  |

Ogni riga ha l’azione adatta al suo stato (**Avvia** per un pod arrestato, **Arresta** per uno in esecuzione) e **Rimuovi**. Seleziona un pod per aprirne la pagina. L’elenco si aggiorna da solo man mano che i container del pod si avviano e si arrestano, che sia da Homeport o da un terminale.

Nella pagina **Container**, un container che gira in un pod mostra il nome del pod accanto al proprio; seleziona il nome per aprire il pod.

## La pagina di un pod

L’intestazione ha **Esegui container** e le azioni del pod: **Avvia**, **Arresta**, **Riavvia**, **Sospendi**, **Riprendi** e **Rimuovi**, secondo quanto consente il suo stato. Ognuna agisce su tutti i container del pod insieme, come fanno `podman pod start` e simili.

- **Container** elenca i container del pod, con il container infra per ultimo. Selezionane uno per aprire la sua pagina di container, con log, terminale e file.
- **Impostazioni** mostra le porte che il pod pubblica, i namespace condivisi dai suoi container, le sue reti, il nome host e le etichette, e il comando `podman pod create` con cui è stato creato, quando Podman lo ha conservato.

## Rimuovere un pod

**Rimuovi** chiede prima conferma e dice quanti container vengono rimossi con il pod. Un pod in esecuzione viene prima arrestato: il pulsante diventa **Arresta e rimuovi**. I volumi del pod vengono conservati.

## Creare un pod

**Nuovo pod** (o `n` nella pagina Pod, o **Nuovo pod** nel riquadro comandi) chiede:

- un **Nome**: lettere, cifre, `_`, `.` e `-`, che inizi con una lettera o una cifra;
- le **Porte** da pubblicare, dalla porta dell’host alla porta del container, come nella finestra Esegui. Lascia vuota la porta dell’host perché Podman ne scelga una libera;
- le **Etichette**, una `key=value` per riga.

Le porte appartengono al pod: sono pubblicate sul suo container infra e raggiungono il container del pod che è in ascolto su quella porta. Si apre la pagina del nuovo pod, pronta per eseguirvi dei container. Un pod non ha container propri finché non ne esegui uno; Podman lo avvia con il primo.

## Eseguire un container in un pod

Su un host Podman, la finestra **Esegui** ha un campo **Pod**, e il pulsante **Esegui container** di un pod apre la finestra con quel pod già scelto. Un container in un pod:

- condivide la rete del pod, quindi non può pubblicare porte né collegarsi a un’altra rete: la finestra nasconde quei campi appena viene scelto un pod;
- usa il nome host e le impostazioni DNS del pod, quindi Homeport rifiuta i suoi;
- mantiene tutto il resto che offre la finestra Esegui: comando, ambiente, montaggi, criterio di riavvio, limiti, capability, dispositivi, controllo di integrità e logging.

Duplica e Modifica non spostano mai un container in un pod: Podman non può spostarvi un container esistente.

## Cosa serve a Homeport

I pod funzionano su Podman 4.0 e successivi, con ogni tipo di connessione (socket locale, SSH, TLS o TCP), perché passano dallo stesso socket di tutto il resto.
