// #![allow(dead_code)]

use crate::errors::ParseGridError;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid([[Option<u8>; 9]; 9]);

#[derive(Debug, Clone)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Grid {
    /// Renvoie un itérateur sur les coordonnées de la grille qui sont vides
    pub fn empty_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|&i| i.1.is_none())
                    .map(|i| i.0)
            })
            .enumerate()
            .filter(|(_, row)| row.clone().next().is_some())
            .flat_map(|(row, cols)| cols.map(move |col| Coord { row, col }))
    }

    #[inline(always)]
    pub fn associated_values<'a>(&'a self, coords: &'a Coord) -> impl Iterator<Item = u8> + 'a {
        let in_row = self.0.get(coords.row).unwrap().iter().filter_map(|i| *i);
        let in_col = self.iter().filter_map(|row| *row.get(coords.col).unwrap());
        let in_subsquare = self.get_subsquare(coords).flatten().filter_map(|i| *i);
        in_row.chain(in_col).chain(in_subsquare).unique()
    }

    /// Renvoie true si la grille est complète selon les règles du sudoku, sinon false
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        for (row_ind, row) in self.iter().enumerate() {
            for col_ind in 0..9 {
                let coords = Coord {
                    row: row_ind,
                    col: col_ind,
                };
                // On vérifie que la ligne contient exactement une fois les numéros de 1 à 9
                // et pas de 0
                let mut mask = 0b0;
                row.iter().for_each(|i| mask |= 1 << i.unwrap_or(0));
                if mask != 0b1111111110 {
                    return false;
                }
                // colonne
                mask = 0b0;
                self.0
                    .iter()
                    .for_each(|row| mask |= 1 << row[coords.col].unwrap_or(0));
                if mask != 0b1111111110 {
                    return false;
                }
                // sous-carré
                mask = 0b0;
                self.get_subsquare(&coords)
                    .flatten()
                    .for_each(|i| mask |= 1 << i.unwrap_or(0));
                if mask != 0b1111111110 {
                    return false;
                }
            }
        }
        true
    }

    /// Fonction récursive de résolution de la grille
    fn backtrack(
        &mut self,
        empty_coords: &mut (impl Iterator<Item = Coord> + Clone),
    ) -> Result<Self, ()> {
        let coord = empty_coords.next();
        if coord.is_none() {
            // On a fini d'explorer les cases vides, ce qui signifie qu'on a tout rempli
            // et qu'on a la solution
            return Ok(Grid(self.0));
        }
        let coord = coord.unwrap();
        for choice in self.cell_possible_choices(&coord).unwrap() {
            self.0[coord.row][coord.col] = Some(choice);
            match self.backtrack(&mut empty_coords.clone()) {
                Ok(solution) => {
                    // le niveau supérieur dans la récursion retourne une solution
                    return Ok(solution);
                }
                Err(_) => {
                    // Mauvais nombre, on tente le suivant
                    self.0[coord.row][coord.col] = None;
                }
            }
        }
        // Rien de trouvé, situation bloquante, il faut remonter et recommencer
        Err(())
    }

    /// Construit et renvoie la grille solution de cette grille.
    pub fn solution(&self) -> Self {
        let mut solution = self.clone();
        let mut empty_coords = self.empty_coords().sorted_by(|i, j| {
            self.nb_cell_possible_choices(i)
                .cmp(&self.nb_cell_possible_choices(j))
        });
        solution.backtrack(&mut empty_coords).unwrap()
    }

    #[inline(always)]
    pub fn get_cell(&self, coords: &Coord) -> Option<u8> {
        self.0[coords.row][coords.col]
    }

    /// Retourne un itérateur de trois éléments itérant sur trois slices de trois éléments
    /// représentant le sous-carré auquel appartient les coordonnées données en argument.
    ///
    pub fn get_subsquare<'a>(
        &'a self,
        coords: &'a Coord,
    ) -> impl Iterator<Item = &[Option<u8>]> + 'a {
        self.iter()
            .skip(coords.row / 3 * 3)
            .take(3)
            .map(|row| &row[coords.col / 3 * 3..coords.col / 3 * 3 + 3])
    }

    /// Si la cellule correspondant aux coordonnées données n'est pas vide, renvoie None.
    /// Sinon, renvoie un itérateur sur les solutions possibles pour cette case.
    #[inline(always)]
    fn cell_possible_choices(&self, coords: &Coord) -> Option<impl Iterator<Item = u8>> {
        if self.get_cell(coords).is_some() {
            return None;
        }
        let mut associated = self.associated_values(coords).sorted().peekable();
        Some((1..=9).filter(move |i| associated.next_if(|j| j == i).is_none()))
    }

    /// renvoie le nombre de solutions possibles pour la cellule correspondant
    /// aux coordonnées données. Si la cellule n'est pas vide, ce nombre vaut 0.
    #[inline(always)]
    fn nb_cell_possible_choices(&self, coords: &Coord) -> usize {
        if self.get_cell(coords).is_some() {
            return 0;
        }
        9 - self.associated_values(coords).count()
    }
}

