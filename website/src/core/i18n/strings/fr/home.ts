import type { Translation } from '../../types';
import type en from '../en/home';

/** The home page, in French. */
export default {
  // Home: the hero and its board.
  'home.title': 'Homeport — Chaque Docker Engine, sur un seul tableau.',
  'home.hero.new': 'Nouveau',
  'home.hero.release': 'Homeport {version} est disponible',
  'home.hero.title': 'Chaque Docker Engine, sur un seul tableau.',
  'home.hero.lead':
    'Homeport s’exécute sur votre machine et joint vos hôtes via SSH, TLS ou TCP. Aucun agent sur le serveur. Gratuit, open source, Windows, macOS, Linux.',
  'home.hero.source': 'Lire le code source',
  'home.board.caption':
    'Un exemple de tableau : cinq hôtes, leurs moteurs et la façon dont Homeport les joint. L’un d’eux demande votre attention.',
  'home.board.host': 'Hôte',
  'home.board.engine': 'Moteur',
  'home.board.via': 'Via',
  'home.board.stacks': 'Stacks',
  'home.board.status': 'État',
  'home.board.allRunning': 'Tout fonctionne',
  'home.board.needsYou': '1 stack a besoin de vous',
  'home.board.note': 'Ce qui va bien reste sobre. Seul ce qui a besoin de vous passe au rouge.',
  'home.runtimes.label': 'Fonctionne avec',
  'home.runtimes.title': 'Fonctionne avec ce que vous utilisez déjà',

  // Home: where it lives.
  'home.visits.eyebrow': 'Où il vit',
  'home.visits.title':
    'Homeport reste sur votre machine et rend visite à vos serveurs. Il ne s’y installe jamais.',
  'home.visits.agent.title': 'Aucun agent',
  'home.visits.agent.body':
    'Rien à installer, à mettre à jour ni à sécuriser sur le serveur. Homeport parle au moteur qui s’y trouve déjà.',
  'home.visits.panel.title': 'Aucun panneau web',
  'home.visits.panel.body':
    'Aucun tableau de bord déployé sur vos serveurs, et aucune page de connexion qui attend que quelqu’un la trouve.',
  'home.visits.account.title': 'Aucun compte',
  'home.visits.account.body':
    'Pas d’inscription, pas de licence par poste, pas d’abonnement et pas de télémétrie. Il vous montre vos conteneurs, et c’est tout.',
  'home.visits.ports.title': 'Ce qu’il lui faut sur le serveur, en toute franchise',
  'home.visits.ports.ssh':
    'Le port que vous avez déjà ouvert. Avec la clé que crée Homeport, une ligne dans {file}.',
  'home.visits.ports.tls':
    'Le port de l’API du moteur (en général {port}), que vous exposez vous-même.',
  'home.visits.ports.tcp':
    'Le port de l’API du moteur (en général {port}), sans chiffrement. Uniquement sur un réseau de confiance.',
  'home.visits.ports.advice':
    'Quand vous le pouvez, utilisez SSH : il n’ajoute rien au serveur que vous n’exécutiez déjà.',
  'home.visits.link': 'Comment fonctionnent les connexions',

  // Home: the features.
  'home.features.eyebrow': 'Fonctionnalités',
  'home.features.title': 'Tout ce qu’exécutent vos moteurs, dans une seule fenêtre',
  'home.features.lead':
    'Le quotidien de Docker — ce qui tourne, ce que ça dit, ce que ça coûte — sans avoir à se souvenir du serveur où ça se trouve.',
  'home.features.hosts.title': 'Des hôtes proches et lointains',
  'home.features.hosts.body':
    'Les moteurs locaux et les machines Podman sont trouvés pour vous. Les moteurs distants s’ajoutent via SSH, en passant par des bastions si besoin, via TLS mutuel ou TCP, ou directement depuis votre ~/.ssh/config et vos contextes Docker. Chaque hôte se reconnecte de lui-même quand le réseau a une coupure.',
  'home.features.containers.title': 'Les conteneurs d’un coup d’œil',
  'home.features.containers.body':
    'Groupés par stack, avec le CPU et la mémoire en direct et la raison pour laquelle un conteneur n’est pas en bonne santé. Exécutez-les avec des limites, des périphériques et des healthchecks, puis modifiez, dupliquez, commitez ou exportez un conteneur.',
  'home.features.stacks.title': 'Stacks Compose',
  'home.features.stacks.body':
    'Partez d’un modèle, liez un dossier de projet de cet ordinateur ou suivez un dépôt git, puis déployez sur n’importe quel hôte — builds compris. Voyez ce qui change avant chaque redéploiement ou synchronisation, revenez en arrière, et passez à des images plus récentes en une étape.',
  'home.features.swarm.title': 'Docker Swarm',
  'home.features.swarm.body':
    'Les Swarms apparaissent via leurs hôtes managers. Gérez les nœuds, services, tâches, secrets et configs, déployez des stacks sur un Swarm, et créez-en un, rejoignez-le ou quittez-le depuis un hôte, sans rien installer sur aucun nœud.',
  'home.features.podman.title': 'Podman, en tant que Podman',
  'home.features.podman.body':
    'Homeport reconnaît Podman à la connexion et ajoute ce que seul Podman propose : les pods, créés avec leurs ports et dans lesquels exécuter des conteneurs, et les unités Quadlet, installées, modifiées et démarrées sous systemd, via l’API de Podman lui-même.',
  'home.features.containerd.title': 'containerd, directement',
  'home.features.containerd.body':
    'Connectez-vous à containerd sur son socket, en local ou via SSH, sans Docker : choisissez un espace de noms, démarrez, arrêtez et envoyez des signaux aux tâches de ses conteneurs, lisez leurs journaux, ouvrez un terminal, récupérez des images avec vos identifiants de registre, et trouvez les instantanés qui remplissent votre disque.',
  'home.features.incus.title': 'Instances Incus',
  'home.features.incus.body':
    'Ajoutez Incus via son socket, SSH ou HTTPS avec un jeton de confiance, et exécutez ses conteneurs système et ses machines virtuelles : créez-les depuis une image distante, démarrez-les, gelez-les et arrêtez-les, ouvrez leur console ou un shell, prenez et restaurez des instantanés, et gérez les images, profils, stockages et réseaux.',
  'home.features.clusters.title': 'Clusters Kubernetes et Nomad',
  'home.features.clusters.body':
    'Ajoutez des clusters Kubernetes depuis votre kubeconfig — EKS, GKE et AKS compris — ou trouvez les clusters locaux en un clic. Parcourez en direct les charges de travail, Pods, Services, la configuration et toute ressource de l’API ; suivez les journaux, ouvrez un terminal, redirigez des ports, mettez à l’échelle et appliquez du YAML avec un aperçu, et installez des charts Helm, mettez-les à niveau ou revenez à une révision antérieure. Ajoutez des clusters Nomad par leur adresse pour exécuter des jobs depuis du HCL avec leur plan, les mettre à l’échelle et les arrêter, suivre les journaux, ouvrir un shell et drainer des nœuds. Rien n’est installé sur un cluster.',
  'home.features.images.title': 'Images',
  'home.features.images.body':
    'Cherchez sur Docker Hub, récupérez et poussez des images avec vos identifiants de registre, buildez depuis un dossier de votre ordinateur, enregistrez et chargez des archives, et voyez quand une image plus récente est disponible.',
  'home.features.volumes.title': 'Volumes et réseaux',
  'home.features.volumes.body':
    'Voyez quels conteneurs utilisent quoi. Créez, connectez et déconnectez, avec une suppression qui vous dit exactement ce que vous allez perdre.',
  'home.features.board.title': 'Un seul tableau pour tous les hôtes',
  'home.features.board.body':
    'Chaque conteneur de chaque hôte dans un seul tableau, filtré et géré en lot. Chaque hôte a aussi sa page : les détails du moteur, l’espace disque utilisé avec un nettoyage en une seule boîte de dialogue, et un flux en direct de ce que fait le moteur.',
  'home.features.keyboard.title': 'Le clavier d’abord',
  'home.features.keyboard.body':
    'Une palette de commandes pour aller n’importe où, changer d’hôte ou lancer une commande, et pour trouver n’importe quel conteneur, image ou volume sur l’hôte qui l’a. Des raccourcis pour chaque page, listés avec ?.',
  'home.features.background.title': 'Une surveillance en arrière-plan',
  'home.features.background.body':
    'L’état de chaque hôte dans la zone de notification, des notifications quand un conteneur plante ou qu’un hôte se déconnecte, et Homeport qui démarre à l’ouverture de session, masqué jusqu’à ce que vous en ayez besoin.',

  // Home: Hosts.
  'home.hosts.eyebrow': 'Hôtes',
  'home.hosts.title': 'Des moteurs distants, sans cérémonie',
  'home.hosts.body':
    'Ajoutez un serveur une fois, et il reste sur le tableau. Pour SSH, Homeport crée une clé rien que pour cet hôte et vous donne la seule commande qui l’autorise. Il vérifie l’empreinte du serveur au premier contact, et vous prévient si elle change un jour.',
  'home.hosts.import':
    'Vous les avez déjà décrits ? Importez vos serveurs depuis {file}, serveurs de rebond compris, ou vos contextes Docker, certificats compris. Vous changez d’ordinateur ? Emportez-les dans un seul fichier verrouillé par une phrase de passe.',
  'home.hosts.socket.name': 'Socket local',
  'home.hosts.socket.detail': 'Sockets Unix et canaux nommés Windows, détectés automatiquement.',
  'home.hosts.ssh.detail':
    'Une clé Ed25519 dédiée par hôte, votre propre clé, ssh-agent ou Pageant ; via des bastions aussi.',
  'home.hosts.tls.detail': 'TLS mutuel avec votre CA, votre certificat client et sa clé.',
  'home.hosts.tcp.detail':
    'TCP simple, sans chiffrement, uniquement pour un moteur sur un réseau de confiance.',
  'home.hosts.link': 'Comment fonctionnent les connexions',
  'home.hosts.screenshot':
    'La page Ajouter un hôte avec SSH sélectionné : serveur, port, utilisateur, chemin du socket distant et une nouvelle clé pour cet hôte.',

  // Home: containers.
  'home.containers.eyebrow': 'Conteneurs',
  'home.containers.title': 'Tout sur un conteneur, à un clic',
  'home.containers.body':
    'Ouvrez un conteneur, et ses journaux défilent déjà. Changez d’onglet pour un terminal interactif, ses fichiers, des graphiques en direct, ses processus et le document inspect complet — sur un hôte à l’autre bout du monde aussi facilement que sur votre portable.',
  'home.containers.logs.title': 'Journaux',
  'home.containers.logs.body': 'Suivez, cherchez, filtrez par flux et par date, téléchargez.',
  'home.containers.terminal.title': 'Terminal',
  'home.containers.terminal.body': 'Des shells interactifs, autant qu’il vous en faut.',
  'home.containers.files.title': 'Fichiers',
  'home.containers.files.body': 'Parcourez, envoyez et téléchargez, les dossiers en .tar.',
  'home.containers.stats.title': 'Statistiques',
  'home.containers.stats.body': 'Graphiques du CPU, de la mémoire, du réseau et des E/S disque.',
  'home.containers.link': 'Travailler avec les conteneurs',
  'home.containers.screenshot':
    'L’onglet Journaux d’un conteneur, qui affiche la sortie en direct avec la recherche, les horodatages et les filtres de flux.',

  // Home: Stacks.
  'home.stacks.eyebrow': 'Stacks',
  'home.stacks.title': 'Des stacks Compose, gérées, liées, synchronisées ou trouvées',
  'home.stacks.body':
    'Gardez un fichier compose et son environnement dans Homeport et déployez-le sur n’importe quel hôte — partez d’un modèle, combinez plusieurs fichiers, choisissez des profils ; les variables sont stockées chiffrées, jamais écrites sur le disque. Ou liez un dossier de projet de cet ordinateur : Homeport modifie ses fichiers sur place et déploie à partir de lui, builds compris. Ou suivez une branche d’un dépôt git, avec une clé de déploiement ou un jeton : chaque synchronisation montre les nouveaux commits et ce qu’ils changent avant de déployer, et tant que Homeport est ouvert, il les recherche et déploie de lui-même si vous le demandez. Chaque redéploiement montre d’abord ce qui change, chaque déploiement est conservé pour que vous puissiez revenir en arrière, et Mettre à jour récupère les images plus récentes et redéploie en une étape. Les stacks que quelqu’un a déployées à la main apparaissent aussi : démarrez-les, arrêtez-les ou supprimez-les, ou adoptez-les.',
  'home.stacks.link': 'Gérer les stacks',
  'home.stacks.screenshot':
    'La définition d’une stack gérée : l’éditeur de compose.yaml, son environnement et le bouton Enregistrer et redéployer.',

  // Home: security.
  'home.security.eyebrow': 'Sécurité',
  'home.security.title': 'Vos clés restent scellées',
  'home.security.lead':
    'Homeport manipule les identifiants qui ouvrent vos serveurs, et il les traite en conséquence.',
  'home.security.rest.title': 'Chiffré au repos',
  'home.security.rest.body':
    'Les clés SSH, certificats TLS, environnements de stacks et jetons de dépôts sont scellés avec XChaCha20-Poly1305. La clé est conservée dans le trousseau du système, pas à côté des données.',
  'home.security.disk.title': 'Jamais en clair sur le disque',
  'home.security.disk.body':
    'Homeport parle SSH lui-même, donc les clés déchiffrées restent en mémoire. Aucun fichier de clé, aucune modification de votre {dir}.',
  'home.security.home.title': 'Aucune remontée de données',
  'home.security.home.body':
    'Aucun compte, aucune télémétrie, aucun pistage. Homeport parle à vos moteurs, et à rien d’autre. Chaque version est buildée publiquement, avec des sommes de contrôle et la provenance du build.',
  'home.security.link': 'La sécurité et vos données',

  // Home: desktops and the closing call.
  'home.desktops.eyebrow': 'Télécharger',
  'home.desktops.title': 'Natif sur chaque bureau',
  'home.desktops.lead':
    'Une application légère et rapide, construite avec Tauri et Rust — pas un onglet de navigateur déguisé. Des installeurs pour x64 et ARM qui maintiennent Homeport à jour.',
  'home.desktops.windows.versions': '10 et 11 · x64 et ARM64',
  'home.desktops.windows.formats': 'Installeur (.exe) · MSI',
  'home.desktops.macos.versions': 'Apple silicon et Intel',
  'home.desktops.macos.formats': 'Image disque (.dmg)',
  'home.desktops.linux.versions': 'x64 et ARM64',
  'home.desktops.linux.formats': 'apt · dnf · zypper · AppImage',
  'home.cta.title': 'Rien à installer sur vos serveurs.',
  'home.cta.body':
    'Téléchargez Homeport sur la machine depuis laquelle vous travaillez et ajoutez vos hôtes. Il est gratuit, open source sous licence GNU AGPL, et développé publiquement sur GitHub.',
  'home.cta.github': 'Voir sur GitHub',
} satisfies Translation<typeof en>;
