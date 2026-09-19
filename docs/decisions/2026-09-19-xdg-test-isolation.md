# XDG test isolation

## Observation

The 2026-09-19 `main` CI run failed because parallel pin-file tests changed the
process-wide `XDG_CONFIG_HOME` variable independently. A roundtrip test then
read the invalid TOML fixture owned by another test and returned zero pins.

Evidence: [GitHub Actions run 35435409749](https://github.com/opaopa6969/syslenz/actions/runs/35435409749),
retrieved 2026-09-19 under the GitHub Terms of Service. The repository owns the
workflow and log content; no third-party source material was copied.

Local reproduction from the repository root:

```bash
for run_id in $(seq 1 30); do
  cargo test pinfile_ -- --test-threads=16 || break
done
```

## Decision

All tests that change process environment use one test-only RAII guard. The
guard serializes mutations and restores the prior value during normal return or
unwinding. Runtime configuration and pin persistence are unchanged.

This is narrower than serializing the whole test suite and safer than relying
on test names or timing. Revert this change if the tests move to explicit path
injection and no longer mutate process environment.

## Acceptance evidence

- 30 consecutive parallel pin-file test runs pass.
- The all-feature Rust test, format check, and CI-matrix lint checks pass.
- CI passes on the pull request.
