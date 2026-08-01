//! Integration smoke tests.
//!
//! T17 and T18 (Snapshot::capture and individual parser smoke tests)
//! live in src/proc/mod.rs as unit tests since this is a binary crate.
//! This file provides end-to-end verification that the binary builds
//! and the test suite passes.

#[cfg(target_os = "linux")]
#[test]
fn binary_builds_successfully() {
    let status = std::process::Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("Failed to run cargo build");
    assert!(status.success(), "cargo build failed");
}

#[cfg(target_os = "linux")]
#[test]
fn binary_responds_to_help_or_version() {
    let output = run_binary(&["--help"]);
    assert!(output.status.success(), "--help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("USAGE:"),
        "--help should print usage; got:\n{}",
        stdout
    );
    assert!(
        stdout.contains("--query"),
        "--help should list --query; got:\n{}",
        stdout
    );
}

/// Helper: run the built binary with the given arguments.
#[cfg(target_os = "linux")]
fn run_binary(args: &[&str]) -> std::process::Output {
    // Ensure binary is built first
    let build = std::process::Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("cargo build failed");
    assert!(build.success(), "cargo build failed");

    let bin = format!("{}/target/debug/syslenz", env!("CARGO_MANIFEST_DIR"));
    std::process::Command::new(&bin)
        .args(args)
        .output()
        .expect("Failed to run syslenz binary")
}

#[cfg(target_os = "linux")]
#[test]
fn query_lists_sources() {
    let output = run_binary(&["--query"]);
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("meminfo"), "should list meminfo source");
    assert!(stdout.contains("uptime"), "should list uptime source");
}

#[cfg(target_os = "linux")]
#[test]
fn query_source_fields_tsv() {
    let output = run_binary(&["--query", "meminfo"]);
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("MemTotal\t"),
        "should contain MemTotal field in TSV"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn query_specific_field() {
    let output = run_binary(&["--query", "meminfo.MemTotal"]);
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // MemTotal should produce some non-empty value
    assert!(!stdout.trim().is_empty(), "should print a value");
}

#[cfg(target_os = "linux")]
#[test]
fn query_specific_field_json() {
    let output = run_binary(&["--query", "meminfo.MemTotal", "--json"]);
    assert!(output.status.success(), "exit code should be 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    // Verify it looks like valid JSON with expected keys
    assert!(
        trimmed.starts_with('{') && trimmed.ends_with('}'),
        "should be JSON object"
    );
    assert!(
        trimmed.contains("\"source\":\"meminfo\""),
        "should contain source field"
    );
    assert!(
        trimmed.contains("\"field\":\"MemTotal\""),
        "should contain field name"
    );
    assert!(trimmed.contains("\"value\""), "should contain value field");
}

#[cfg(target_os = "linux")]
#[test]
fn query_nonexistent_source() {
    let output = run_binary(&["--query", "nonexist"]);
    assert!(!output.status.success(), "exit code should be 1");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Source 'nonexist' not found"),
        "should print error to stderr"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn query_nonexistent_field() {
    let output = run_binary(&["--query", "meminfo.NoSuchField"]);
    assert!(!output.status.success(), "exit code should be 1");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Field 'NoSuchField' not found in 'meminfo'"),
        "should print field error to stderr"
    );
}
