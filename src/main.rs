use crate::command::SudokuCli;
use crate::grid::Grid;
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

mod command;
mod errors;
mod grid;

fn main() {
    let cli = SudokuCli::parse();
    let Ok(mut file) = File::open(&cli.filename) else {
        eprintln!("File {} doesn't exist", cli.filename.to_str().unwrap());
        exit(1);
    };
    let grid = {
        // lecture du fichier et conversion du contenu en une struct Grid
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Couldn't read file");
        content.parse::<Grid>().unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(1);
        })
    };
    let solution = grid.solution();

    // Si un fichier de sortie a été donné, on écrit la grille dans ce fichier,
    // sinon on l'affiche sur la sortie standard.
    match cli.output {
        None => println!("{}", solution),
        Some(filename) => {
            File::create(&filename)
                .unwrap()
                .write_all(solution.to_string().as_bytes())
                .expect("Couldn't save the result file");
            println!("Fichier {} enregistré", &filename.as_str());
        }
    };
}
