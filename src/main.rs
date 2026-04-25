use std::fs;

use rust_gestionnaire_telephonique::models::Contact;
use rust_gestionnaire_telephonique::plantuml::generate_plantuml;
use rust_gestionnaire_telephonique::trie::Trie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "data/01_simple.json";
    let output_path = "graph/01_simple.puml";

    let content = fs::read_to_string(input_path)?;
    let content = content.replace(",\r\n]", "\r\n]");
    let content = content.replace(",\n]", "\n]");

    let contacts: Vec<Contact> = serde_json::from_str(&content)?;

    let mut trie = Trie::new();

    for contact in &contacts {
        trie.insert(&contact.nb);
    }

    let plantuml = generate_plantuml(&trie);

    fs::write(output_path, plantuml)?;

    println!("Fichier genere : {}", output_path);

    Ok(())
}
