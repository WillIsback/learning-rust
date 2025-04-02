# Projet d'Apprentissage Rust - Gestion Utilisateurs CLI

Bienvenue dans mon projet d'apprentissage du langage Rust ! Ce projet est une application CLI (Command Line Interface) pour gérer des utilisateurs. Il utilise des bibliothèques populaires comme `clap` pour la gestion des arguments en ligne de commande et `serde` pour la sérialisation/désérialisation des données.

## Objectifs du Projet

- Découvrir les bases de Rust : structures, fonctions, gestion des erreurs, etc.
- Comprendre et utiliser des bibliothèques externes comme `serde` et `clap`.
- Apprendre à manipuler des fichiers JSON pour stocker des données.
- Explorer les concepts avancés comme les closures, les itérateurs, et les traits.

---

## Fonctionnalités

1. **Créer un utilisateur** : Ajouter un nouvel utilisateur avec des informations comme prénom, nom, email, etc.
2. **Modifier un utilisateur** : Mettre à jour les informations d'un utilisateur existant.
3. **Supprimer un utilisateur** : Retirer un utilisateur de la liste.
4. **Lister les utilisateurs** : Afficher tous les utilisateurs enregistrés.

---

## Concepts Appris

### 1. **Parser**
Un parser est un outil qui analyse des données (comme des arguments CLI ou des fichiers JSON) pour les convertir en une structure utilisable dans le programme. Dans ce projet, j'ai utilisé `clap` pour parser les arguments en ligne de commande.

### 2. **Sérialisation et Désérialisation**
Avec la bibliothèque `serde`, j'ai appris à :
- **Sérialiser** : Convertir une structure Rust en JSON pour l'enregistrer dans un fichier.
- **Désérialiser** : Lire un fichier JSON et le convertir en une structure Rust.

### 3. **Gestion des Erreurs avec `Result`**
Rust utilise le type `Result<T, E>` pour gérer les erreurs. J'ai appris à utiliser `Ok()` pour indiquer une réussite et `Err()` pour signaler une erreur.

### 4. **Attributs Rust**
Les attributs comme `#[derive(Serialize, Deserialize)]` permettent de générer automatiquement du code pour des traits spécifiques.

### 5. **L'Opérateur `?`**
Le `?` est un raccourci pour propager les erreurs sans écrire de longues structures `match`.

### 6. **Closures et Itérateurs**
Les closures (définies avec `| |`) sont des fonctions anonymes utilisées dans des méthodes comme `iter().find()` ou `iter_mut().position()`.

---

## Commandes Disponibles

Voici les commandes que j'ai implémentées dans ce projet :

### 1. **Créer un utilisateur**
```bash
cargo run -- generate-user --firstname John --lastname Doe --email john.doe@example.com --burge Paris --si 12345
```

### 2. **Modifier un utilisateur**
```bash
cargo run -- edit-user --firstname John --lastname Doe --email john.new@example.com --burge Lyon --si 67890 --login johndoe --password newpass
```

### 3. **Supprimer un utilisateur**
```bash
cargo run -- delete-user --login johndoe
```

### 4. **Lister les utilisateurs**
```bash
cargo run -- list-users
```

---

## Ressources Utilisées

- [Documentation officielle de Rust](https://doc.rust-lang.org/)
- [Documentation de clap](https://docs.rs/clap/)
- [Documentation de serde](https://serde.rs/)
- [Crates.io](https://crates.io/) pour explorer les bibliothèques Rust.

---

## Journal de Bord

### Jour 1
- Découverte de Rust et installation de l'environnement.
- Création d'un projet avec `cargo new`.

### Jour 2
- Apprentissage des bases : variables, fonctions, et gestion des erreurs.
- Implémentation de la commande `generate-user`.

### Jour 3
- Exploration des bibliothèques `serde` et `clap`.
- Ajout des commandes `edit-user` et `delete-user`.

### Jour 4
- Gestion des fichiers JSON pour stocker les utilisateurs.
- Utilisation des itérateurs et des closures.

---

## Conclusion

Ce projet m'a permis de découvrir Rust et de comprendre ses concepts fondamentaux. Bien que le langage soit exigeant, il offre une grande sécurité et des performances impressionnantes. Je continuerai à explorer Rust avec des projets plus complexes à l'avenir !
