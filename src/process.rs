use crate::error::AppError;
use crate::models::Contact;
use crate::plantuml::generate_plantuml;
use crate::trie::Trie;

pub fn process(input_path: &str) -> Result<String, AppError> {
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
