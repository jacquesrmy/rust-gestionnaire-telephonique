use std::env;
use std::fs;
use std::path::Path;

use rust_gestionnaire_telephonique::models::Contact;
use rust_gestionnaire_telephonique::plantuml::generate_plantuml;
use rust_gestionnaire_telephonique::trie::Trie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -- <input_json_path>")?;

    let output_path = build_output_path(&input_path)?;

    let content = fs::read_to_string(&input_path)?;
    let content = content.replace(",\r\n]", "\r\n]");
    let content = content.replace(",\n]", "\n]");

    let contacts: Vec<Contact> = serde_json::from_str(&content)?;

    let mut trie = Trie::new();

    for contact in &contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    let plantuml = generate_plantuml(&trie);

    fs::write(&output_path, plantuml)?;

    println!("Fichier genere : {}", output_path);

    Ok(())
}

fn build_output_path(input_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(input_path);

    let file_stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or("Impossible de recuperer le nom du fichier d'entree")?;

    Ok(format!("graph/{file_stem}.puml"))
}
