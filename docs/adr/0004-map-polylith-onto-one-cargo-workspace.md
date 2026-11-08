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

# ADR-0004: Map Polylith onto one Cargo workspace
## Context and problem statement
Polylith has no Rust tooling, and Cargo has opinions: a crate is the unit of compilation and of visibility, a binary needs a `main.rs`, and a workspace shares one lockfile and one `target/` directory. We need to decide how bricks, projects, the development environment and the namespace map onto Cargo so that the rest of the template, and every contributor, can assume one shape.

## Decision drivers
- Brick boundaries should be enforced by the compiler wherever Cargo can do it, and by `cargo poly check` only where it cannot.
- One build, one test run and one lint for the whole codebase.
- Projects must hold no business logic, yet a Rust binary needs a `main` somewhere.
- Brick names must not collide with crates.io names.
- Scratch work needs a home that builds against the bricks without becoming a deliverable.
- Best Simple System for Now: the least structure that gives all of the above now.

## Considered options
- Bricks: one crate per brick in a single workspace; modules of one crate; one workspace per project
- Projects: a binary crate whose `main.rs` is the composition root; `[[bin]]` targets inside the base crate; feature flags on a base
- Crate names: prefixed with a namespace (`apperception-greeting`); bare brick names (`greeting`)
- Scratch: a `development` crate that is a workspace member; a directory excluded from the workspace; nothing
- Tests: interface tests in each brick's `tests/`; a root `test/` tree mirroring the bricks as in the Python template

## Decision outcome
Chosen option: one Cargo workspace at the root, every brick a crate, every project a binary crate that is only a composition root.

- The root `Cargo.toml` is a virtual manifest whose members are `components/*`, `bases/*`, `projects/*`, `development` and `tools/poly`. `Cargo.lock` is committed. A `.keep` file holds each glob directory in git when it is empty, because Cargo fails on a glob whose directory is missing.
- Each component and base is a library crate named `<namespace>-<directory>`, with the namespace in `[workspace.metadata.polylith]`. Every brick is declared once in `[workspace.dependencies]` and consumed as `apperception-<brick>.workspace = true`, so no consumer writes a relative path.
- A brick's `src/lib.rs` is its public interface: private `mod` declarations and an explicit `pub use` list. No `pub mod`, no `pub use x::*`. Then the interface really is one file, the interface-change gate (ADR-0006) can watch one path, and the compiler keeps everything else private.
- A project is a binary crate and the composition root: its `src/` holds only `main.rs`, which depends on exactly one base (plus any components the project adds) and holds no logic. A base is a library crate so it can be tested and reused across projects; the project is where it becomes a binary.
- `development/` is a scratch binary crate and a workspace member, so it builds against the bricks and runs with `cargo run -p development`. Nothing in it is a deliverable; it is reset before committing.
- Tests that pin behavior are integration tests in each brick's `tests/` directory. They see only what `lib.rs` exports, so they test at the interface by construction. Unit tests inside `src/` remain available for internals and cite no spec rule.

Over modules of one crate because modules give no compiler-enforced boundary and one compilation unit; over one workspace per project because that duplicates lockfiles and loses the single build. Over `[[bin]]` targets in the base because that blurs base and project and ties a base to one deployment; over feature flags because they are invisible in the layout. The namespace prefix exists because a component named `log` or `http` would otherwise collide with a crates.io dependency of the same name. A `development` directory excluded from the workspace would need its own lockfile and `target/`, which is more structure, not less. A root `test/` tree is not idiomatic Cargo and would need a crate per brick to compile at all.

### Consequences
- Good: visibility and dependency direction are enforced by the compiler and Cargo; `cargo poly check` only adds the Polylith-specific rules (naming, location, lib.rs shape, composition root, base to component direction).
- Good: one `cargo build`, `cargo test` and `cargo clippy` cover everything, including doc examples.
- Bad: adding a brick is three edits (the crate, its `[workspace.dependencies]` line, the consumer's dependency) with no scaffolding command.
- Neutral: the `development` crate compiles and lints with everything else, so broken scratch fails the hook. That is the pre-commit checklist's point.

### Confirmation
`cargo poly check` fails the hook and CI when a member is outside the five locations, a brick is misnamed or has a binary target, a brick's `lib.rs` exposes a module or glob, a component depends on a base, a base depends on a base, a project has anything but `main.rs` under `src/` or depends on other than exactly one base, or anything depends on a project, the development crate or a tool. Cargo itself fails on a directory under a glob that is not a crate.

## More information
- ADR-0002 (the architecture this maps)
- ADR-0006 (the interface-change gate that watches `src/lib.rs`)
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Best Simple System for Now (BSSN): https://dannorth.net/blog/best-simple-system-for-now/
