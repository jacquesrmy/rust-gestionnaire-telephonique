use std::env;

use rust_gestionnaire_telephonique::config::build_output_path;
use rust_gestionnaire_telephonique::error::AppError;
use rust_gestionnaire_telephonique::help::print_help;
use rust_gestionnaire_telephonique::process::process;

fn main() {
    if let Err(error) = run() {
        eprintln!("Erreur: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let input_path = env::args().nth(1).ok_or(AppError::MissingArgument)?;

    if input_path == "-help" || input_path == "-h" {
        print_help();
        return Ok(());
    }

    let output_path = build_output_path(&input_path)?;

    let plantuml = process(&input_path)?;

    std::fs::write(&output_path, plantuml).map_err(|e| AppError::IoError(e.to_string()))?;

    println!("Fichier genere : {}", output_path);

    Ok(())
}
