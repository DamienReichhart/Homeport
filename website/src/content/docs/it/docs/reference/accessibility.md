---
title: Tastiera e accessibilità
description: Usare Homeport con la tastiera, il movimento ridotto e uno screen reader, il contrasto che rispetta, i limiti noti oggi, e come segnalare una barriera.
---

Homeport punta a essere utilizzabile con la sola tastiera, con uno screen reader e con il movimento ridotto, in ciascuna delle sue [lingue](/docs/guides/settings/#aspetto). Questa pagina dice cosa funziona, cosa non ancora, e come farci sapere quando qualcosa ti ostacola.

## Spostarsi

Tutto in Homeport si raggiunge con <kbd>Tab</kbd> e <kbd>Shift</kbd> <kbd>Tab</kbd>, e si usa con <kbd>Enter</kbd> o <kbd>Space</kbd>. Un anello visibile mostra dove si trova lo stato attivo.

- **La barra laterale** elenca ogni pagina; ciascuna è un link. Da <kbd>Mod</kbd> <kbd>1</kbd> a <kbd>9</kbd> le aprono in ordine, e il [riquadro comandi](/docs/guides/keyboard/#riquadro-comandi) (<kbd>Mod</kbd> <kbd>K</kbd>) raggiunge ogni pagina, host e azione digitando.
- **Elenchi** (container, immagini, volumi, reti, stack, host, Pod…): il nome di ogni riga è un pulsante o un link. Raggiungilo con <kbd>Tab</kbd> e premi <kbd>Enter</kbd> per aprire l’elemento, come fa un clic sulla riga. Le caselle di controllo e il menu della riga vengono prima e dopo di esso.
- **Ordinamento**: l’intestazione di una colonna ordinabile è un pulsante. Premi <kbd>Enter</kbd> o <kbd>Space</kbd> per ordinare in base ad essa, di nuovo per invertire; gli screen reader annunciano l’ordine.
- **Gli stack nell’elenco dei container** si aprono e si chiudono con il pulsante prima del loro nome, che indica se sono espansi.
- **Le finestre di dialogo** prendono lo stato attivo quando si aprono, lo tengono al loro interno mentre sono aperte, e lo restituiscono a ciò che le ha aperte quando si chiudono. <kbd>Esc</kbd> chiude una finestra di dialogo senza fare nulla.
- **Menu, selettori e schede** si spostano con i tasti freccia, <kbd>Home</kbd> e <kbd>End</kbd>; <kbd>Enter</kbd> sceglie, <kbd>Esc</kbd> chiude.
- **Scorciatoie**: <kbd>?</kbd> su qualsiasi pagina elenca quelle che funzionano lì; vedi [Tastiera](/docs/guides/keyboard/).

## Terminali, log e l’editor

Il **Terminale** di un container, e il visualizzatore dei log, ricevono ogni tasto mentre hanno lo stato attivo, così il programma al loro interno riceve anche <kbd>Tab</kbd>, <kbd>Esc</kbd> e i tasti <kbd>Ctrl</kbd>. Lasciali con il mouse, o spostando lo stato attivo con i comandi propri del tuo screen reader.

Gli editor di compose, YAML, job e unità sono editor CodeMirror con un nome per gli screen reader. Al loro interno, <kbd>Tab</kbd> indenta; premi <kbd>Esc</kbd> e poi <kbd>Tab</kbd> per spostare lo stato attivo fuori dall’editor.

## Movimento ridotto

Quando il sistema operativo chiede di ridurre il movimento (**Riduci movimento** su macOS, **Effetti di animazione** disattivato su Windows, **Reduce animation** su GNOME e KDE), Homeport usa solo dissolvenze: finestre di dialogo, menu, selettori e descrizioni comandi non si ingrandiscono né scorrono, nulla pulsa, i pulsanti non si muovono quando vengono premuti e il cursore del terminale smette di lampeggiare. Gli indicatori di caricamento continuano a girare, più lentamente, perché dicono che c’è un lavoro in corso. L’impostazione viene seguita mentre cambia, senza riavvio.

## Screen reader

Ogni pagina e le finestre di dialogo principali vengono controllate con [axe](https://github.com/dequelabs/axe-core) a ogni modifica: pulsanti, campi e regioni hanno un nome, titoli e punti di riferimento sono in ordine, e nulla usa ARIA in modo scorretto. Homeport imposta la lingua della pagina, così uno screen reader legge ogni lingua con la sua voce, e l’arabo viene mostrato da destra a sinistra.

La finestra di Homeport è una web view: WebKitGTK su Linux, WKWebView su macOS e WebView2 su Windows. È in corso una verifica pratica con Orca, VoiceOver e NVDA su ciascuno ([#184](https://github.com/DamienReichhart/Homeport/issues/184)); finché non è terminata, considera l’esperienza con gli screen reader come testata solo da strumenti automatici.

### Limiti noti

- **Terminali e log**: il testo all’interno del terminale di un container e del visualizzatore dei log non viene ancora letto dagli screen reader. Copia l’output, o apri i log in un terminale con `docker logs`.
- **Elenchi in tempo reale**: gli elenchi si aggiornano sul posto man mano che l’Engine segnala modifiche, senza annunciare ogni modifica. I toast annunciano ciò che hai fatto.
- **I grafici** disegnano i loro valori; l’ultimo valore di ciascuno è anche nel titolo del grafico, come testo.

## Contrasto

Il testo rispetta WCAG 2.2 AA (4.5:1) sia nel tema chiaro sia in quello scuro, e così i bordi dei campi, l’anello dello stato attivo e le forme degli stati (3:1). Lo stato non si affida mai al solo colore: ciascuno ha la sua forma. I rapporti sono elencati nella guida del marchio del progetto.

## Segnalare una barriera

Se qualcosa ti impedisce di usare Homeport con la tastiera, lo screen reader, l’ingranditore o un’altra tecnologia assistiva, [apri una issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) con l’etichetta `accessibility`. Indica:

- cosa stavi cercando di fare, e dove (pagina, finestra di dialogo, tipo di host);
- il tuo sistema operativo e la sua versione, e la tecnologia assistiva e la sua versione;
- cosa hai sentito o visto, e cosa ti aspettavi.

Le barriere vengono trattate come bug, non come richieste di funzioni.
