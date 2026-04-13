# rust-gestionnaire-telephonique

## Objectif
Gestionnaire de numeros de telephone en Rust base sur un trie (prefix tree).

## Structure
- `src/main.rs` : point d'entree
- `src/lib.rs` : logique principale
- `src/config.rs` : configuration CLI
- `src/models.rs` : structures JSON
- `src/trie.rs` : trie from scratch
- `src/plantuml.rs` : generation PlantUML
- `src/error.rs` : gestion des erreurs
- `data/` : fichiers de test JSON
- `graph/` : fichiers `.puml`
- `tests/` : tests d'integration

## Lancer le projet

Exemple de lancement :
```bash
cargo run --release
#TODO: Ajouter veritable commande de lancement (OBLIGATOIRE)
```

## Commandes utiles
### Compilation rapide
```bash
cargo check
```
Verifie rapidement que le projet compile, sans produire le binaire final.

### Analyse du code
```bash
cargo clippy -- -D warnings
```
Lance les verifications de style et de qualite avec Clippy, et echoue si un warning est detecte.

### Tests
```bash
cargo test
```
Lance les tests unitaires et les tests d'integration du projet.

## Donnees de test
Les fichiers presents dans le dossier `data/` sont fournis pour les tests
et doivent etre utilises sans modification.

## Visualisation PlantUML
Le programme genere un fichier `.puml`.
La visualisation du resultat se fait ensuite manuellement avec PlantUML.

## Ressources
Ce projet doit s'appuyer sur les bonnes pratiques suivantes :

- Rust Book : organisation d’un projet  
  https://doc.rust-lang.org/book/ch12-00-an-io-project.html

- Conventional Commits : standardisation des messages Git  
  https://www.conventionalcommits.org/en/v1.0.0/