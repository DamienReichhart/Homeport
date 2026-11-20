/** The home page, in French. */
export default {
  // Home: the hero and its map.
  'home.title': 'Homeport — Une seule application pour tous vos moteurs de conteneurs',
  'home.hero.title': 'Une seule application pour tous vos moteurs de conteneurs.',
  'home.hero.lead':
    'Homeport s’exécute sur votre machine et n’installe rien sur vos serveurs. Gratuit et open source.',
  'home.hero.docs': 'Lire la documentation',
  'home.map.label':
    'Un exemple de parc dessiné comme un plan de métro. Cette machine est au terminus de cinq lignes : le socket local dessert laptop, SSH dessert prod-eu-1, nas et edge-1, TLS dessert staging et vps-fra-1, TCP dessert lan-box, et l’API du cluster dessert k8s-prod, jobs et lab. Une stack sur vps-fra-1 est à traiter.',
  'home.map.thisMachine': 'Cette machine',
  'home.map.needsYou': '1 stack à traiter',
  'home.map.keyNeedsYou': 'À traiter',
  'home.map.example': 'Un exemple de parc',

  // Home: the questions people open it for.
  'home.questions.title': 'Une question, un seul endroit.',
  'home.questions.lead':
    'Vous ouvrez Homeport pour savoir quelque chose, puis vous retournez au travail. Chaque réponse est à une étape du Tableau.',
  'home.questions.up.title': 'Est-ce que ça tourne ?',
  'home.questions.up.body':
    'Le Tableau montre tous les hôtes, Swarms et clusters à la fois, et place ce qui est à traiter avant tout ce qui va bien.',
  'home.questions.up.link': 'Le Tableau et la Vue d’ensemble',
  'home.questions.up.screenshot':
    'Le Tableau : une vignette pour chacun des trois hôtes, un conteneur à traiter, puis les conteneurs de tous les hôtes dans un seul tableau.',
  'home.questions.died.title': 'Pourquoi a-t-il planté ?',
  'home.questions.died.body':
    'Ouvrez un conteneur, et ses journaux défilent déjà. Un terminal, ses fichiers, des graphiques en direct et le document inspect ne sont qu’à un onglet de là.',
  'home.questions.died.link': 'Travailler avec les conteneurs',
  'home.questions.died.screenshot':
    'L’onglet Journaux d’un conteneur, qui affiche la sortie en direct avec la recherche, les horodatages et les filtres de flux.',
  'home.questions.changed.title': 'Qu’est-ce qui a changé ?',
  'home.questions.changed.body':
    'Chaque hôte tient l’historique de ce qu’a fait son moteur : le conteneur qui a redémarré, l’image qui a été récupérée, le volume qui a été monté.',
  'home.questions.changed.link': 'Événements',
  'home.questions.changed.screenshot':
    'La page Événements d’un hôte : healthchecks, redémarrages, récupérations et montages, du plus récent au plus ancien, avec la stack à laquelle chacun appartient.',
  'home.questions.act.title': 'Puis changez-le, en voyant d’abord ce qui change.',
  'home.questions.act.body':
    'Un redéploiement montre ce qui va changer avant de s’exécuter. Chaque déploiement est conservé, pour que vous puissiez revenir en arrière, et Mettre à jour récupère les images plus récentes en une étape.',
  'home.questions.act.link': 'Gérer les stacks',
  'home.questions.act.screenshot':
    'La définition d’une stack gérée : l’éditeur de compose.yaml, son environnement et le bouton Déployer.',

  // Home: the lines, and what each needs on the server.
  'home.lines.title': 'Rien ne s’installe.',
  'home.lines.lead':
    'Homeport reste sur votre machine et rend visite à vos serveurs. Aucun agent à installer, aucun panneau web à exposer, aucun compte à créer et aucune télémétrie.',
  'home.lines.line': 'Ligne',
  'home.lines.reaches': 'Dessert',
  'home.lines.needs': 'Ce qu’il lui faut sur le serveur',
  'home.lines.socket.name': 'Socket local',
  'home.lines.socket.reaches':
    'Docker Desktop, OrbStack, Colima, Rancher Desktop et les machines Podman de cet ordinateur, trouvés pour vous.',
  'home.lines.socket.needs': 'Rien. Le moteur est déjà sur cette machine.',
  'home.lines.ssh.reaches':
    'Docker Engine, Podman, containerd et Incus sur un serveur, en passant par des bastions si besoin.',
  'home.lines.ssh.needs':
    'Le port que vous avez déjà ouvert. Avec la clé que crée Homeport, une ligne dans {file}.',
  'home.lines.tls.reaches':
    'L’API d’un moteur compatible Docker, avec votre CA et votre certificat client.',
  'home.lines.tls.needs':
    'Le port de l’API du moteur (en général {port}), que vous exposez vous-même.',
  'home.lines.tcp.reaches': 'L’API d’un moteur compatible Docker, sans chiffrement.',
  'home.lines.tcp.needs':
    'Le port de l’API du moteur (en général {port}). Uniquement sur un réseau de confiance.',
  'home.lines.cluster.name': 'API du cluster',
  'home.lines.cluster.reaches':
    'Kubernetes depuis votre kubeconfig, Nomad par l’adresse de son agent, Incus via HTTPS avec un jeton de confiance.',
  'home.lines.cluster.needs': 'L’API que le cluster sert déjà. Rien n’est installé sur aucun nœud.',
  'home.lines.advice':
    'Quand vous le pouvez, utilisez SSH : il n’ajoute rien au serveur que vous n’exécutiez déjà.',
  'home.lines.import':
    'Vous avez déjà décrit vos serveurs ? Importez-les depuis {file}, bastions compris, ou depuis vos contextes Docker.',
  'home.lines.link': 'Comment fonctionnent les connexions',

  // Home: the Engines, as the map's index of stations.
  'home.engines.title': 'Chaque moteur, avec les mêmes mots.',
  'home.engines.lead':
    'Un hôte est un hôte et une stack est une stack, quel que soit le moteur qui la fait tourner. Quand un moteur a quelque chose qui lui est propre, Homeport le montre aussi.',
  'home.engines.docker.body':
    'Conteneurs, images, volumes, réseaux et stacks Compose : gardées dans Homeport, liées à un dossier ou synchronisées depuis un dépôt git.',
  'home.engines.swarm.body':
    'Nœuds, services, tâches, secrets et configs, trouvés via ses hôtes managers. Déployez des stacks sur le Swarm.',
  'home.engines.podman.body':
    'Les pods, et les unités Quadlet installées et démarrées sous systemd, via l’API de Podman lui-même.',
  'home.engines.containerd.body':
    'Espaces de noms, tâches, journaux, un terminal et les instantanés qui remplissent votre disque, sans Docker.',
  'home.engines.incus.body':
    'Conteneurs système et machines virtuelles : leur console, leurs instantanés, images, profils, stockages et réseaux.',
  'home.engines.kubernetes.body':
    'Charges de travail, Pods, Services et toute ressource de l’API, en direct. Journaux, terminaux, redirections de port, YAML avec un aperçu, releases Helm.',
  'home.engines.nomad.body':
    'Des jobs exécutés depuis du HCL avec leur plan, leurs allocations et leurs journaux, un shell dans une tâche, et le drainage des nœuds.',

  // Home: security.
  'home.security.title': 'Vos clés restent scellées.',
  'home.security.lead':
    'Homeport détient les identifiants qui ouvrent vos serveurs, et il les traite en conséquence.',
  'home.security.rest.title': 'Chiffré au repos',
  'home.security.rest.body':
    'Les clés SSH, certificats TLS, environnements de stacks et jetons de dépôts sont scellés avec XChaCha20-Poly1305. La clé est conservée dans le trousseau du système, pas à côté des données.',
  'home.security.disk.title': 'Jamais en clair sur le disque',
  'home.security.disk.body':
    'Homeport parle SSH lui-même, donc les clés déchiffrées restent en mémoire. Aucun fichier de clé, aucune modification de votre {dir}.',
  'home.security.home.title': 'Aucune remontée de données',
  'home.security.home.body':
    'Aucun compte, aucune télémétrie, aucun pistage. Chaque version est buildée publiquement, avec des sommes de contrôle et la provenance du build.',
  'home.security.link': 'La sécurité et vos données',

  // Home: the close, where the visitor starts.
  'home.close.title': 'Partez de cette machine.',
  'home.close.body':
    'Téléchargez Homeport sur l’ordinateur depuis lequel vous travaillez, puis ajoutez vos hôtes. Il est gratuit, open source sous licence GNU AGPL, et développé publiquement sur GitHub.',
  'home.close.github': 'Voir sur GitHub',
  'home.desktops.windows.versions': '10 et 11, x64 et ARM64',
  'home.desktops.windows.formats': 'Installeur (.exe) ou MSI',
  'home.desktops.macos.versions': 'Apple silicon et Intel',
  'home.desktops.macos.formats': 'Image disque (.dmg)',
  'home.desktops.linux.versions': 'x64 et ARM64',
  'home.desktops.linux.formats': 'apt, dnf, zypper ou AppImage',
};
