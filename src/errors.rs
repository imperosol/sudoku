use std::error::Error;
use std::fmt::{Display, Formatter};


/// Erreur retournée quand la conversion d'une chaine de caractères
/// en une grille a échoué
#[derive(Debug)]
pub struct ParseGridError;

impl Display for ParseGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Impossible de parser la grille")
    }
}

impl Error for ParseGridError {}
