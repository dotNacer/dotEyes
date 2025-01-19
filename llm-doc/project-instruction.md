# Projet : Application d’Enregistrement d’Écran avec Effets de Caméra

## 🔨 **Stack Technique**

### 🔌 **Frontend**

-   **Framework** : [Svelte 5](https://svelte.dev/) (dernière version pour une réactivité accrue et une syntaxe moderne).
-   **Styling** : [TailwindCSS](https://tailwindcss.com/) pour des styles modulaires et réactifs.
-   **Gestion d’état** : [Svelte Store](https://svelte.dev/docs#run-time-svelte-store) pour un partage fluide de l’état entre composants.

### 🔧 **Backend**

-   **Base** : [Rust](https://www.rust-lang.org/) pour sa sécurité, sa performance, et son intégration native avec Tauri.
-   **Librairies** :
    -   **FFmpeg** : pour l’enregistrement et la manipulation vidéo.
    -   **X11/xlib** (Linux) : pour la capture d’écran et le suivi de la souris.
    -   **Serde** : pour la sérialisation/désérialisation des données.

### 🔄 **Framework Desktop**

-   **Tauri** (v2) : pour créer une application multiplateforme légère avec un backend Rust et un frontend basé sur Svelte.

### 🔗 **Communication Interne**

-   **IPC** : API de communication entre le frontend (Svelte) et le backend (Rust) via Tauri.
-   **WebSockets** : pour la synchronisation temps réel des données (par exemple : position de la souris).

### 🏠 **Structure des Fichiers**

```plaintext
.
├── src/
│   ├── backend/             # Code Rust pour l’interaction système
│   │   ├── commands.rs      # Commandes appelées par le frontend via IPC
│   │   ├── capture.rs       # Fonctionnalités d’enregistrement d’écran
│   │   ├── effects.rs       # Gestion des effets vidéo (zoom, etc.)
│   │   └── utils.rs         # Fonctions utilitaires génériques
│   ├── frontend/            # Application Svelte
│   │   ├── components/      # Composants Svelte réutilisables
│   │   ├── pages/           # Pages principales (Accueil, Paramètres, Aperçu)
│   │   ├── stores/          # Gestion des états globaux
│   │   └── App.svelte       # Point d’entrée de l’application
│   └── styles/              # Fichiers CSS (via Tailwind)
├── public/                  # Fichiers statiques
├── tauri.conf.json          # Configuration de Tauri
├── Cargo.toml               # Dépendances Rust
└── package.json             # Dépendances frontend
```

---

## 🎯 **Conventions de Code**

### ✍️ **Frontend**

-   **Nom des fichiers** : PascalCase pour les composants Svelte (ex. `MainMenu.svelte`).
-   **Fonctions** : camelCase (ex. `handleMouseZoom`).
-   **Store Management** :
    -   Utiliser des **writable stores** pour les états dynamiques (ex. position de la souris).
    -   Créer des **custom stores** pour encapsuler la logique complexe.
-   **Styles** :
    -   Priorité à TailwindCSS.
    -   Création de classes utilitaires dans `styles/` si nécessaire.

### 🔧 **Backend**

-   **Modules Rust** : chaque fonctionnalité majeure a son propre module (ex. `capture.rs`, `effects.rs`).
-   **Fonctions** : snake_case pour les noms (ex. `start_screen_recording`).
-   **Tests** :

    -   Ajouter des tests unitaires dans un sous-module `tests`.
    -   Exemple :

        ```rust
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_zoom_effect() {
                assert_eq!(apply_zoom(2.0), expected_output);
            }
        }
        ```

---

## 🚀 **Principes d’Architecture**

1. **Modularité**

    - Chaque module ou composant doit avoir une responsabilité unique (principe SRP).
    - Les interfaces (fonctions exposées) doivent être simples et précises.

2. **Interopérabilité**

    - Les commandes Rust accessibles au frontend via IPC doivent être clairement documentées.
    - Exemple de commande dans `commands.rs` :
        ```rust
        #[tauri::command]
        fn start_screen_recording() -> Result<(), String> {
            // Implémentation...
        }
        ```

3. **Extensibilité**

    - Ajouter de nouvelles fonctionnalités (ex. un nouvel effet) doit nécessiter un minimum de changements dans le code existant.
    - Utiliser des structures Rust énumérées (« Enums ») pour définir les types d’effets :
        ```rust
        enum EffectType {
            Zoom,
            Pan,
        }
        ```

4. **Tests**

    - Tests unitaires pour le backend (Rust).
    - Tests de bout en bout (E2E) pour le frontend avec [Playwright](https://playwright.dev/).

5. **Performances**
    - Utiliser des Web Workers (ou Tauri Tasks) pour les opérations intensives en frontend.
    - Optimiser les appels à `ffmpeg` avec des préréglages performants.

---

## 📈 **Flux Fonctionnel**

1. **Démarrage de l’Application**

    - Le frontend charge la page principale.
    - Le backend initialise les modules nécessaires (état de l’enregistrement, suivi de la souris).

2. **Enregistrement de l’Écran**

    - L’utilisateur démarre l’enregistrement via l’interface.
    - Une commande IPC est envoyée pour démarrer la capture avec `ffmpeg`.

3. **Effets Temps Réel**

    - Le backend surveille les positions de la souris.
    - Les coordonnées sont transmises au frontend via WebSocket pour un rendu interactif.

4. **Traitement Vidéo**
    - Une fois l’enregistrement terminé, les effets sont appliqués via un pipeline Rust + FFmpeg.

---

## 🔄 **Plan de Réalisation**

### 🔁 **Phase 1 : Initialisation**

1. Configurer le projet Tauri avec Svelte 5.
2. Configurer les dépendances essentielles (FFmpeg, TailwindCSS, librairies Rust).
3. Créer la structure initiale des dossiers (backend et frontend).

### 🌀 **Phase 2 : Fonctionnalités de Base**

1. Implémenter l’interface utilisateur basique avec les boutons "Démarrer" et "Arrêter".
2. Implémenter la capture d’écran basique via `ffmpeg`.
3. Créer une commande IPC Rust pour démarrer et arrêter l’enregistrement.
4. Stocker temporairement la vidéo dans un fichier local.

### 🔄 **Phase 3 : Suivi et Effets**

1. Implémenter le suivi de la souris en temps réel (backend Rust).
2. Ajouter un store Svelte pour afficher les coordonnées de la souris en live.
3. Créer une logique de zoom (par exemple, zoom sur la position de la souris).
4. Transmettre les coordonnées de zoom au backend pour le traitement vidéo.

### ⚙️ **Phase 4 : Interface Avancée**

1. Ajouter une page de paramètres (choix de la résolution, FPS, chemin de sortie).
2. Implémenter un aperçu vidéo en direct (facultatif).
3. Gérer les erreurs et afficher des messages utilisateur (ex. "Capture impossible").

### 🔀 **Phase 5 : Optimisation**

1. Optimiser les performances des appels FFmpeg.
2. Implémenter une file d’attente pour les effets si plusieurs sont ajoutés simultanément.
3. Ajouter des tests unitaires et E2E.

### 🎮 **Phase 6 : Finalisation et Livraison**

1. Emballer l’application pour Linux, Windows, et MacOS.
2. Tester le comportement sur plusieurs environnements.
3. Créer une documentation utilisateur simple.

---

## 📊 **Références Utiles**

-   [Documentation Svelte](https://svelte.dev/docs)
-   [Guide Tauri](https://tauri.app/v2/guides/)
-   [FFmpeg Filters](https://ffmpeg.org/ffmpeg-filters.html)
-   [Rust Book](https://doc.rust-lang.org/book/)

---
