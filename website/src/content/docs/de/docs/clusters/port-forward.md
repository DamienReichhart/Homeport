---
title: Portweiterleitung
description: Den Port eines Pods oder eines Services von diesem Rechner aus erreichen, wie mit kubectl port-forward, ohne ihn freizugeben.
---

Eine Portweiterleitung macht einen Port eines Pods oder eines Services auf diesem Rechner erreichbar: eine Datenbank, eine Admin-Oberfläche oder eine interne API, ohne `LoadBalancer`, ohne Ingress und ohne ein Terminal, das mit `kubectl port-forward` offen bleibt.

## Einen Port weiterleiten

- Auf der Seite eines **Pods** listen die Container die Ports auf, die sie deklarieren; **Weiterleiten…** steht neben jedem TCP-Port.
- Auf der Seite eines **Services** steht **Weiterleiten…** neben jedem seiner TCP-Ports.

Wählen Sie den lokalen Port: Er beginnt mit derselben Nummer, wenn dieser Port auf diesem Rechner frei ist, und ist sonst leer. Lassen Sie ihn leer, und Homeport wählt einen freien. Eine Benachrichtigung sagt, wo er lauscht, mit **Öffnen**, um ihn in Ihrem Browser zu laden.

Die Weiterleitung lauscht nur auf den Loopback-Adressen – `127.0.0.1` und `[::1]`, wenn derselbe Port dort frei ist –, sodass nichts anderes in Ihrem Netzwerk sie erreichen kann.

### Pods und Services

Eine Weiterleitung an einen **Pod** erreicht diesen Pod. Wenn der Pod verschwindet (ein Rollout hat ihn ersetzt), endet die Weiterleitung, und Homeport sagt das.

Eine Weiterleitung an einen **Service** geht über einen seiner bereiten Pods, wie `kubectl port-forward svc/…` es tut, an den Port, an den der Service diesen Port sendet (sein `targetPort`, als Nummer oder als Name eines Container-Ports). Wenn dieser Pod verschwindet, wählt die nächste Verbindung einen anderen bereiten, sodass eine Weiterleitung an einen Service Rollouts übersteht. Sie verteilt Verbindungen nicht auf mehrere Pods.

Nur TCP kann weitergeleitet werden. Services ohne Selektor (die auf externe Endpunkte zeigen) können es nicht.

## Portweiterleitungen

**Portweiterleitungen** in der Seitenleiste eines Clusters listet jede laufende Weiterleitung über alle Ihre Cluster hinweg auf: ihre lokale Adresse, den Pod oder Service und den Port, den sie erreicht (und bei einem Service den Pod, über den sie gerade geht), ihren Namespace und Cluster und wie viele Verbindungen über sie offen sind. **Öffnen** lädt sie in Ihrem Browser; **Stoppen** beendet sie und schließt ihre Verbindungen.

Weiterleitungen leben nur so lange wie Homeport: Beenden Sie es, stoppen sie alle, und keine kommt beim nächsten Start zurück. Das Entfernen oder Deaktivieren eines Clusters stoppt auch seine Weiterleitungen.

## Wenn es nicht funktioniert

- **Port belegt**: Ein anderes Programm lauscht bereits auf diesem lokalen Port. Wählen Sie einen anderen oder lassen Sie ihn leer.
- **Ports unter 1024**: Die meisten Systeme lassen eine App nicht darauf lauschen; wählen Sie einen höheren lokalen Port.
- **Läuft nicht**, **kein bereiter Pod**: Es gibt noch nichts, wohin weitergeleitet werden kann; die Seite des Pods sagt, warum.
- **Nicht erlaubt**: Weiterleiten braucht `create` auf `pods/portforward`; Homeport nennt das, mit dem Befehl `kubectl auth can-i` zur Prüfung.
