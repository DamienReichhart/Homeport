---
title: Clavier et accessibilité
description: Utiliser Homeport au clavier, avec des animations réduites et un lecteur d’écran, le contraste qu’il respecte, les limites connues aujourd’hui, et comment signaler un obstacle.
---

Homeport vise à être utilisable au clavier seul, avec un lecteur d’écran et avec des animations réduites, dans chacune de ses [langues](/docs/guides/settings/#apparence). Cette page indique ce qui fonctionne, ce qui ne fonctionne pas encore, et comment nous prévenir quand quelque chose vous bloque.

## Se déplacer

Tout dans Homeport peut être atteint avec <kbd>Tab</kbd> et <kbd>Shift</kbd> <kbd>Tab</kbd>, et utilisé avec <kbd>Enter</kbd> ou <kbd>Space</kbd>. Un anneau visible montre où se trouve le focus.

- **La barre latérale** liste chaque page ; chacune est un lien. <kbd>Mod</kbd> <kbd>1</kbd> à <kbd>9</kbd> les ouvrent dans l’ordre, et la [palette de commandes](/docs/guides/keyboard/#palette-de-commandes) (<kbd>Mod</kbd> <kbd>K</kbd>) atteint chaque page, hôte et action en tapant.
- **Les listes** (conteneurs, images, volumes, réseaux, stacks, hôtes, Pods…) : le nom de chaque ligne est un bouton ou un lien. Allez-y avec <kbd>Tab</kbd> et appuyez sur <kbd>Enter</kbd> pour ouvrir l’élément, comme le fait un clic sur la ligne. Les cases à cocher et le menu de la ligne viennent avant et après lui.
- **Le tri** : l’en-tête d’une colonne triable est un bouton. Appuyez sur <kbd>Enter</kbd> ou <kbd>Space</kbd> pour trier selon cette colonne, et de nouveau pour inverser l’ordre ; les lecteurs d’écran annoncent l’ordre.
- **Les stacks dans la liste des conteneurs** s’ouvrent et se ferment avec le bouton placé avant leur nom, qui indique si elles sont déployées.
- **Les boîtes de dialogue** prennent le focus à leur ouverture, le gardent à l’intérieur tant qu’elles sont ouvertes, et le rendent à ce qui les a ouvertes quand elles se ferment. <kbd>Esc</kbd> ferme une boîte de dialogue sans rien faire.
- **Les menus, listes déroulantes et onglets** se parcourent avec les flèches, <kbd>Home</kbd> et <kbd>End</kbd> ; <kbd>Enter</kbd> choisit, <kbd>Esc</kbd> ferme.
- **Les raccourcis** : <kbd>?</kbd> sur n’importe quelle page liste ceux qui y fonctionnent ; voir [Clavier](/docs/guides/keyboard/).

## Terminaux, journaux et éditeur

Le **Terminal** d’un conteneur, et la visionneuse de journaux, reçoivent toutes les touches tant qu’ils ont le focus, pour que le programme à l’intérieur reçoive aussi <kbd>Tab</kbd>, <kbd>Esc</kbd> et les touches <kbd>Ctrl</kbd>. Quittez-les avec la souris, ou en déplaçant le focus avec les commandes propres à votre lecteur d’écran.

Les éditeurs de compose, de YAML, de jobs et d’unités sont des éditeurs CodeMirror nommés pour les lecteurs d’écran. Dans ces éditeurs, <kbd>Tab</kbd> indente ; appuyez sur <kbd>Esc</kbd> puis sur <kbd>Tab</kbd> pour sortir le focus de l’éditeur.

## Réduction des animations

Quand votre système d’exploitation demande des animations réduites (**Réduire les animations** sous macOS, **Effets d’animation** désactivés sous Windows, **Réduire les animations** sous GNOME et KDE), Homeport se contente de fondus : les boîtes de dialogue, menus, listes déroulantes et infobulles ne zooment ni ne glissent, rien ne pulse, les boutons ne bougent pas quand on appuie dessus et le curseur du terminal cesse de clignoter. Les indicateurs de chargement continuent de tourner, plus lentement, parce qu’ils signalent qu’un travail est en cours. Le réglage est suivi à mesure qu’il change, sans redémarrage.

## Lecteurs d’écran

Chaque page et les principales boîtes de dialogue sont vérifiées avec [axe](https://github.com/dequelabs/axe-core) à chaque modification : les boutons, champs et régions ont des noms, les titres et les repères sont dans l’ordre, et rien ne fait un mauvais usage d’ARIA. Homeport définit la langue de la page, pour qu’un lecteur d’écran lise chaque langue avec sa propre voix, et l’arabe s’affiche de droite à gauche.

La fenêtre de Homeport est une vue web : WebKitGTK sous Linux, WKWebView sous macOS et WebView2 sous Windows. Un audit manuel avec Orca, VoiceOver et NVDA sur chacune est en cours ([#184](https://github.com/DamienReichhart/Homeport/issues/184)) ; tant qu’il n’est pas terminé, considérez que l’expérience avec un lecteur d’écran n’a été testée que par des outils.

### Limites connues

- **Terminaux et journaux** : le texte à l’intérieur du terminal d’un conteneur et de la visionneuse de journaux n’est pas encore lu par les lecteurs d’écran. Copiez la sortie, ou ouvrez les journaux dans un terminal avec `docker logs`.
- **Listes en direct** : les listes se rafraîchissent sur place à mesure que le moteur signale des changements, sans annoncer chaque changement. Les notifications annoncent ce que vous avez fait.
- **Les graphiques** dessinent leurs valeurs ; la dernière valeur de chacun figure aussi dans le titre du graphique, sous forme de texte.

## Contraste

Le texte respecte WCAG 2.2 AA (4,5:1) dans le thème clair comme dans le thème sombre, de même que les bordures des champs, l’anneau de focus et les formes d’état (3:1). L’état ne repose jamais sur la seule couleur : chacun a sa propre forme. Les ratios sont listés dans la charte graphique du projet.

## Signaler un obstacle

Si quelque chose vous empêche d’utiliser Homeport avec votre clavier, votre lecteur d’écran, votre loupe ou une autre technologie d’assistance, [ouvrez une issue](https://github.com/DamienReichhart/Homeport/issues/new?labels=accessibility) avec le label `accessibility`. Indiquez :

- ce que vous essayiez de faire, et où (page, boîte de dialogue, type d’hôte) ;
- votre système d’exploitation et sa version, et la technologie d’assistance et sa version ;
- ce que vous avez entendu ou vu, et ce que vous attendiez.

Les obstacles sont traités comme des bugs, pas comme des demandes de fonctionnalité.
