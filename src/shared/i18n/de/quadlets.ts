/** German text of the `quadlets` namespace. */
import type en from '../en/quadlets';
import type { Translation } from '../types';

export default {
  'quadlets.nav': 'Quadlet',
  'quadlets.title': 'Quadlet-Units',
  'quadlets.summary': '{count, plural, one {# Unit} other {# Units}}',
  'quadlets.search': 'Units durchsuchen',
  'quadlets.empty': 'Keine Quadlet-Units',
  'quadlets.emptyDescription':
    'Quadlet-Units sind Dateien, die Podman in systemd-Services umwandelt, sodass Container mit dem Host starten und systemd sie am Laufen hält. Installieren Sie eine, um sie hier zu sehen.',
  'quadlets.new': 'Neue Unit',
  'quadlets.edit': 'Bearbeiten',
  'quadlets.notGenerated': 'noch nicht erzeugt',
  'quadlets.gone': '{name} ist nicht mehr installiert',
  'quadlets.column.name': 'Unit',
  'quadlets.column.kind': 'Art',
  'quadlets.column.service': 'Service',
  'quadlets.column.status': 'Status',
  'quadlets.column.containers': 'Container',
  'quadlets.status.running': 'Läuft',
  'quadlets.status.starting': 'Wird gestartet',
  'quadlets.status.stopping': 'Wird gestoppt',
  'quadlets.status.stopped': 'Gestoppt',
  'quadlets.status.failed': 'Fehlgeschlagen',
  'quadlets.status.notLoaded': 'Nicht von systemd geladen',
  'quadlets.status.template': 'Vorlage',
  'quadlets.action.start': 'Starten',
  'quadlets.action.stop': 'Stoppen',
  'quadlets.action.restart': 'Neu starten',
  'quadlets.actionOne.start': '{name} starten',
  'quadlets.actionOne.stop': '{name} stoppen',
  'quadlets.actionOne.restart': '{name} neu starten',
  'quadlets.done.start': '{name} gestartet',
  'quadlets.done.stop': '{name} gestoppt',
  'quadlets.done.restart': '{name} neu gestartet',
  'quadlets.installed': '{name} installiert',
  'quadlets.saved': '{name} gespeichert',
  'quadlets.removed': '{name} entfernt',
  'quadlets.blocked.noCommands':
    'Homeport startet und stoppt Units mit systemctl, das es nur über SSH oder für ein Podman auf diesem Computer ausführen kann.',
  'quadlets.blocked.notLoaded':
    'systemd hat den Service dieser Unit nicht erzeugt, daher gibt es nichts zu starten.',
  'quadlets.blocked.podmanTooOld':
    'Lesen, Installieren, Bearbeiten und Entfernen von Units erfordert Podman 5.8 oder neuer; auf diesem Host läuft Podman {version}.',
  'quadlets.blocked.noFile':
    'Diese Unit ist nur über ihre Container bekannt, daher lässt sich ihre Datei nicht ändern.',
  'quadlets.note.labels':
    'Podman {version} kann keine Quadlet-Units auflisten, daher stammen diese aus den Containern, die sie erstellt haben. Ihre Dateien und Units, die keinen Container erstellt haben, erscheinen ab Podman 5.7.',
  'quadlets.note.readOnly':
    'Podman {version} listet Units auf, kann sie aber nicht lesen, installieren, bearbeiten oder entfernen; dafür ist Podman 5.8 oder neuer nötig.',
  'quadlets.note.noCommands':
    'Zum Starten und Stoppen von Units muss dieser Host über SSH erreicht werden, oder es muss ein Podman auf diesem Computer sein.',
  'quadlets.detail.service': 'Service',
  'quadlets.detail.unit': 'systemd-Unit',
  'quadlets.detail.scope': 'Läuft als',
  'quadlets.detail.path': 'Datei',
  'quadlets.detail.app': 'Anwendung',
  'quadlets.detail.pod': 'Pod',
  'quadlets.detail.containers':
    '{count, plural, =0 {Container} one {# Container} other {# Container}}',
  'quadlets.detail.noContainers':
    'Sie hat keinen Container erstellt, oder ihr Service ist gestoppt.',
  'quadlets.scope.user': 'Eine Benutzer-Unit (systemctl --user)',
  'quadlets.scope.system': 'Eine System-Unit',
  'quadlets.file.title': 'Unit-Datei',
  'quadlets.file.unknown': 'Diese Unit ist nur über das Label ihrer Container bekannt.',
  'quadlets.file.tooOld':
    'Podman {version} kann keine Unit-Dateien lesen; zum Anzeigen ist Podman 5.8 oder neuer nötig.',
  'quadlets.form.newTitle': 'Neue Quadlet-Unit',
  'quadlets.form.newDescription':
    'Podman schreibt die Datei dorthin, wo die Units seines Benutzers liegen, und lädt systemd neu. Starten Sie die Unit, sobald sie installiert ist.',
  'quadlets.form.editTitle': '{name} bearbeiten',
  'quadlets.form.editDescription':
    'Beim Speichern wird die Datei ersetzt und systemd neu geladen; starten Sie die Unit neu, damit ein laufender Service sie verwendet. Daneben installierte Dateien bleiben erhalten.',
  'quadlets.form.name': 'Dateiname',
  'quadlets.form.kind': 'Art',
  'quadlets.form.openFile': 'Datei öffnen…',
  'quadlets.form.openTitle': 'Quadlet-Unit-Datei öffnen',
  'quadlets.form.unitFiles': 'Quadlet-Unit-Dateien',
  'quadlets.form.content': 'Unit-Datei',
  'quadlets.form.extra': 'Dateien daneben',
  'quadlets.form.extraHint':
    'Ein Containerfile, Kube-YAML oder eine Env-Datei, die die Unit verwendet, in ihrem Anwendungsordner installiert.',
  'quadlets.form.extraTitle': 'Dateien wählen, die neben der Unit installiert werden',
  'quadlets.form.addExtra': 'Dateien hinzufügen…',
  'quadlets.form.removeExtra': '{name} entfernen',
  'quadlets.form.install': 'Installieren',
  'quadlets.form.save': 'Speichern',
  'quadlets.form.overwrite': 'Überschreiben',
  'quadlets.form.nameInvalid':
    'Benennen Sie sie wie web.container, mit der Endung .container, .pod, .volume, .network, .kube, .image, .build oder .artifact.',
  'quadlets.form.sectionMissing':
    'Die Datei benötigt einen Abschnitt {section}, damit Quadlet sie lesen kann.',
  'quadlets.form.extraInvalid':
    'Jede Datei neben der Unit braucht einen eigenen einfachen Namen, ohne Ordner.',
  'quadlets.remove.title': '{name} entfernen?',
  'quadlets.remove.description':
    'Podman löscht die Unit-Datei und lädt systemd neu. Von ihr erstellte Container, Volumes und Images bleiben erhalten.',
  'quadlets.remove.stop': 'Ihren Service stoppen, falls er läuft',
  'quadlets.remove.stopRunning': 'Zuerst ihren laufenden Service stoppen',
} satisfies Translation<typeof en>;
