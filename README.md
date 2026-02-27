# Bassin Laboratoire SEACom - ISEN

Ce projet vise à remettre le bassin de test du laboratoire SEACom de l'ISEN au goût du jour. Pour cela, une interface Web est déployée sur un serveur Ethernet hébergé sur une carte Beaglebone Black qui communique avec le bassin via un shield customisé.

## TODO

### Partie Logicielle

- écrire une suite de tests exhaustive qui permet de comprendre ce qui doit être envoyé par le backend en fonction des commandes exécutées
- définir les trames de messages dans la documentation
- compléter la description de cette documentation (à partir de "Redémarrage des commandes numériques (RESET)")

### Partie Matérielle

- Imprimer ou découper à la CNC le couvercle du boitier de la carte
- Ajouter la vitre plexi devant les LED
- Produire la nouvelle version de la carte électronique
- acheter les différents composant manquant pour autonomiser le boitier
	- [prise filtre 230V](https://fr.rs-online.com/web/p/filtres-iec/1676800?searchId=2fb21e5c-5856-4a20-a56e-df45db2b0a76) (ça sera serré mais normalement ça passe)
    - [Alim 5V](https://fr.rs-online.com/web/p/alimentations-a-decoupage/0413673?searchId=7ffa1f5d-4cfa-4edf-9e52-69dc6d7725a8)
    - connecteur DB9 de facade

## Installation

### Configuration de la carte Beaglebone

Premièrement, afin d'avoir accès à tous les ports GPIO de la carte, celle-ci doit être démarrée à partir d'une carte SD au lieu de sa mémoire interne. Pour ce faire, téléchargez [la dernière image de Debian pour Beaglenone Black](https://www.beagleboard.org/distros) et flashez-la sur votre carte SD. Faites attention de bien choisir une image et non pas un flasher eMMC (le nom devrait finir par `4GB microSD IoT`).

Puisque les dernières versions ont un serveur nginx préinstallé, c'est ce que ce projet va utiliser.

Afin de vous connecter au Beaglebone, connecter le câble ethernet à votre pc et configurer votre carte réseau pour avoir une IPv4 telle que 192.168.7.4, via la commande `sudo ifconfig enp3s0 192.168.7.4` (en remplaçant enp3s0 par le nom de votre interface réseau).

Une fois la carte SD insérée dans la Beaglebone et celle-ci branchée sur le même réseau que votre ordinateur, vous pourrez ouvrir une console `ssh` grâce à la commande suivante :

```sh
ssh debian@192.168.7.3
```

Le mot de passe par défaut est `temppwd`.

Afin de configurer nginx, ouvrez le fichier `/etc/nginx/sites-enabled/default` et remplacez la ligne `root /var/www` par `root /home/debian/dist`. Enregistrez le fichier.

Finalement, il faut ouvrir le port 3333 de la Beaglebone pour permettre la connection TCP entre l'interface et le backend :

```sh
sudo nft add table ip filter
sudo nft add chain ip filter INPUT { type filter hook input priority 0 \; }
sudo nft add rule ip filter INPUT tcp dport 3333 accept
sudo nft list ruleset # pour vérifier que les règles ont bien été appliquées
sudo nft list ruleset | sudo tee -a /etc/nftables.conf # pour les sauvegarder
```

Vous pouvez maintenant quitter la console `ssh` avec la commande :

```sh
exit
```

### Compilation du projet

La compilation nécessite d'avoir podman, podman-compose et git d'installés sur le pc.
Clonez et compilez le projet GitHub sur votre ordinateur avec les commandes suivantes :

```sh
git clone https://github.com/Kats29/bassin_seacom.git
cd bassin_seacom
podman build .
podman compose run --rm arm-builder
```

Une fois que le projet est bien compilé (penser à vérifier les logs pour cela) vous pouvez envoyer la release sur la Beaglebone via le script `./.send.sh`.

Tapez le mot de passe de la Beaglebone lorsqu'il vous est demandé.

Vous pouvez à présent rouvrir une console `ssh` pour démarrer le backend :

```sh
ssh debian@192.168.7.3
```

Tapez le mot de passe.

```sh
sudo systemctl restart backend.service
```

Si tout va bien, vous devez avoir quelque chose qui ressemble à ci-dessous en tapant la commande  `sudo systemctl status backend.service` :
```sh
● backend.service - Backend du bassin
     Loaded: loaded (/etc/systemd/system/backend.service; enabled; preset: enabled)
     Active: active (running) since Fri 2025-03-14 21:15:03 CET; 5s ago
   Main PID: 2041 (backend)
      Tasks: 1 (limit: 1024)
     Memory: 72.0K
        CPU: 61ms
     CGroup: /system.slice/backend.service
             └─2041 /home/debian/backend

Mar 14 21:15:03 bassin systemd[1]: Started backend.service - Backend du bassin.
Mar 14 21:15:03 bassin backend[2041]: Starting backend...
```

Si vous souhaitez éditer la configuration du service, elle se trouve ici `/etc/systemd/system/backend.service` et nécessite un redémarrage de systemctl via `sudo systemctl daemon-reload`.

### Accès à l'interface

Vous pouvez à présent vous connecter à l'interface depuis votre navigateur internet, en tapant `192.168.7.3:9090` dans la barre d'adresse.

## Branchement

### Installation de la Beaglebone

### Raccordement réseau

### Communication avec le bassin

## Documentation

Une fois le projet cloné, la documentation technique du code peut être générée grâce à la commande :

```sh
make doc
```

La documentation devrait s'ouvrir dans un navigateur. Sinon, elle peut être visualisée dans un navigateur en ouvrant les fichiers `index.html` générés dans `target/doc/common`, `target/armv7-unknown-linux-musleabihf/doc/backend` et `target/wasm32-unknown-unknown/doc/frontend`

## Utilisation

![Interface de contrôle](interface.gif)

### Apparence et organisation

L'interface est constituée d'une seule vue, qui est séparée en trois zones : une vue centrale, et deux panneaux latéraux. En cas de nécéssité, une fenêtre d'erreur peut également s'ouvrir devant la vue centrale.

#### Vue centrale

La vue centrale représente une visualisation de l'état actuel du bassin, ainsi que des commandes prévues.

Sa partie supérieure montre une vue du dessus en séparant les moitiés droite et gauche du bassin. La même séparation est visible sur la partie inférieure, qui elle montre une vue de côté du bassin (pour visualiser la profondeur).

Les cadres noirs délimitent les zones où chacun des deux bras peut se déplacer, et les symboles <img src="frontend/assets/emitter.png" width="20"> en noir symbolisent la position actuelle de chaque bras. Les symboles <img src="frontend/assets/emitter.png" width="20"> grisés montrent le prochain mouvement pouvant être ajouté.

#### Panneau gauche

Le panneau latéral gauche montre deux choses :

* Les coordonnées du prochain mouvement à ajouter pour chaque bras.

* L'état actuel du bassin, sous formes d'indicateurs colorés (rouges ou verts, gris si le bassin n'est pas connecté).
	* En face de chaque coordonnée, un indicateur passe au rouge si un mouvement est en cours sur cet axe.
	* Sous les coordonnées, d'autres informations sont données sous cette forme.

#### Panneau droit

Le panneau latéral droit montre simplement la liste des mouvement prévus pour chaque bras, dans l'ordre chronologique.

#### Fenêtre d'erreur

### Commandes d'alimentation

La première chose à faire pour allumer le bassin est de le mettre physiquement sous tension. Une fois que c'est fait, l'alimentation peut être contrôlée à distance par l'interface.

#### Allumer le bassin

Pour allumer le bassin, allez dans le menu `Alimentation` et cliquez sur `Démarrer`. Après un démarrage de cette façon, une commande de RESET (expliquée plus loin) et de remise à l'origine doivent obligatoirement être effectuées dans cet ordre.

Après appui, les indicateurs `Bassin alimenté` et `Bassin démarré` devraient être au vert.

#### Éteindre le bassin

Une fois le bassin sous tension et démarré, le menu `Alimentation` devrait contenir un bouton `Arrêter`, qui doit éteindre le bassin.

Après appui, l'indicateur `Bassin démarré` devrait repasser au rouge.

#### Redémarrage des commandes numériques (RESET)

### Gestion des mouvements

#### Ajouter un mouvement

#### Supprimer un mouvement

#### Modifier un mouvement


### Effectuer un mouvement

#### Commandes générales

#### Commandes par bras

#### Commandes par axe

### Sauvegarde et importation de positions

#### Sauvegarder la configuration

#### Importer une configuration
