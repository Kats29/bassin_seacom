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

## Utilisation

![Interface de contrôle](interface.gif)

### Apparence et organisation

L'interface est constituée d'une seule vue, qui est séparée en trois zones : une vue centrale, et deux panneaux latéraux. En cas de nécéssité, une fenêtre d'erreur peut également s'ouvrir devant la vue centrale.

#### Vue centrale

La vue centrale représente une visualisation de l'état actuel du bassin, ainsi que des commandes prévues.

Sa partie supérieure montre une vue du dessus en séparant les moitiés droite et gauche du bassin. La même séparation est visible sur la partie inférieure, qui elle montre une vue de côté du bassin (pour visualiser la profondeur).

Les cadres noirs délimitent les zones où chacun des deux bras peut se déplacer, et les symboles ![émetteur](frontend/assets/emitter.png) en noir symbolysent la position actuelle de chaque bras.

#### Panneau gauche

#### Panneau droit

#### Fenêtre d'erreur

### Commandes d'alimentation

#### Allumer le bassin

#### Éteindre le bassin

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