impl Deref for Grid {
    type Target = [[Option<u8>; 9]; 9];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self
            .iter()
            .map(|row| {
                row.map(|i| match i {
                    None => '_',
                    Some(i) => char::from_digit(i as u32, 10).unwrap(),
                })
                .into_iter()
                .join(" ")
            })
            .join("\n");
        f.write_str(str.as_str())
    }
}

impl From<[[u8; 9]; 9]> for Grid {
    fn from(value: [[u8; 9]; 9]) -> Self {
        let mut res = [[None; 9]; 9];
        for (row_ind, row) in value.iter().enumerate() {
            for (col_ind, col) in row.iter().enumerate() {
                res[row_ind][col_ind] = match *col {
                    0 => None,
                    other => Some(other),
                }
            }
        }
        Grid(res)
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = [[None; 9]; 9];
        for (str_row, grid_row) in s.trim().lines().zip(grid.iter_mut()) {
            let couples = str_row
                .chars()
                .filter(|c| !c.is_whitespace())
                .zip(grid_row.iter_mut());
            for (value, grid_cell) in couples {
                match value.to_digit(10) {
                    Some(value) => *grid_cell = Some(value as u8),
                    None => {
                        if ['_', '.', '?', '*'].contains(&value) {
                            *grid_cell = None;
                        } else {
                            return Err(ParseGridError);
                        }
                    }
                }
            }
        }
        Ok(Grid(grid))
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

    /// Renvoie une grille non-résolue
    fn unresolved_grid() -> Grid {
        Grid::from([
            [9, 0, 0, 1, 0, 0, 0, 0, 5],
            [0, 0, 5, 0, 9, 0, 2, 0, 1],
            [8, 0, 0, 0, 4, 0, 0, 0, 7],
            [0, 0, 0, 0, 8, 0, 0, 0, 0],
            [0, 0, 0, 7, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 6, 0, 0, 9],
            [2, 0, 0, 3, 0, 0, 0, 0, 6],
            [0, 0, 0, 2, 0, 0, 9, 0, 0],
            [0, 0, 1, 9, 0, 4, 5, 7, 0],
        ])
    }

    /// Test que la méthode `Grid::is_valid` fonctionne bien.
    #[test]
    fn test_valid() {
        let valid = Grid::from([
            [9, 6, 7, 1, 3, 2, 4, 8, 5],
            [4, 3, 5, 8, 9, 7, 2, 6, 1],
            [8, 1, 2, 6, 4, 5, 3, 9, 7],
            [1, 2, 6, 4, 8, 9, 7, 5, 3],
            [5, 9, 8, 7, 1, 3, 6, 2, 4],
            [7, 4, 3, 5, 2, 6, 8, 1, 9],
            [2, 7, 9, 3, 5, 8, 1, 4, 6],
            [6, 5, 4, 2, 7, 1, 9, 3, 8],
            [3, 8, 1, 9, 6, 4, 5, 7, 2],
        ]);
        assert!(valid.is_valid());

        let incomplete = unresolved_grid();
        assert!(!incomplete.is_valid());

        // the 5 in 4th row 4th column should be a 4
        let invalid = Grid::from([
            [9, 6, 7, 1, 3, 2, 4, 8, 5],
            [4, 3, 5, 8, 9, 7, 2, 6, 1],
            [8, 1, 2, 6, 4, 5, 3, 9, 7],
            [1, 2, 6, 5, 8, 9, 7, 5, 3],
            [5, 9, 8, 7, 1, 3, 6, 2, 4],
            [7, 4, 3, 5, 2, 6, 8, 1, 9],
            [2, 7, 9, 3, 5, 8, 1, 4, 6],
            [6, 5, 4, 2, 7, 1, 9, 3, 8],
            [3, 8, 1, 9, 6, 4, 5, 7, 2],
        ]);
        assert!(!invalid.is_valid())
    }

    /// Teste que `Grid::solution` renvoie une solution correcte
    #[test]
    fn test_resolution() {
        let unresolved = unresolved_grid();
        assert!(unresolved.solution().is_valid());
    }

    /// Test que le trait FromStr est bien implémenté
    #[test]
    fn test_parse_str() {
        let grid_str = "
9 . . 1 . . . . 5
. . 5 . 9 . 2 . 1
8 . . . 4 . . . 7
. . . . 8 . . . .
. . . 7 . . . . .
. . . . 2 6 . . 9
2 . . 3 . . . . 6
. . . 2 . . 9 . .
. . 1 9 . 4 5 7 .";
        assert!(grid_str.parse::<Grid>().is_ok());
        assert_eq!(grid_str.parse::<Grid>().unwrap(), unresolved_grid());
    }
}
