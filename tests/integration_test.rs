use rust_gestionnaire_telephonique::process::process;
use std::process::Command;

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

#[test]
fn test_program_fails_without_argument() {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Argument manquant"));
}

#[test]
fn test_program_fails_with_missing_file() {
    let output = Command::new("cargo")
        .args(["run", "--", "data/introuvable.json"])
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Erreur"));
}
