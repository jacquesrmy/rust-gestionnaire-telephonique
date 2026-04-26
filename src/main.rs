use std::env;

use rust_gestionnaire_telephonique::config::build_output_path;
use rust_gestionnaire_telephonique::error::AppError;
use rust_gestionnaire_telephonique::models::Contact;
use rust_gestionnaire_telephonique::plantuml::generate_plantuml;
use rust_gestionnaire_telephonique::trie::Trie;

fn main() {
    if let Err(error) = run() {
        eprintln!("Erreur: {}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let input_path = env::args().nth(1).ok_or(AppError::MissingArgument)?;

    let output_path = build_output_path(&input_path)?;

    let plantuml = process(&input_path)?;

    std::fs::write(&output_path, plantuml).map_err(|e| AppError::IoError(e.to_string()))?;

    println!("Fichier genere : {}", output_path);

    Ok(())
}

fn process(input_path: &str) -> Result<String, AppError> {
    let content =
        std::fs::read_to_string(input_path).map_err(|e| AppError::IoError(e.to_string()))?;

    let content = content.replace(",\r\n]", "\r\n]");
    let content = content.replace(",\n]", "\n]");

    let contacts: Vec<Contact> =
        serde_json::from_str(&content).map_err(|e| AppError::JsonError(e.to_string()))?;

    let mut trie = Trie::new();

    for contact in &contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    Ok(generate_plantuml(&trie))
}
