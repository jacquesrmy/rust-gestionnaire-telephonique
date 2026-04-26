use crate::error::AppError;
use crate::models::Contact;
use crate::plantuml::generate_plantuml;
use crate::trie::Trie;

pub fn process(input_path: &str) -> Result<String, AppError> {
    let content =
        std::fs::read_to_string(input_path).map_err(|e| AppError::IoError(e.to_string()))?;

    let cleaned_content = clean_json_trailing_commas(&content);

    let contacts: Vec<Contact> =
        serde_json::from_str(&cleaned_content).map_err(|e| AppError::JsonError(e.to_string()))?;

    let mut trie = Trie::new();

    for contact in &contacts {
        trie.insert(&contact.nb, &contact.name);
    }

    Ok(generate_plantuml(&trie))
}

fn clean_json_trailing_commas(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == ',' {
            let mut j = i + 1;

            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }

            if j < chars.len() && (chars[j] == ']' || chars[j] == '}') {
                i += 1;
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple() {
        let result = process("data/01_simple.json").unwrap();

        assert!(result.contains("Alice"));
        assert!(result.contains("@startmindmap"));
    }
}

#[test]
fn test_clean_json_trailing_commas_in_array() {
    let input = "[\n  {\n    \"nb\": \"123\",\n    \"name\": \"Alice\"\n  },\n]";
    let expected = "[\n  {\n    \"nb\": \"123\",\n    \"name\": \"Alice\"\n  }\n]";

    assert_eq!(clean_json_trailing_commas(input), expected);
}

#[test]
fn test_clean_json_trailing_commas_in_object() {
    let input = "{\n  \"a\": 1,\n}";
    let expected = "{\n  \"a\": 1\n}";

    assert_eq!(clean_json_trailing_commas(input), expected);
}
