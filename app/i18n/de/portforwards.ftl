### The `portforwards` namespace, in Deutsch (de).
###
### Written by `bun run i18n:fluent` from src/shared/i18n/de/portforwards.ts:
### change the text there, then run it again.

portforwards-nav = Portweiterleitungen
portforwards-title = Portweiterleitungen
portforwards-summary = { $count ->
        [one] { NUM($count) } Weiterleitung
       *[other] { NUM($count) } Weiterleitungen
    }
portforwards-empty-title = Keine Portweiterleitungen
portforwards-empty-description = Leiten Sie auf der Seite eines Pods oder eines Service einen Port weiter, um ihn von diesem Computer aus zu erreichen. Weiterleitungen enden, wenn Homeport beendet wird.
portforwards-column-local = Lokale Adresse
portforwards-column-target = Leitet weiter an
portforwards-column-namespace = Namespace
portforwards-column-cluster = Cluster
portforwards-column-connections = Verbindungen
portforwards-forward = Weiterleiten…
portforwards-forwardPort = Port { $port } weiterleiten
portforwards-open = Öffnen
portforwards-stop = Stoppen
portforwards-stopFailed = Homeport konnte die Weiterleitung nicht stoppen
portforwards-started = { $address } wird an { $target }:{ $port } weitergeleitet
portforwards-failed = Eine Portweiterleitung ist fehlgeschlagen
portforwards-ended = Eine Portweiterleitung wurde beendet
portforwards-dialog-title = { $target }:{ $port } weiterleiten
portforwards-dialog-description = Im Namespace { $namespace }. Erreichen Sie es über den lokalen Port, solange die Weiterleitung läuft.
portforwards-dialog-localPort = Lokaler Port
portforwards-dialog-anyPort = Beliebiger freier Port
portforwards-dialog-loopback = Nur dieser Computer kann sich verbinden: Die Weiterleitung lauscht auf 127.0.0.1 (und ::1).
portforwards-dialog-submit = Weiterleiten
