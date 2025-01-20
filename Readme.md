## Multiplayer-FPS

```my_game_project/
│
├── src/
│   ├── server/  @Gaby           # Logiciel pour le serveur
│   │   ├── mod.rs               # Module principal du serveur
│   │   ├── udp.rs               # Logiciel UDP du serveur
│   │   ├── auth.rs              # Authentification et gestion des connexions
│   │   └── maze.rs              # Génération de labyrinthes
│   │
│   ├── client/    @Kendi        # Logiciel pour le client
│   │   ├── mod.rs               # Module principal du client
│   │   ├── udp.rs               # Communication UDP du client
│   │   ├── graphics.rs          # Interface graphique (SDL, SFML)
│   │   ├── movement.rs          # Logique des mouvements
│   │   └── hud.rs               # Affichage de la mini-carte et FPS
│   │
│   ├── common/@Aliou/@Kendi/@Gaby # Code commun entre client et serveur
│   │   ├── mod.rs               # Module commun
│   │   ├── protocol.rs          # Définition des messages (JSON ou binaire)
│   │   ├── events.rs            # Gestion des événements
│   │   └── sync.rs              # Synchronisation des états
│   │
│   ├── ai/                      # Intelligence Artificielle
│   │   ├── mod.rs               # Module IA
│   │   ├── pathfinding.rs       # Algorithmes de déplacement IA
│   │   └── decision_making.rs   # Prise de décision IA
│   │
│   ├── editor/                  # Éditeur de labyrinthes
│   │   ├── mod.rs               # Module éditeur
│   │   ├── editor_gui.rs        # Interface graphique pour l'éditeur
│   │   └── file_io.rs           # Sauvegarde et chargement de labyrinthes
│   │
│   ├── performance/             # Tests et optimisations
│   │   ├── mod.rs               # Module performance
│   │   ├── load_testing.rs      # Tests de charge
│   │   └── stress_testing.rs    # Tests de stress
│   │
│   ├── utils/                   # Utilitaires
│   │   ├── mod.rs               # Module utilitaires
│   │   ├── logger.rs            # Log des actions
│   │   └── timer.rs             # Gestion du temps (FPS, délais)
│
├── assets/                      # Ressources du jeu (images, sons, etc.)
│   ├── maze_map.png             # Image du labyrinthe
│   ├── player_sprite.png        # Sprite du joueur
│   └── sound_effects/           # Dossier des effets sonores
│
├── Cargo.toml                   # Dépendances du projet
└── README.md                    # Documentation du projet

```
