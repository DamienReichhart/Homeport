---
title: Tastatur
description: Homeport ohne Maus bedienen – die Befehlspalette und die Suche über alle Hosts, Tastenkürzel für jede Seite und jeden Host und die Hilfeübersicht, die sie auflistet.
---

Homeport lässt sich mit der Tastatur bedienen. Drücken Sie <kbd>Mod</kbd> <kbd>K</kbd>, um die [Befehlspalette](#command-palette) zu öffnen, und auf jeder Seite <kbd>?</kbd>, um alle Tastenkürzel zu sehen, die dort funktionieren.

**Mod** steht im Folgenden für <kbd>⌘</kbd> unter macOS und für <kbd>Ctrl</kbd> unter Windows und Linux; unter macOS ist <kbd>Alt</kbd> die Taste <kbd>⌥</kbd>.

## Befehlspalette

<kbd>Mod</kbd> <kbd>K</kbd> öffnet die Befehlspalette auf jeder Seite. Tippen Sie, was Sie möchten, bewegen Sie sich mit <kbd>↑</kbd> und <kbd>↓</kbd> und drücken Sie <kbd>Enter</kbd>; <kbd>Esc</kbd> schließt sie und bringt Sie dorthin zurück, wo Sie waren. Sie listet auf:

- **Gehe zu** – jede Seite der Seitenleiste: Tippen Sie `vol` und drücken Sie <kbd>Enter</kbd>, um Volumes zu öffnen.
- **Hosts** – jeden Host mit seinem Status: Tippen Sie einen Teil des Namens eines Hosts, um zu ihm zu wechseln.
- **Aktionen** – **Container ausführen**, **Image pullen**, **Neuer Stack**, **Ausgewählten Host aufräumen**, **Host hinzufügen**, **Hosts verwalten**, **Nach Updates suchen** und **Tastenkürzel anzeigen**.
- **Einstellungen** – **Design: Hell**, **Design: Dunkel** und **Design: Wie das System**.

Aktionen, die einen Host brauchen – einen Container ausführen, pullen, ein neuer Stack, aufräumen –, werden nur aufgelistet, solange der ausgewählte Host verbunden ist.

### Alle Hosts durchsuchen

Tippen Sie zwei oder mehr Zeichen, und die Palette durchsucht außerdem die **Container**, **Stacks**, **Images**, **Volumes** und **Netzwerke** aller verbundenen Hosts – „wo läuft `postgres`?“ lässt sich beantworten, ohne den Host zu wechseln. Sie vergleicht Namen, kurze IDs und bei Containern deren Image und Stack; Namen, die mit dem Getippten beginnen, stehen vorn.

- Die Ergebnisse sind nach Art gruppiert, fünf pro Art bis **Alle anzeigen**. Jedes zeigt seinen Host und bei Containern und Stacks, ob sie laufen: _prod-server · läuft_.
- Wenn Sie eines wählen, wechselt Homeport zu dessen Host und öffnet es: die Seite eines Containers oder eines Stacks oder die Seite Images, Volumes oder Netzwerke, nach ihm durchsucht.
- Hosts, die nicht verbunden sind, werden übersprungen, und die Palette sagt, wie viele: _2 Hosts nicht durchsucht (nicht verbunden)_.

Homeport lädt die Listen erst, wenn Sie zwei Zeichen getippt haben, und hält sie danach über die Live-Ereignisse jedes Hosts aktuell. Logs, Dateien und Compose-Definitionen werden nicht durchsucht. Jeder Eintrag zeigt sein Tastenkürzel, falls er eines hat. Ohne Eingabe stehen die zuletzt verwendeten Einträge vorn, unter **Zuletzt verwendet**, bis Homeport beendet wird.

## Tastenkürzel

### Überall

| Tasten                                     | Bewirkt                                                                                         |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| <kbd>?</kbd>                               | Die Tastenkürzel anzeigen                                                                       |
| <kbd>Mod</kbd> <kbd>1</kbd> … <kbd>9</kbd> | Die Seiten der Seitenleiste der Reihe nach öffnen: Übersicht, Container, Images…                |
| <kbd>Mod</kbd> <kbd>,</kbd>                | Einstellungen öffnen                                                                            |
| <kbd>Alt</kbd> <kbd>↑</kbd> / <kbd>↓</kbd> | Den vorherigen oder nächsten Host auswählen, in der Reihenfolge der Seite Hosts                 |
| <kbd>Mod</kbd> <kbd>R</kbd>                | Die Listen des ausgewählten Hosts aktualisieren: Container, Images, Volumes, Netzwerke, Stacks… |

Fahren Sie in der Seitenleiste mit dem Mauszeiger über eine Seite, um ihre Tasten zu sehen.

### Auf einer Seite

| Tasten       | Bewirkt                                                                                                                                                        |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <kbd>/</kbd> | Das Suchfeld der Seite fokussieren. <kbd>Esc</kbd> in einem leeren Feld verlässt es.                                                                           |
| <kbd>N</kbd> | Die Hauptaktion der Seite starten: **Container ausführen**, **Pullen**, **Volume erstellen**, **Netzwerk erstellen**, **Neuer Stack** oder **Host hinzufügen** |

Die Schaltfläche der Hauptaktion einer Seite zeigt <kbd>N</kbd>, wenn Sie mit dem Mauszeiger darüberfahren.

### Beim Tippen

Einzelne Tasten – <kbd>N</kbd>, <kbd>/</kbd>, <kbd>?</kbd> und die <kbd>Alt</kbd>-Pfeile – lösen nie aus, während Sie in einem Feld, im Editor eines Stacks oder in einem Dialog tippen: Sie tippen stattdessen das Zeichen. Tastenkürzel mit <kbd>Mod</kbd> funktionieren auch dort. Im **Terminal** eines Containers geht jede Taste an den Container, und kein Tastenkürzel löst aus.
