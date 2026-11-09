---
title: Tastatur und Barrierefreiheit
description: Homeport mit der Tastatur, mit reduzierter Bewegung und mit einem Screenreader verwenden, der erreichte Kontrast, die heute bekannten Grenzen und wie Sie eine Barriere melden.
---

Homeport soll allein mit der Tastatur, mit einem Screenreader und mit reduzierter Bewegung nutzbar sein, in jeder seiner [Sprachen](/docs/guides/settings/#appearance). Diese Seite sagt, was funktioniert, was noch nicht, und wie Sie uns mitteilen, wenn Ihnen etwas im Weg steht.

## Sich bewegen

Alles in Homeport ist mit <kbd>Tab</kbd> und <kbd>Shift</kbd> <kbd>Tab</kbd> erreichbar und mit <kbd>Enter</kbd> oder <kbd>Space</kbd> bedienbar. Ein sichtbarer Ring zeigt, wo der Fokus ist.

- **Die Seitenleiste** listet jede Seite auf; jede ist ein Link. <kbd>Mod</kbd> <kbd>1</kbd> bis <kbd>9</kbd> öffnen sie der Reihe nach, und die [Befehlspalette](/docs/guides/keyboard/#command-palette) (<kbd>Mod</kbd> <kbd>K</kbd>) erreicht jede Seite, jeden Host und jede Aktion durch Tippen.
- **Listen** (Container, Images, Volumes, Netzwerke, Stacks, Hosts, Pods…): Der Name jeder Zeile ist eine Schaltfläche oder ein Link. Springen Sie mit <kbd>Tab</kbd> dorthin und drücken Sie <kbd>Enter</kbd>, um das Element zu öffnen, wie es ein Klick auf die Zeile tut. Die Kontrollkästchen und das Menü der Zeile kommen davor und danach.
- **Sortieren**: Die Kopfzeile einer sortierbaren Spalte ist eine Schaltfläche. Drücken Sie <kbd>Enter</kbd> oder <kbd>Space</kbd>, um danach zu sortieren, erneut, um umzukehren; Screenreader sagen die Reihenfolge an.
- **Stacks in der Containerliste** öffnen und schließen sich mit der Schaltfläche vor ihrem Namen, die angibt, ob sie aufgeklappt sind.
- **Dialoge** übernehmen beim Öffnen den Fokus, halten ihn darin, solange sie offen sind, und geben ihn beim Schließen an das zurück, was sie geöffnet hat. <kbd>Esc</kbd> schließt einen Dialog, ohne etwas zu tun.
- **Menüs, Auswahllisten und Tabs** bewegen sich mit den Pfeiltasten, <kbd>Home</kbd> und <kbd>End</kbd>; <kbd>Enter</kbd> wählt, <kbd>Esc</kbd> schließt.
- **Tastenkürzel**: <kbd>?</kbd> auf jeder Seite listet diejenigen auf, die dort funktionieren; siehe [Tastatur](/docs/guides/keyboard/).

## Terminals, Logs und der Editor

Das **Terminal** eines Containers und die Log-Ansicht nehmen jede Taste entgegen, solange sie den Fokus haben, sodass das Programm darin auch <kbd>Tab</kbd>, <kbd>Esc</kbd> und <kbd>Ctrl</kbd>-Tasten erhält. Verlassen Sie sie mit der Maus oder indem Sie den Fokus mit den eigenen Befehlen Ihres Screenreaders verschieben.

Die Editoren für Compose, YAML, Jobs und Units sind CodeMirror-Editoren, die für Screenreader benannt sind. Darin rückt <kbd>Tab</kbd> ein; drücken Sie <kbd>Esc</kbd> und dann <kbd>Tab</kbd>, um den Fokus aus dem Editor zu bewegen.

## Reduzierte Bewegung

Wenn Ihr Betriebssystem reduzierte Bewegung verlangt (**Bewegung reduzieren** unter macOS, **Animationseffekte** aus unter Windows, **Reduce animation** unter GNOME und KDE), blendet Homeport nur noch ein und aus: Dialoge, Menüs, Auswahllisten und Tooltips zoomen oder gleiten nicht, nichts pulsiert, Schaltflächen bewegen sich beim Drücken nicht, und der Cursor des Terminals hört auf zu blinken. Ladeanzeigen drehen sich weiter, langsamer, weil sie anzeigen, dass Arbeit im Gange ist. Die Einstellung wird befolgt, sobald sie sich ändert, ohne Neustart.

## Screenreader

Jede Seite und die wichtigsten Dialoge werden bei jeder Änderung mit [axe](https://github.com/dequelabs/axe-core) geprüft: Schaltflächen, Felder und Bereiche haben Namen, Überschriften und Landmarks sind in Ordnung, und nichts missbraucht ARIA. Homeport setzt die Sprache der Seite, sodass ein Screenreader jede Sprache mit ihrer eigenen Stimme liest, und Arabisch wird von rechts nach links angezeigt.

Das Fenster von Homeport ist eine Web-Ansicht: WebKitGTK unter Linux, WKWebView unter macOS und WebView2 unter Windows. Eine praktische Prüfung mit Orca, VoiceOver und NVDA auf jedem System ist im Gange ([#184](https://github.com/DamienReichhart/Homeport/issues/184)); bis sie abgeschlossen ist, betrachten Sie die Screenreader-Erfahrung als nur durch Werkzeuge getestet.

### Bekannte Grenzen

- **Terminals und Logs**: Der Text im Terminal eines Containers und in der Log-Ansicht wird von Screenreadern noch nicht gelesen. Kopieren Sie die Ausgabe oder öffnen Sie die Logs in einem Terminal mit `docker logs`.
- **Live-Listen**: Listen aktualisieren sich an Ort und Stelle, wenn die Engine Änderungen meldet, ohne jede Änderung anzusagen. Toasts sagen an, was Sie getan haben.
- **Diagramme** zeichnen ihre Werte; der neueste Wert jedes Diagramms steht außerdem als Text in seinem Titel.

## Kontrast

Text erfüllt WCAG 2.2 AA (4,5:1) sowohl im hellen als auch im dunklen Design, ebenso die Ränder von Feldern, der Fokusring und die Statusformen (3:1). Der Status verlässt sich nie allein auf Farbe: Jeder hat seine eigene Form. Die Verhältnisse sind im Markenleitfaden des Projekts aufgeführt.

## Eine Barriere melden

Wenn Sie etwas daran hindert, Homeport mit Ihrer Tastatur, Ihrem Screenreader, Ihrer Bildschirmlupe oder einer anderen assistiven Technologie zu verwenden, [eröffnen Sie ein Issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) mit dem Label `accessibility`. Nennen Sie:

- was Sie tun wollten und wo (Seite, Dialog, Host-Art);
- Ihr Betriebssystem und seine Version sowie die assistive Technologie und ihre Version;
- was Sie gehört oder gesehen haben und was Sie erwartet hatten.

Barrieren werden als Fehler behandelt, nicht als Funktionswünsche.
