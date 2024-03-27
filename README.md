# Bassin Laboratoire SEACom - ISEN

Ce projet vise à remettre le bassin de test du laboratoire SEACom de l'ISEN au goût du jour. Pour cela, une interface Web est déployée sur un serveur Ethernet hébergé sur une carte Beaglebone Black qui communique avec le bassin via un shield customisé.

## Installation

### Configuration de la carte Beaglebone

Premièrement, afin d'avoir accès à tous les ports GPIO de la carte, celle-ci doit être démarrée à partir d'une carte SD au lieu de sa mémoire interne. Pour ce faire, téléchargez [la dernière image de Debian pour Beaglenone Black](https://www.beagleboard.org/distros) et flashez-la sur votre carte SD. Faites attention de bien choisir une image et non pas un flasher eMMC (le nom devrait finir par `4GB microSD IoT`).

Puisque les dernières versions ont un serveur nginx préinstallé, c'est ce que ce projet va utiliser.

Une fois la carte SD insérée dans la Beaglebone et celle-ci branchée sur le même réseau que votre ordinateur, vous pourrez ouvrir une console `ssh` grâce à la commande suivante :

```sh
ssh debian@beaglebone.local
```

Le mot de passe par défaut est `temppwd`, pensez à le changer dès que possible.

Il faut maintenant changer le nom de domaine local pour qu'il soit `bassin.local`. Pour cela, éxecutez les commandes :

```sh
sudo echo bassin > /etc/hostname
sudo service avahi-daemon restart
```

À présent, pour vous connecter en `ssh`, il faudra taper :

```sh
ssh debian@bassin.local
```

Afin de configurer nginx, ouvrez le fichier `/etc/nginx/sites-enabled/default` et remplacez la ligne `root /var/www` par `root /home/debian/dist`. Enregistrez le fichier.

Finalement, il faut ouvrir le port 3333 de la Beaglebone pour permettre la connection TCP entre l'interface et le backend :

```sh
sudo iptables -A INPUT -p tcp --dport 3333 --jump ACCEPT
sudo iptables-save
```

Vous pouvez maintenant quitter la console `ssh` avec la commande :

```sh
exit
```

### Compilation du projet

Clonez et compilez le projet GitHub sur votre ordinateur avec les commandes suivantes, en vous assurant que vous êtes bien sur le même réseau que la Beaglebone :

```sh
git clone https://github.com/Mousakaa/bassin_seacom.git
cd bassin_seacom
make
```

Tapez le mot de passe de la Beaglebone lorsqu'il vous est demandé.

Vous pouvez à présent rouvrir une console `ssh` pour démarrer le backend :

```sh
ssh debian@bassin.local
```

Tapez le mot de passe.

```sh
sudo ./backend & exit
```

Tapez à nouveau le mot de passe.

### Accès à l'interface

Vous pouvez à présent vous connecter à l'interface depuis votre navigateur internet, en tapant `bassin.local` dans la barre d'adresse.

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
