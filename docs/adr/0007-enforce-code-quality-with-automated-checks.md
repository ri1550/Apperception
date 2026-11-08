---
status: accepted
date: 2026-11-08
decision-makers: Owen Robert McGregor
consulted: []
informed: []
affects:
  components: ["*"]
  bases: ["*"]
  projects: ["*"]
interface-impact: none
---

# ADR-0007: Enforce code quality with automated checks
## Context and problem statement
The codebase needs to stay architecturally sound, warning-free and tested as it grows. Relying on code review alone to catch broken brick boundaries, failing tests, lint warnings or a stale lockfile does not scale and is exactly what fails under time pressure. We need code quality to be enforced at the gate.

## Decision drivers
- Brick boundaries and the base-to-component rule must be structurally enforced, not just documented.
- Behavioral guarantees, including doc examples, must be verified on every change, not assumed.
- Warnings must not accumulate: a warning that is allowed to stand becomes a hundred.
- Known-vulnerable dependencies should be caught before they reach `main`.
- Best Simple System for Now: use the tools the Rust toolchain already ships, wire them into the same two-layer gate as ADR-0006, add nothing speculative.

## Considered options
- Rely on code review alone
- Run checks locally by convention only
- Local hooks only; CI only; both layers (local hooks plus CI gated by branch protection)
- Clippy's default lint set with warnings as errors; clippy pedantic
- `cargo audit` for advisories; `cargo deny` for advisories, licenses and bans; no dependency scanning

## Decision outcome
Chosen option: both layers, following the same approach as ADR-0006, using the toolchain's own tools.

The checks are `cargo fmt --check` (one formatting, no debate), `cargo poly check` (brick boundaries, ADR-0004), `cargo clippy --workspace --all-targets -- -D warnings` (every rustc and clippy warning is an error, on tests and examples too), `cargo test --workspace` (unit tests, interface tests and doc examples), `cargo build --locked` in CI (the committed `Cargo.lock` is current), and `cargo audit` in CI through the RustSec action (no dependency with a known advisory). The workspace sets `unsafe_code = "deny"`; a brick that needs `unsafe` opts out explicitly and visibly. Rust is statically typed, so the type check is the build itself and needs no separate tool.

The hook runs the fast checks: formatting, the Polylith check, the lints, clippy and the interface heads-up. Tests and the audit run in CI, as in the Python template, so a commit stays quick while the gate stays complete.

Clippy's default set over pedantic because pedantic is opinionated enough to generate churn without a concrete need; it can be enabled per crate or workspace-wide when one appears. `cargo audit` over `cargo deny` because advisories are the one check with a clear need now; license and duplicate-dependency policy can be added when a project has one. Over no scanning because govulncheck-style advisory checks are cheap in CI and expensive to discover late.

### Consequences
- Good: architectural violations, failing tests, stale usage examples, warnings and known-vulnerable dependencies are all caught before code reaches `main`.
- Good: editor and gate use the same tools (rustfmt, clippy), so inline feedback matches what CI blocks.
- Bad: `-D warnings` means a new clippy lint in a toolchain update can fail CI until the code is adjusted or the lint allowed with a reason.
- Neutral: tests do not run in the hook. Run `cargo test --workspace` before pushing; CI runs it regardless.

### Confirmation
`cargo fmt --check`, `cargo poly check` and `cargo clippy -D warnings` run in the pre-commit hook on every commit. All of those plus `cargo build --locked`, `cargo test --workspace` and `cargo audit` run in CI as part of the `checks` workflow required by branch protection on `main`.

## More information
- ADR-0002 and ADR-0004 (the architecture `cargo poly check` enforces)
- ADR-0006 (context system checks that use the same two-layer approach)
- `CONTRIBUTING.md` (setup and day-to-day workflow)
- `tools/poly/README.md` (what each check does)
- Clippy: https://doc.rust-lang.org/clippy/
- RustSec advisory database and `cargo audit`: https://rustsec.org/
- Best Simple System for Now (BSSN): https://dannorth.net/blog/best-simple-system-for-now/
