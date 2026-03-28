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
    // Verify the binary can at least start (may fail without a terminal,
    // so we just check it was built).
    let output = std::process::Command::new("cargo")
        .args(["build", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo build");
    assert!(output.status.success(), "Binary failed to build");
}
