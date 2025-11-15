#  Jeu d'Aventure en Rust

Un jeu d'action 2D développé en Rust avec la bibliothèque ggez.

##  Aperçu

Jeu d'aventure où vous devez collecter des pièces tout en combattant des ennemis !

##  Objectif

Collectez **20 pièces** pour gagner tout en survivant aux attaques des ennemis.

##  Contrôles

- **ZQSD** ou **Flèches directionnelles** : Déplacement
- **ESPACE** : Attaquer
- **R** : Redémarrer (après Game Over/Victoire)

##  Ennemis

-  **Goblin** : Faible mais rapide (5 dégâts)
-  **Orc** : Moyen (10 dégâts)
-  **Dragon** : Fort mais lent (20 dégâts)

## ��� Items

-  **Pièce** : +1 pièce (objectif: 20)
-  **Potion** : +25 PV
-  **Arme** : Améliore les dégâts d'attaque

##  Technologies

- **Rust** 
- **ggez** (moteur de jeu 2D)
- **rand** (génération aléatoire)

##  Installation et Exécution

### Prérequis

- Rust installé ([rustup.rs](https://rustup.rs/))

### Linux/WSL
```bash
# Cloner le projet
git clone https://github.com/NOURELHOUDAFARHAT10/jeu_complet.git
cd jeu_complet

# Compiler et lancer
cargo run --release
```

### Windows
```powershell
# Cloner le projet
git clone https://github.com/NOURELHOUDAFARHAT10/jeu_complet.git
cd jeu_complet

# Compiler et lancer
cargo run --release
```

##  Structure du Projet
```
jeu_complet/
├── src/
│   ├── main.rs          # Point d'entrée
│   ├── game.rs          # Logique du jeu
│   ├── player.rs        # Joueur
│   ├── enemy.rs         # Ennemis
│   ├── items.rs         # Items collectables
│   ├── map.rs           # Carte et obstacles
│   └── ui.rs            # Interface utilisateur
├── resources/           # Sprites et assets
│   ├── player.png
│   ├── goblin.png
│   ├── orc.png
│   ├── dragon.png
│   ├── coin.png
│   ├── potion.png
│   └── weapon.png
├── Cargo.toml
└── README.md
```

##  Note WSL

Si vous utilisez WSL, l'audio est désactivé par défaut. Pour afficher l'interface graphique :

### Windows 11 (WSLg intégré)
```bash
wsl --update
```

### Windows 10 (VcXsrv requis)
1. Installez [VcXsrv](https://sourceforge.net/projects/vcxsrv/)
2. Lancez XLaunch
3. Dans WSL :
```bash
export DISPLAY=$(cat /etc/resolv.conf | grep nameserver | awk '{print $2}'):0
```

##  Fonctionnalités

- Déplacement fluide dans 4 directions
-  Système de combat
-  Collecte d'items
-  Spawn dynamique d'ennemis
-  Système de vie et progression
-  Écrans de victoire/défaite
-  Détection de collisions
-  Interface utilisateur

##  Améliorations Futures

- [ ] Plus de types d'ennemis
- [ ] Niveaux multiples
- [ ] Boss de fin
- [ ] Système de score
- [ ] Sons et musique
- [ ] Animations plus fluides
- [ ] Sauvegarde de progression

## Auteur:

**Nour El Houda Farhat**
- GitHub: [@NOURELHOUDAFARHAT10](https://github.com/NOURELHOUDAFARHAT10)

## ��� Licence

Ce projet est sous licence MIT.

## ��� Remerciements

- Bibliothèque [ggez](https://github.com/ggez/ggez)
- Communauté Rust ���
