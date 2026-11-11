---
title: Tastiera
description: Usa Homeport senza mouse — il riquadro comandi e la ricerca in tutti gli host, le scorciatoie per ogni pagina e host, e la scheda di aiuto che le elenca.
---

Homeport si può usare dalla tastiera. Premi <kbd>Mod</kbd> <kbd>K</kbd> per aprire il [riquadro comandi](#riquadro-comandi), e <kbd>?</kbd> in qualsiasi pagina per vedere tutte le scorciatoie che funzionano lì.

**Mod**, più sotto, indica <kbd>⌘</kbd> su macOS e <kbd>Ctrl</kbd> su Windows e Linux; su macOS, <kbd>Alt</kbd> è <kbd>⌥</kbd>.

## Riquadro comandi

<kbd>Mod</kbd> <kbd>K</kbd> apre il riquadro comandi da qualsiasi pagina. Digita ciò che vuoi, spostati con <kbd>↑</kbd> e <kbd>↓</kbd> e premi <kbd>Enter</kbd>; <kbd>Esc</kbd> lo chiude e ti riporta dove eri. Elenca:

- **Vai a**: tutte le pagine della barra laterale; digita `vol` e premi <kbd>Enter</kbd> per aprire Volumi.
- **Host**: tutti gli host con il loro stato; digita parte del nome di un host per passare a quell’host.
- **Azioni**: **Esegui un container**, **Scarica un’immagine**, **Nuovo stack**, **Ripulisci l’host selezionato**, **Aggiungi host**, **Gestisci gli host**, **Aggiungi un cluster**, **Gestisci i cluster**, **Cerca aggiornamenti** e **Mostra le scorciatoie da tastiera**. Un host Podman aggiunge **Nuovo pod** e **Nuova unità**, un host containerd **Scarica immagine** e **Nuovo namespace**, e un host Incus **Nuova istanza** e **Ottieni immagine**.
- **Impostazioni**: **Tema: chiaro**, **Tema: scuro** e **Tema: come il sistema**.

Le azioni che richiedono un host (eseguire un container, scaricare, un nuovo stack, fare pulizia) sono elencate solo quando l’host selezionato è connesso.

### Cercare in tutti gli host

Digita due o più caratteri e il riquadro cerca anche tra i **container**, gli **stack**, le **immagini**, i **volumi** e le **reti** di tutti gli host connessi: «dove gira `postgres`?» ha una risposta senza cambiare host. Trova nomi, ID brevi e, per i container, la loro immagine e il loro stack; i nomi che iniziano con ciò che hai digitato vengono per primi.

- I risultati sono raggruppati per tipo, cinque per tipo fino a **Mostra tutti**. Ognuno mostra il suo host e, per container e stack, se sono in esecuzione: _prod-server · in esecuzione_.
- Sceglierne uno passa al suo host e lo apre: la pagina di un container o di uno stack, oppure la pagina Immagini, Volumi o Reti con la ricerca già impostata.
- Gli host non connessi vengono saltati, e il riquadro dice quanti sono: _2 host non inclusi nella ricerca (non connessi)_.

Homeport carica gli elenchi solo dopo che hai digitato due caratteri, e da lì li tiene aggiornati con gli eventi in tempo reale di ogni host. Log, file e definizioni compose non vengono cercati. Ogni voce mostra la sua scorciatoia, se ne ha una. Senza nulla digitato, le voci usate per ultime vengono per prime, sotto **Recenti**, finché non esci da Homeport.

## Scorciatoie

### Ovunque

| Tasti                                      | Azione                                                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Mostra le scorciatoie da tastiera                                                     |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Apre le pagine della barra laterale nell’ordine: Panoramica, Bacheca, Container…      |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Apre le Impostazioni                                                                  |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Seleziona l’host precedente o successivo, nell’ordine in cui li elenca la pagina Host |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Aggiorna gli elenchi dell’host selezionato: container, immagini, volumi, reti, stack… |

Passa il puntatore su una pagina della barra laterale per vederne i tasti.

### In una pagina

| Tasti        | Azione                                                                                                                                                                                                                                              |
| ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>/</kbd> | Porta il cursore nel campo di ricerca della pagina. <kbd>Esc</kbd> in un campo vuoto lo abbandona.                                                                                                                                                  |
| <kbd>N</kbd> | Avvia l’azione principale della pagina: **Esegui container**, **Scarica**, **Crea volume**, **Crea rete**, **Nuovo stack** o **Aggiungi host**; e nelle pagine di Podman, containerd, Incus e Helm, la loro, come **Nuovo pod** o **Nuova istanza** |

Il pulsante dell’azione principale di una pagina mostra <kbd>N</kbd> quando ci passi sopra il puntatore.

### Mentre digiti

I tasti singoli (<kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> e le frecce con <kbd>Alt</kbd>) non si attivano mai mentre digiti in un campo, nell’editor di uno stack o in una finestra di dialogo: digitano invece il carattere. Le scorciatoie con <kbd>Mod</kbd> funzionano anche lì. Nel **Terminale** di un container, ogni tasto va al container e nessuna scorciatoia si attiva.
