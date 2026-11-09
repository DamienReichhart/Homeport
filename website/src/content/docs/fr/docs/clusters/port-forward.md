---
title: Redirection de port
description: Joignez le port d’un Pod ou d’un Service depuis cet ordinateur, comme kubectl port-forward, sans l’exposer.
---

Une redirection de port rend un port d’un Pod, ou d’un Service, joignable sur cet ordinateur : une base de données, une interface d’administration ou une API interne, sans `LoadBalancer`, sans Ingress et sans terminal laissé ouvert avec `kubectl port-forward`.

## Rediriger un port

- Sur la page d’un **Pod**, les conteneurs listent les ports qu’ils déclarent ; **Rediriger…** se trouve à côté de chaque port TCP.
- Sur la page d’un **Service**, **Rediriger…** se trouve à côté de chacun de ses ports TCP.

Choisissez le port local : il commence au même numéro quand ce port est libre sur cet ordinateur, et vide sinon. Laissez-le vide et Homeport en choisit un libre. Une notification indique où elle écoute, avec **Ouvrir** pour la charger dans votre navigateur.

La redirection n’écoute que sur les adresses de loopback (`127.0.0.1`, et `[::1]` quand le même port y est libre) : rien d’autre sur votre réseau ne peut donc la joindre.

### Pods et Services

Une redirection vers un **Pod** joint ce Pod. Si le Pod disparaît (un déploiement progressif l’a remplacé), la redirection prend fin et Homeport l’indique.

Une redirection vers un **Service** passe par l’un de ses Pods prêts, comme le fait `kubectl port-forward svc/…`, vers le port auquel le Service envoie ce port (son `targetPort`, par numéro ou par le nom d’un port de conteneur). Quand ce Pod disparaît, la connexion suivante en choisit un autre qui est prêt : une redirection vers un Service survit donc aux déploiements progressifs. Elle ne répartit pas les connexions entre les Pods.

Seul le TCP peut être redirigé. Les Services sans sélecteur (qui pointent vers des endpoints externes) ne peuvent pas l’être.

## Redirections de port

**Redirections de port**, dans la barre latérale d’un cluster, liste chaque redirection active, sur tous vos clusters : son adresse locale, le Pod ou le Service et le port qu’elle joint (et, pour un Service, le Pod par lequel elle passe en ce moment), son Namespace et son cluster, et le nombre de connexions ouvertes par elle. **Ouvrir** la charge dans votre navigateur ; **Arrêter** y met fin et ferme ses connexions.

Les redirections ne vivent qu’aussi longtemps que Homeport : le quitter les arrête toutes, et aucune ne revient au lancement suivant. Supprimer ou désactiver un cluster arrête aussi ses redirections.

## Quand cela ne fonctionne pas

- **Port utilisé** : un autre programme écoute déjà sur ce port local. Choisissez-en un autre, ou laissez-le vide.
- **Ports inférieurs à 1024** : la plupart des systèmes ne laissent pas une application y écouter ; choisissez un port local plus élevé.
- **Pas en cours d’exécution**, **aucun Pod prêt** : il n’y a encore rien vers quoi rediriger ; la page du Pod indique pourquoi.
- **Non autorisé** : rediriger demande `create` sur `pods/portforward` ; Homeport le nomme, avec la commande `kubectl auth can-i` pour vérifier.
