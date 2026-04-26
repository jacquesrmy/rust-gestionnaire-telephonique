use rust_gestionnaire_telephonique::process::process;

#[test]
fn test_process_generates_plantuml_from_simple_file() {
    let result = process("data/01_simple.json").unwrap();

    assert!(result.contains("@startmindmap"));
    assert!(result.contains("* 0"));
    assert!(result.contains("Alice"));
    assert!(result.contains("@endmindmap"));
}

#[test]
fn test_process_generates_plantuml_for_common_parts() {
    let result = process("data/04_common_parts.json").unwrap();

    assert!(result.contains("Alice"));
    assert!(result.contains("Bob"));
    assert!(result.contains("Urgences"));
    assert!(result.contains("SAMU"));
}

#[test]
fn test_process_fails_for_missing_file() {
    let result = process("data/introuvable.json");

    assert!(result.is_err());
}
