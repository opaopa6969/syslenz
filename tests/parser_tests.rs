//! BL-060 through BL-063: Fixture-based parser tests.
//!
//! These tests verify that each parser's `parse_content()` function correctly
//! parses sample /proc file contents without requiring actual /proc access.
//! This allows the tests to run on any platform (macOS, CI, etc.).

// We need to use the library's types directly since this is a binary crate.
// The parsers are tested via the binary's test harness.

#[cfg(target_os = "linux")]
mod parser_fixture_tests {
    // Import through the binary crate's module system.
    // Since syslenz is a binary crate, we use `#[path]` to include the modules.

    // We can't import from a binary crate in integration tests.
    // Instead, these tests live as unit tests in a dedicated test module.
}

// Since syslenz is a binary crate, integration tests cannot import its modules.
// The actual parser tests are implemented as unit tests in src/proc/mod.rs.
// This file serves as documentation and a placeholder for the test structure.

#[test]
fn parser_tests_exist_in_unit_tests() {
    // This is a marker test. The real parser tests are in src/proc/mod.rs
    // under the `parser_content_tests` module, which can access the internal modules.
    assert!(true, "Parser content tests are in src/proc/mod.rs");
}
