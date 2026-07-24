use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn missing_input_fails() {
    let mut cmd = Command::cargo_bin("cambiar").unwrap();

    cmd.arg("definitely-does-not-exist.md")
        .arg("output.txt")
        .assert()
        .failure()
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn unsupported_conversion_fails() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.md");
    let output = dir.path().join("output.csv");

    std::fs::write(&input, "# Hello").unwrap();

    let mut cmd = Command::cargo_bin("cambiar").unwrap();

    cmd.arg(&input)
        .arg(&output)
        .assert()
        .failure()
        .stdout(predicate::str::contains("Unsupported"));
}

#[test]
fn refuses_to_overwrite_without_force() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.md");
    let output = dir.path().join("output.txt");

    std::fs::write(&input, "# New content").unwrap();
    std::fs::write(&output, "IMPORTANT OLD CONTENT").unwrap();

    let mut cmd = Command::cargo_bin("cambiar").unwrap();

    cmd.arg(&input).arg(&output).assert().failure();

    let contents = std::fs::read_to_string(&output).unwrap();

    assert_eq!(contents, "IMPORTANT OLD CONTENT");
}

#[test]
fn list_formats_succeeds() {
    let mut cmd = Command::cargo_bin("cambiar").unwrap();

    cmd.arg("--list-formats")
        .assert()
        .success()
        .stdout(predicate::str::contains("csv"))
        .stdout(predicate::str::contains("json"))
        .stdout(predicate::str::contains("md"))
        .stdout(predicate::str::contains("txt"));
}

#[test]
fn force_overwrites_existing_output() {
    let dir = tempfile::tempdir().unwrap();

    let input = dir.path().join("input.md");
    let output = dir.path().join("output.txt");

    std::fs::write(&input, "# New content").unwrap();
    std::fs::write(&output, "OLD CONTENT").unwrap();

    let mut cmd = Command::cargo_bin("cambiar").unwrap();

    cmd.arg(&input)
        .arg(&output)
        .arg("--force")
        .assert()
        .success();

    let contents = std::fs::read_to_string(&output).unwrap();

    assert!(contents.contains("New content"));
    assert!(!contents.contains("OLD CONTENT"));
}
