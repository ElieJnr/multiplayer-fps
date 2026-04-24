# Multiplayer FPS (Gatsa-Gatsa)

Petit jeu de tir à la première personne multijoueur en **Rust**, moteur **Bevy 0.14**, réseau **UDP** et messages **JSON** (serde). Un même binaire sert de **serveur** (console) ou de **client** (lancement du jeu graphique après connexion).

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021)
- **`wget`** (utilisé au démarrage serveur pour télécharger les modèles 3D `.glb` s’ils sont absents)
- Carte graphique et pilotes compatibles **Vulkan / Metal / DX12** (pile graphique de Bevy)

## Lancer le projet

```bash
cargo run
```

Un menu texte s’affiche dans le terminal :

1. **Start as Server** — démarre le serveur UDP
2. **Start as Client** — se connecte à un serveur puis ouvre la fenêtre de jeu

### Héberger une partie (serveur)

1. Choisir **1**
2. Indiquer le **nombre de joueurs** attendu (minimum **2** ; une valeur invalide retombe sur ce minimum)
3. Noter l’adresse affichée (`IP:8080`). Le serveur écoute sur le port défini dans `src/common/constant.rs` (**8080**)
4. Au premier lancement, les fichiers `assets/player.glb` et `assets/enemy.glb` sont téléchargés depuis les URLs du dépôt référencé dans le code si besoin

### Rejoindre une partie (client)

1. Choisir **2**
2. Saisir l’**adresse du serveur** (ex. `192.168.1.10:8080`) puis un **nom de joueur** (unique sur la partie)
3. Attendre que la salle d’attente se remplisse ; quand la partie démarre, la fenêtre Bevy s’ouvre (résolution typique **1200×1000**)

## Contrôles (client)

Les entrées réelles dans le jeu (voir `src/player/movement.rs`) :

| Action | Entrée |
|--------|--------|
| Déplacement | **W A S D** |
| Visée / orientation souris | **Clic droit** maintenu (associé au champ `r_key` côté protocole) |
| Tir | **Espace** |
| Autre action souris | **Clic gauche** (champ `space` dans `PlayerInput`) |

La souris est utilisée pour la caméra ; les mises à jour de position et d’actions sont envoyées au serveur en UDP.

## Architecture du dépôt

| Zone | Rôle |
|------|------|
| `src/main.rs` | Point d’entrée : lance le menu réseau (`run_socket`) |
| `src/server/` | Boucle serveur UDP, diffusion des messages, logique de connexion |
| `src/client/` | Connexion client, échange jusqu’au démarrage du jeu |
| `src/common/` | Port **8080**, santé, protocole `GameMessage` / `MessageType`, utilitaires réseau |
| `src/graphics/` | Application Bevy : menu, labyrinthe, minimap, sons, états de jeu |
| `src/maze/` | Génération / affichage du labyrinthe, collisions, UI (barre de vie, etc.) |
| `src/player/` | Joueur local, animations, mouvement, tir |
| `assets/` | Textures, polices ; modèles `.glb` une fois téléchargés |

## Dépendances principales

Définies dans `Cargo.toml` : **bevy**, **bevy_tweening**, **serde** / **serde_json**, **colored**, **lazy_static**, **rand**.

## Compilation release (optionnel)

```bash
cargo build --release
```

Le binaire se trouve dans `target/release/multiplayer_fps`.

## Limites connues

- Le serveur est **synchrone** dans le terminal (pas de binaire client/serveur séparés) : chaque machine lance le même programme avec un choix différent.
- Après le début de partie, de **nouvelles connexions** sont refusées côté serveur.
- La fiabilité et l’ordre des paquets **UDP** ne sont pas garantis par le protocole ; le jeu repose sur des mises à jour fréquentes et une numérotation de séquence côté messages.

---

Projet expérimental orienté apprentissage / démo multijoueur avec Bevy.
