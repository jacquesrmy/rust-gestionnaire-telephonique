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

### Exemple simple
```bash
cargo run --release -- data/01_simple.json
```

### Plusieurs racines
```bash
cargo run --release -- data/02_different_roots.json
```

### Cas imbriqué
```bash
cargo run --release -- data/03_one_in_another.json
```

### Cas complet
```bash
cargo run --release -- data/04_common_parts.json
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

## Makefile
Le projet fournit un `Makefile` pour simplifier les commandes.

### Verification complete locale
```bash
make
```
ou
```bash
make ci
```

### Formatage du code
```bash
make fmt
```

### Verification rapide de compilation
```bash
make check
```

### Analyse avec Clippy
```bash
make clippy
```

### Lancement des tests
```bash
make test
```

## CI
Le depot utilise GitHub Actions pour verifier automatiquement :
- le formatage du code
- la compilation
- les checks Clippy
- les tests

Les modifications doivent passer par une pull request avant fusion dans main.

## Workflow Git recommande
Avant de commencer une modification :
```bash
git checkout main
git pull
git checkout -b type/description-courte
```

Une fois les modifications terminees :

```bash
make fmt
make ci
git add .
git commit -m "type: description concise"
git push -u origin nom-de-branche
```

Ensuite :
- ouvrir une pull request sur github
- attendre la CI
- merger si tout est valide

## Donnees de test
Les fichiers presents dans le dossier `data/` sont fournis pour les tests et ne doivent pas etre modifie.

## Visualisation PlantUML
Le programme genere un fichier `.puml`.
La visualisation du resultat se fait ensuite manuellement avec PlantUML.

## Ressources
Ce projet doit s'appuyer sur les bonnes pratiques suivantes :

- Rust Book : organisation d’un projet  
  https://doc.rust-lang.org/book/ch12-00-an-io-project.html

- Conventional Commits : standardisation des messages Git  
  https://www.conventionalcommits.org/en/v1.0.0/
