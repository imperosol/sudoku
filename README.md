Sudoku
======

Un résolveur de sudoku écrit en Rust.

Utilise un algorithme de résolution récursive en 
explorant les cellules dans l'ordre du nombre de 
solutions potentielles.

Pour faire tourner : 
```bash
cargo run --release <fichier contenant la grille>
```

Ou bien, si on dispose de l'exécutable :
```bash
sudoku <fichier contenant la grille>
```

Les fichiers contenant la grille sont des fichiers texte
avec 9 lignes de texte contenant chacune les valeurs 
correspondantes de la grille de sudoku. 
Les valeurs inconnues sont représentées au choix
par les caractères `*`, `.`, `?` ou `_`.
Les caractères peuvent indifférement être séparés par un espace ou non.


Par exemple, le contenu suivant est valide :
```text
9 . . 1 . . . . 5
. . 5 . 9 . 2 . 1
8 . . . 4 . . . 7
. . . . 8 . . . .
. . . 7 . . . . .
. . . . 2 6 . . 9
2 . . 3 . . . . 6
. . . 2 . . 9 . .
. . 1 9 . 4 5 7 .
```

Par défaut, le résultat est affiché sur la sortie standard,
mais on peut aussi l'écrire dans un fichier.
Pour ça, on appelle la commande de la manière suivante :

```bash
sudoku <fichier à résoudre> -o <fichier solution>
```
