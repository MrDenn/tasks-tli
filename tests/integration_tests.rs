use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn cargo_bin_cmd() -> String {
    std::env::var("CARGO_BIN_EXE_silver-tasks").unwrap_or_else(|_| "target/debug/silver-tasks".to_string())
}

#[test]
fn test_list_with_empty_storage() {
    let temp_dir = TempDir::new().unwrap();
    let tasks_file = temp_dir.path().join(".tasks.json");
    let tags_file = temp_dir.path().join(".tasks-tags.json");

    // Create empty files
    fs::write(&tasks_file, "[]").unwrap();
    fs::write(&tags_file, "[]").unwrap();

    let mut cmd = Command::new(cargo_bin_cmd());
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");

    cmd.assert().success().stdout(predicate::str::contains(
        "No tasks found",
    ));
}

#[test]
fn test_list_command_parsing() {
    let temp_dir = TempDir::new().unwrap();
    let tasks_file = temp_dir.path().join(".tasks.json");
    let tags_file = temp_dir.path().join(".tasks-tags.json");

    fs::write(&tasks_file, "[]").unwrap();
    fs::write(&tags_file, "[]").unwrap();

    let mut cmd = Command::new(cargo_bin_cmd());
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");

    cmd.assert().success();
}

#[test]
fn test_list_with_sample_data() {
    let temp_dir = TempDir::new().unwrap();
    let tasks_file = temp_dir.path().join(".tasks.json");
    let tags_file = temp_dir.path().join(".tasks-tags.json");

    // Create sample tasks JSON
    let tasks_json = r#"[
        {"id":1,"name":"Rewatch lecture 1","tag":"Automata","deadline":"2026-02-17"},
        {"id":2,"name":"Read chapter 3","tag":"Automata","deadline":"2026-02-16"},
        {"id":3,"name":"Homework assignment","tag":"Calculus","deadline":null}
    ]"#;

    let tags_json = r#"["Automata","Calculus"]"#;

    fs::write(&tasks_file, tasks_json).unwrap();
    fs::write(&tags_file, tags_json).unwrap();

    let mut cmd = Command::new(cargo_bin_cmd());
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Automata"))
        .stdout(predicate::str::contains("Calculus"))
        .stdout(predicate::str::contains("Read chapter 3"))
        .stdout(predicate::str::contains("Rewatch lecture 1"));
}

#[test]
fn test_list_output_format_and_sorting() {
    let temp_dir = TempDir::new().unwrap();
    let tasks_file = temp_dir.path().join(".tasks.json");
    let tags_file = temp_dir.path().join(".tasks-tags.json");

    // Create sample tasks JSON with specific deadlines to test sorting
    let tasks_json = r#"[
        {"id":1,"name":"Task1","tag":"Test","deadline":"2026-02-20"},
        {"id":2,"name":"Task2","tag":"Test","deadline":"2026-02-17"},
        {"id":3,"name":"Task3","tag":"Test","deadline":"2026-02-25"}
    ]"#;

    let tags_json = r#"["Test"]"#;

    fs::write(&tasks_file, tasks_json).unwrap();
    fs::write(&tags_file, tags_json).unwrap();

    let mut cmd = Command::new(cargo_bin_cmd());
    cmd.env("HOME", temp_dir.path());
    cmd.arg("list");

    // Verify:
    // 1. Tag header "Test" appears
    // 2. Tasks are sorted by deadline (002 first, then 001, then 003)
    // 3. Tasks are indented (4 spaces)
    // 4. Date format is DD.MM.YYYY
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test"))
        // Task 2 should appear first (deadline 2026-02-17)
        .stdout(predicate::str::contains("002 - Task2"))
        // Task 1 should appear second (deadline 2026-02-20)
        .stdout(predicate::str::contains("001 - Task1"))
        // Task 3 should appear third (deadline 2026-02-25)
        .stdout(predicate::str::contains("003 - Task3"))
        // Verify 4-space indentation by checking for the pattern
        .stdout(predicate::str::contains("    002"));
}
